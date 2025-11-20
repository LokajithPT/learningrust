pub struct App {
    pub input: String,
    pub output: String,
}

impl App {
    pub fn new() -> App {
        App {
            input: String::new(),
            output: String::new(),
        }
    }
}
