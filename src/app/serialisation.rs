use crate::app::task::List;
use std::fs::File;
use std::io::{
    prelude::{Read, Write},
    Result,
};

/// The filename that the app should serialise and deserialise from
const FILE_NAME: &str = ".tasks.md";

/// Writes the given vector of list structs to the .tasks.md file
///
/// # Arguments
///
/// * `lists` - The vector of list's to serialise
pub fn serialise(lists: Vec<List>) -> Result<()> {
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
        writeln!(file)?;
    }

    file.flush()?;
    Ok(())
}

/// Reads the .tasks.md file and returns a vector of list's
///
/// The following guidelines are followed when reading the file
/// - Empty lines are skipped
/// - Lines beginning with '- [x] ' or '- [ ] ' are added as task to the current lists
/// - Every line that is not empty and does not meet the previous criteria is treated as a new list
///
/// # Returns
///
/// The deserialised data as a vector of `List` structs
pub fn deserialise() -> Result<Vec<List>> {
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

        lists.push(
            List::new(line.to_string())
                .expect("We have checked to see if this string is empty already"),
        );
    }

    Ok(lists)
}

/// Generates a fresh start for the program
///
/// # Arguments
///
/// * `list_name` - The name the list should have
///
/// # Returns
///
/// A vector of `List` structs with one element, a `List` with the given name
pub fn new_tasks_data<T: ToString>(list_name: T) -> Vec<List> {
    let mut name = list_name.to_string();

    if name.is_empty() {
        name = "Main".to_string();
    }

    vec![List::new(name).expect("Name can never be empty")]
}
