mod app;

use app::{deserialise, new_tasks_data, serialise, TasksApp};

use crossterm::event::{read, Event, KeyCode};

const DEFAULT_LIST_NAME: &str = "Main";

fn main() {
    let lists = match deserialise() {
        Ok(lists) => lists,
        Err(_) => {
            println!("Couldn't find a .tasks.md file in this directory, create one? (Y/n)");
            crossterm::terminal::enable_raw_mode().expect("Failed to enable raw terminal mode");
            match read().expect("failed to read") {
                Event::Key(key) => match key.code {
                    KeyCode::Char('n') | KeyCode::Char('N') => return,
                    _ => new_tasks_data(DEFAULT_LIST_NAME),
                },
                _ => new_tasks_data(DEFAULT_LIST_NAME),
            }
        }
    };
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw terminal mode");

    let mut app = TasksApp::new(lists);

    match app.run() {
        Ok(lists) => {
            if let Some(lists) = lists {
                serialise(lists).expect("Couldn't serialize")
            }
        }

        Err(e) => eprintln!("Error: {}", e),
    };
}
