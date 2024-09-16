#[derive(Debug)]
pub enum StatusTask {
    Pendente,
    Concluido,
}

#[derive(Debug)]
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
        // [INFO] Add task
        self.log();
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn log(&self) {
        println!("id: {}", self.id);
    }
}
