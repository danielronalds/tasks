mod task;
mod app;
mod serialisation;

use app::App;
use task::List;

fn main() {
    let mut lists = vec![ List::new("Main"), List::new("Second"), List::new("Third")];
    lists[0].add_task("this is a task");
    lists[0].add_task("this is another task");

    lists[1].add_task("this is task on the second list");

    lists[2].add_task("this is task on the third list");
    lists[2].add_task("this another task on the third list");
    lists[2].add_task("Wow look another task on the third list");
    lists[2].add_task("...yet another task on the third list");

    let mut app = App::new(lists.clone());

    match app.run() {
        Ok(lists) => serialisation::serialize(lists).unwrap(),
        Err(e) => eprintln!("Error: {}", e)
    };
}
