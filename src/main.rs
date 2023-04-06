mod app;

use app::{deserialise, new_tasks_data, serialise, TasksApp};

use crossterm::event::{read, Event, KeyCode};

fn main() {
    let lists = match deserialise() {
        Ok(lists) => lists,
        Err(_) => {
            println!("Coudln't find a .tasks.md file in this directory, create one? (Y/n)");
            crossterm::terminal::enable_raw_mode().expect("Failed to enable raw terminal mode");
            match read().expect("failed to read") {
                Event::Key(key) => match key.code {
                    KeyCode::Char('n') | KeyCode::Char('N') => return,
                    _ => new_tasks_data("Main"),
                },
                _ => new_tasks_data("Main"),
            }
        }
    };
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw terminal mode");

    let mut app = TasksApp::new(lists);

    match app.run() {
        Ok(lists) => serialise(lists).expect("Couldn't serialize"),
        Err(e) => eprintln!("Error: {}", e),
    };
}
