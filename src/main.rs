mod app;

use app::{TasksApp, serialize, deserialise};

fn main() {
    let lists = deserialise().expect("Failed to open file");

    let mut app = TasksApp::new(lists.clone());

    match app.run() {
        Ok(lists) => serialize(lists).unwrap(),
        Err(e) => eprintln!("Error: {}", e)
    };
}
