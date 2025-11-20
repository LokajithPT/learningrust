use crossterm::event::{KeyCode, Event as CrosstermEvent};
use futures::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

#[derive(Debug)]
pub enum Event {
    Tick,
    Key(KeyCode),
    ServerMessage(String), 

}

pub struct EventHandler {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
}

impl EventHandler {
    pub fn sender(&self) -> mpsc::Sender<Event> {
        self.sender.clone()
    }


    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel(256);
        let mut event_stream = crossterm::event::EventStream::new();
        let mut tick_interval = time::interval(tick_rate);

        let task_sender = sender.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = tick_interval.tick() => {
                        if task_sender.send(Event::Tick).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(CrosstermEvent::Key(key))) = event_stream.next() => {
                        if task_sender.send(Event::Key(key.code)).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        Self { sender, receiver }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.receiver.recv().await
    }
}
