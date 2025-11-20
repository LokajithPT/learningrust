use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};


use serde::Deserialize;
use std::{collections::HashMap, error::Error, io};

struct App {
    //... 
    pub coin_id: String, 
    pub currency: String, 
    pub price: Option<f64>, 

}

impl App  {
    // idk we define shit here '
    fn new() -> App {
        App{
            coin_id: "bitcoin".to_string(), 
            currency: "inr".to_string(), 
            price: None, 
        }
    }
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    #[serde(flatten)]
    coins: HashMap<String, Coin>,
}

#[derive(Deserialize, Debug)]
struct Coin {
    #[serde(flatten)]
    prices: HashMap<String, f64>,
}



fn main() -> Result<(), Box<dyn Error>> {
    // 1. Set up the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Run the main loop
    let res = run_app(&mut terminal);

    // 3. Restore the terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();
    if let Ok(price) = fetch_price(&app){
        app.price = Some(price);
    }
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    // This is where we will draw our UI
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let block = Block::default().title("Crypto TUI").borders(Borders::ALL);
    let text = match app.price {
        Some(price) => format!(
            "Price of {}: {} {}", 
            app.coin_id, price , app.currency
        ), 
        None => format!("fetching price for {}...", app.coin_id), 
    };

    let paragraph = Paragraph::new(text).block(block);

    f.render_widget(paragraph, chunks[0]);
}

fn fetch_price(app: &App ) -> Result<f64 , Box<dyn Error>>{
    
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        app.coin_id , app.currency
    );

    let api_response: ApiResponse = reqwest::blocking::get(&url)?.json()?;

    let coin_data = api_response
        .coins
        .get(&app.coin_id)
        .ok_or(format!("Coin '{}' not found in response ", 
                app.coin_id))?;

    let price = coin_data
        .prices
        .get(&app.currency)
        .ok_or(format!("Currency '{}' not found for coin ", 
                app.currency))?;


    Ok(*price )

}







