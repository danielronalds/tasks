mod task;
mod app;

use app::App;
use task::Task;

fn main() {
    let tasks = vec![ Task::new("This is a task") ];
    let mut app = App::new(tasks);
    app.run().unwrap();
}
