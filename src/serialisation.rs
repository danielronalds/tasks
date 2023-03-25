use crate::task::List;
use std::fs::File;
use std::io::prelude::Write;

const FILE_NAME: &str = ".tasks.md";

pub fn serialize(lists: Vec<List>) -> std::io::Result<()> {
    let mut file = File::create(FILE_NAME)?;

    for list in lists {
        writeln!(file, "{}", list.name())?;
        for task in list.tasks_iter() {
            writeln!(file, "- [{}] {}", match task.status() {
                true => "x",
                false => " "
            }, task.description())?;
        }
    }

    file.flush()?;
    Ok(())
}
