mod app;

use app::{deserialise, new_tasks_data, serialise, TasksApp};

fn main() {
    let lists = match deserialise() {
        Ok(lists) => lists,
        Err(_) => {
            println!("Coudln't find a .tasks.md file in this directory, Provide a name for the first list:");
            let mut list_name = String::new();
            std::io::stdin()
                .read_line(&mut list_name)
                .expect("Failed to read line");
            new_tasks_data(list_name.trim())
        }
    };

    let mut app = TasksApp::new(lists);

    match app.run() {
        Ok(lists) => serialise(lists).expect("Couldn't serialize"),
        Err(e) => eprintln!("Error: {}", e),
    };
}
