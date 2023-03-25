mod task;
mod app;
mod serialisation;

use app::App;

fn main() {
    let lists = serialisation::deserialise().expect("Failed to open file");

    let mut app = App::new(lists.clone());

    match app.run() {
        Ok(lists) => serialisation::serialize(lists).unwrap(),
        Err(e) => eprintln!("Error: {}", e)
    };
}
