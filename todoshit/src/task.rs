pub struct Task {
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            description,
            completed: false,
        }
    }
}
