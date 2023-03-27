mod app;

use app::{deserialise, serialize, TasksApp};

fn main() {
    let lists = deserialise().expect("Failed to open file");

    let mut app = TasksApp::new(lists);

    match app.run() {
        Ok(lists) => serialize(lists).unwrap(),
        Err(e) => eprintln!("Error: {}", e),
    };
}
