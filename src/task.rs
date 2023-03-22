pub struct List {
    tasks: Vec<Task>,
    name: String,
}

impl List {
    pub fn new<T: ToString>(name: T) -> Self {
        let name = name.to_string();
        Self {
            tasks: vec![],
            name,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn tasks_iter(&self) -> std::slice::Iter<Task> {
        self.tasks.iter()
    }

    pub fn add_task<T: ToString>(&mut self, description: T) {
        self.tasks.push(Task::new(description));
    }
}

pub struct Task {
    description: String,
    completed: bool,
}

impl Task {
    pub fn new<T: ToString>(description: T) -> Task {
        let description = description.to_string();

        Self {
            description,
            completed: false,
        }
    }

    pub fn toggle_status(&mut self) {
        self.completed = !self.completed;
    }

}

impl ToString for Task {
    fn to_string(&self) -> String {
        let mut string = String::new();

        string.push_str(match self.completed {
            true => "[x] ",
            false => "[ ] ",
        });

        string.push_str(&self.description);

        string
    }
}
