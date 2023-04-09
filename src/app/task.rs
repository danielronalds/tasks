use colored::Colorize;

#[derive(Clone, Debug)]
/// A struct to group related tasks under a name
pub struct List {
    tasks: Vec<Task>,
    name: String,
}

impl List {
    /// Creates a new List
    ///
    /// # Arguments
    ///
    /// * `name` - The name the list should have
    ///
    /// # Returns
    /// An error if `name` is empty, Otherwise an Ok() containing a new List
    pub fn new<T: ToString>(name: T) -> Result<Self, ()> {
        let name = name.to_string();

        if name.is_empty() {
            return Err(());
        }

        Ok(Self {
            tasks: vec![],
            name,
        })
    }

    /// Returns an owned copy of the list name
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns an iterator of the tasks contained in the list
    pub fn tasks_iter(&self) -> std::slice::Iter<Task> {
        self.tasks.iter()
    }

    /// Toggles the status of the task at the given index. No error is generated if the task index
    /// is out of bounds, the method just returns early.
    ///
    /// # Arguments
    /// * `index` - The index of the task to toggle
    pub fn toggle_task(&mut self, index: usize) {
        if index >= self.length() {
            return;
        }

        self.tasks[index].toggle_status();
    }

    /// Returns the number of tasks in the list
    pub fn length(&self) -> usize {
        self.tasks.len()
    }

    /// Renames the name of the list. Method returns early if `new_name` is empty
    ///
    /// # Arguments
    /// * `new_name` - The new name of the list
    pub fn rename_list<T: ToString>(&mut self, new_name: T) {
        let new_name = new_name.to_string();

        if new_name.is_empty() {
            return;
        }

        self.name = new_name;
    }

    /// Creates and adds a new task to the list. The task will not be completed by default, and the
    /// method returns early if the description is empty
    ///
    /// # Arguments
    /// * `description` - The description of the task to add
    pub fn add_task<T: ToString>(&mut self, description: T) {
        if description.to_string().is_empty() {
            return;
        }

        self.tasks.push(Task::new(description));
    }

    /// Inserts the given `Task` at the given index. If the index given is 1 greater than the
    /// current length of the task vec, then it is inserted at 0. Otherwise the method returns
    /// early to avoid a panic.
    ///
    /// # Arguments
    /// * `index` - The index to insert the task at
    /// * `task`  - The task to insert
    pub fn insert_task(&mut self, index: usize, task: Task) {
        if index > self.tasks.len() {
            if index - self.tasks.len() == 1 {
                self.tasks.insert(index - 1, task);
            }
            return;
        }
        self.tasks.insert(index, task);
    }

    /// Changes the description of the task at the given index
    ///
    /// # Arguments
    /// * `index`        - The index of the task to change the description of
    /// * `description`  - The new description of the task
    pub fn reword_task<T: ToString>(&mut self, index: usize, description: T) {
        if index >= self.length() || description.to_string().is_empty() {
            return;
        }

        self.tasks[index].description = description.to_string();
    }

    /// Deletes the task at the given index from the list. If the index is out of bounds then the
    /// method returns early to avoid a panic.
    ///
    /// # Arguments
    /// * `index` - The index of the task to delete
    pub fn delete_task(&mut self, index: usize) {
        if index >= self.length() {
            return;
        }

        self.tasks.remove(index);
    }

    /// Removes completed tasks from the list
    pub fn delete_completed_tasks(&mut self) {
        self.tasks = self
            .tasks
            .iter()
            .filter(|task| !task.status())
            .map(|task| task.to_owned())
            .collect();
    }

    /// Removes all tasks from the list
    pub fn delete_all_tasks(&mut self) {
        self.tasks = vec![];
    }

    /// Sorts the list with completed tasks being first, followed by the rest of the tasks
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
/// Struct to represent a task
pub struct Task {
    description: String,
    completed: bool,
}

impl Task {
    /// Creates a new task
    ///
    /// # Arguments
    /// * `description` - The description the task should have
    pub fn new<T: ToString>(description: T) -> Task {
        Self {
            description: description.to_string(),
            completed: false,
        }
    }

    /// Toggles whether the task is completed or not
    pub fn toggle_status(&mut self) {
        self.completed = !self.completed;
    }

    /// Returns the tasks status
    pub fn status(&self) -> bool {
        self.completed
    }

    /// Returns the tasks description
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
