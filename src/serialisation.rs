use crate::task::List;
use std::fs::File;
use std::io::prelude::{Read, Write};

const FILE_NAME: &str = ".tasks.md";

pub fn serialize(lists: Vec<List>) -> std::io::Result<()> {
    let mut file = File::create(FILE_NAME)?;

    for list in lists {
        writeln!(file, "{}", list.name())?;
        for task in list.tasks_iter() {
            writeln!(
                file,
                "- [{}] {}",
                match task.status() {
                    true => "x",
                    false => " ",
                },
                task.description()
            )?;
        }
        writeln!(file, "\n")?;
    }

    file.flush()?;
    Ok(())
}

pub fn deserialise() -> std::io::Result<Vec<List>> {
    let mut lists: Vec<List> = vec![];

    let mut file = File::open(FILE_NAME)?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }

        if line.len() > 5 && line[0..3].to_string() == "- [" {
            if let Some(list) = lists.last_mut() {
                list.add_task(line[6..].to_string());
                if line[3..4].to_string() == "x" {
                    list.toggle_task(list.length() - 1);
                }
            }
            continue;
        }

        lists.push(List::new(line.to_string()));
    }

    return Ok(lists);
}
