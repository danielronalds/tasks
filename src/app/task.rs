use colored::Colorize;

#[derive(Clone, Debug)]
pub struct List {
    tasks: Vec<Task>,
    name: String,
}

impl List {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            tasks: vec![],
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn tasks_iter(&self) -> std::slice::Iter<Task> {
        self.tasks.iter()
    }

    pub fn toggle_task(&mut self, index: usize) {
        if index >= self.length() {
            return;
        }

        self.tasks[index].toggle_status();
    }

    pub fn length(&self) -> usize {
        self.tasks.len()
    }

    pub fn add_task<T: ToString>(&mut self, description: T) {
        if description.to_string().is_empty() {
            return;
        }

        self.tasks.push(Task::new(description));
    }

    pub fn delete_task(&mut self, index: usize) {
        if index >= self.length() {
            return;
        }

        self.tasks.remove(index);
    }

    pub fn delete_completed_tasks(&mut self) {
        self.tasks = self
            .tasks
            .iter()
            .filter(|task| !task.status())
            .map(|task| task.to_owned())
            .collect();
    }

    pub fn sort_list(&mut self) {
        self.tasks = self
            .tasks
            .iter()
            .filter(|task| task.status())
            .chain(self.tasks.iter().filter(|task| !task.status()))
            .map(|task| task.to_owned())
            .collect();
    }
}

#[derive(Clone, Debug)]
pub struct Task {
    description: String,
    completed: bool,
}

impl Task {
    pub fn new<T: ToString>(description: T) -> Task {
        Self {
            description: description.to_string(),
            completed: false,
        }
    }

    pub fn toggle_status(&mut self) {
        self.completed = !self.completed;
    }

    pub fn status(&self) -> bool {
        self.completed
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}

impl ToString for Task {
    fn to_string(&self) -> String {
        let mut string = String::new();

        string.push_str(&match self.completed {
            true => format!("[{}] ", "✔".bright_green()),
            false => "[ ] ".to_string(),
        });

        string.push_str(&self.description);

        string
    }
}
