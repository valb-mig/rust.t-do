pub enum StatusTask {
    Pendente,
    Concluido,
}

pub struct Task {
    id: u32,
    name: String,
    description: Option<String>,
    status: StatusTask,
}

impl Task {
    pub fn new(name: String, description: Option<String>, status: StatusTask) -> Self {
        Self {
            id: 0,
            name,
            description,
            status,
        }
    }

    pub fn add(&self) {
        self.log();
    }

    fn log(&self) {
        println!("id: {}", self.id);
    }
}
