mod task;
mod app;

use app::App;
use task::List;

fn main() {
    let mut lists = vec![ List::new("Main") ];
    lists[0].add_task("this is a task");
    lists[0].add_task("this is another task");
    let mut app = App::new(lists);
    app.run().unwrap();
}
