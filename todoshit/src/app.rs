mod task;
pub use task::Task;

pub struct App {
    pub tasks: Vec<Task>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            should_quit: false,
        }
    }
}
