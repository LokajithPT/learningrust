use reqwest::Client;
use serde::{Deserialize, Serialize};
use futures_util::StreamExt;
use tokio::io::{stdout, AsyncWriteExt};
use tokio::signal;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatChunk {
    message: Option<Message>,
    done: Option<bool>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();
    let model = "gemma3:4b";

    let mut history: Vec<Message> = Vec::new();

    println!("🔥 ollama chat repl running with model `{}`", model);
    println!("type something below\n");

    loop {
        // -------- USER INPUT ----------
        print!("you> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if input == "exit" || input == "quit" {
            break;
        }

        // push user message into history
        history.push(Message {
            role: "user".into(),
            content: input.clone(),
        });

        // build payload
        let payload = serde_json::json!({
            "model": model,
            "messages": history,
            "stream": true
        });

        // send request
        let mut res = client
            .post("http://localhost:11434/api/chat")
            .json(&payload)
            .send()
            .await?;

        let mut stream = res.bytes_stream();
        let mut out = stdout();

        print!("bot> ");
        out.flush().await?;

        let mut full_response = String::new();

        // -------- STREAM RESPONSE WITH CTRL-C CANCEL ----------
        loop {
            tokio::select! {
                // ctrl-c cancels generation
                _ = signal::ctrl_c() => {
                    println!("\n⛔ generation cancelled");
                    break;
                }

                next = stream.next() => {
                    match next {
                        Some(Ok(chunk)) => {
                            let text = String::from_utf8_lossy(&chunk);

                            for line in text.lines() {
                                if line.trim().is_empty() {
                                    continue;
                                }

                                let parsed: ChatChunk = match serde_json::from_str(line) {
                                    Ok(v) => v,
                                    Err(_) => continue,
                                };

                                if let Some(msg) = parsed.message {
                                    out.write_all(msg.content.as_bytes()).await?;
                                    out.flush().await?;
                                    full_response.push_str(&msg.content);
                                }

                                if parsed.done.unwrap_or(false) {
                                    out.write_all(b"\n").await?;
                                    break;
                                }
                            }
                        }

                        // stream ended normally
                        _ => break,
                    }
                }
            }
        }

        // store assistant response (even if cut mid-way)
        history.push(Message {
            role: "assistant".into(),
            content: full_response,
        });
    }

    Ok(())
}

