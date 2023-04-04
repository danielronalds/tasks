mod serialisation;
mod task;

pub use crate::app::serialisation::{deserialise, new_tasks_data, serialise};

use crate::app::task::List;

use colored::Colorize;
use crossterm::{
    cursor::{self, RestorePosition, SavePosition},
    event::{read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Result,
};
use std::io::stdout;

fn println<T: ToString>(text: T) -> Result<()> {
    let text = format!("{}\n\r", text.to_string());
    execute!(stdout(), Print(text))?;
    Ok(())
}

pub struct TasksApp {
    lists: Vec<List>,
    current_list_index: usize,
    current_task_index: usize,
}

impl TasksApp {
    pub fn new(lists: Vec<List>) -> Self {
        Self {
            lists,
            current_list_index: 0,
            current_task_index: 0,
        }
    }

    pub fn run(&mut self) -> Result<Vec<List>> {
        // Saving the start position of the app
        execute!(
            stdout(),
            SavePosition,
            cursor::SetCursorStyle::SteadyUnderScore
        )?;
        enable_raw_mode()?;

        loop {
            execute!(
                stdout(),
                RestorePosition,
                Clear(ClearType::FromCursorDown),
                cursor::SetCursorStyle::SteadyUnderScore
            )?;
            self.draw(&self.lists[self.current_list_index])?;
            execute!(
                stdout(),
                RestorePosition,
                cursor::MoveDown((self.current_task_index + 1) as u16),
                cursor::MoveRight(1)
            )?;

            if self.lists[self.current_list_index].length() == 0 {
                execute!(stdout(), cursor::Hide)?;
            } else {
                execute!(stdout(), cursor::Show)?;
            }

            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Down | KeyCode::Char('j') => self.move_to_next_task(),
                    KeyCode::Up | KeyCode::Char('k') => self.move_to_prev_task(),
                    KeyCode::Right | KeyCode::Char('l') => self.move_to_next_list(),
                    KeyCode::Left | KeyCode::Char('h') => self.move_to_prev_list(),
                    KeyCode::Char('D') => self.delete_current_list()?,
                    KeyCode::Char('d') => {
                        if let Event::Key(key) = read()? {
                            match key.code {
                                KeyCode::Char('d') => self.delete_current_task(),
                                KeyCode::Char('c') => self.delete_completed_tasks(),
                                KeyCode::Char('C') => self.delete_completed_tasks_on_all_lists(),
                                _ => (),
                            }
                        }
                    }
                    KeyCode::Char('N') => self.create_new_list()?,
                    KeyCode::Char('n') => self.create_new_task()?,
                    KeyCode::Char('r') => self.reword_current_task()?,
                    KeyCode::Char('R') => self.rename_current_list()?,
                    KeyCode::Char('s') => self.sort_current_list(),
                    KeyCode::Char('S') => self.sort_all_lists(),
                    KeyCode::Char('?') => self.draw_help()?,
                    KeyCode::Char(' ') => {
                        self.lists[self.current_list_index].toggle_task(self.current_task_index)
                    }
                    KeyCode::Char('q') => break,
                    _ => (),
                }
            }
        }
        disable_raw_mode()?;
        execute!(
            stdout(),
            cursor::SetCursorStyle::DefaultUserShape,
            RestorePosition,
            Clear(ClearType::FromCursorDown)
        )?;

        Ok(self.lists.clone())
    }

    fn draw(&self, list: &List) -> Result<()> {
        let title = format!(
            "({}/{}) {}",
            self.current_list_index + 1,
            self.lists.len(),
            list.name(),
        );

        println(title)?;
        for task in list.tasks_iter() {
            println(task.to_string())?;
        }

        Ok(())
    }

    fn draw_help(&self) -> Result<()> {
        execute!(stdout(), RestorePosition, Clear(ClearType::FromCursorDown))?;

        println("Keybinds")?;
        let keybinds = vec![
            "j/k      Move between tasks",
            "h/l      Move between lists",
            "space    Toggle current tasks status",
            "n        Create new task",
            "N        Create new list",
            "r        Reword current task",
            "R        Rename current list",
            "dd       Delete current task",
            "dc       Delete completed tasks from the current list",
            "dC       Delete completed tasks from the all lists",
            "D        Delete current list",
            "s        Sorts the current list",
            "S        Sorts all lists",
            "?        Show this menu",
            "q        Quit",
        ];

        for keybind in keybinds {
            println(keybind)?;
        }

        println("\nPress any key to return")?;
        read()?;

        Ok(())
    }

    fn move_to_next_list(&mut self) {
        if self.current_list_index + 1 < self.lists.len() {
            self.current_list_index += 1;
            self.current_task_index = 0;
        }
    }

    fn move_to_prev_list(&mut self) {
        self.current_list_index = self.current_list_index.saturating_sub(1);
        if self.current_task_index >= self.lists[self.current_list_index].length() {
            self.current_task_index = 0;
        }
    }

    fn move_to_next_task(&mut self) {
        if self.current_task_index + 1 < self.lists[self.current_list_index].length() {
            self.current_task_index += 1;
        }
    }

    fn move_to_prev_task(&mut self) {
        self.current_task_index = self.current_task_index.saturating_sub(1);
    }

    fn create_new_list(&mut self) -> Result<()> {
        execute!(
            stdout(),
            RestorePosition,
            Clear(ClearType::FromCursorDown),
            cursor::Show,
            cursor::SetCursorStyle::SteadyBlock
        )?;

        let mut name = String::new();

        loop {
            let print_input = format!(
                "\r({}/{}) {}",
                self.current_list_index + 2,
                self.lists.len() + 1,
                &name,
            );
            execute!(
                stdout(),
                cursor::Show,
                Clear(ClearType::CurrentLine),
                Print(print_input)
            )?;
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char(char) => name.push(char),
                    KeyCode::Backspace => {
                        name.pop();
                    }
                    KeyCode::Enter => break,
                    KeyCode::Esc => return Ok(()),
                    _ => (),
                }
            }
        }
        let list = List::new(name);
        self.lists.insert(self.current_list_index + 1, list);
        self.current_list_index += 1;
        Ok(())
    }

    fn rename_current_list(&mut self) -> Result<()> {
        execute!(
            stdout(),
            RestorePosition,
            Clear(ClearType::CurrentLine),
            cursor::Show,
            cursor::SetCursorStyle::SteadyBlock
        )?;

        let mut new_name = self.lists[self.current_list_index].name();

        loop {
            let print_input = format!(
                "\r({}/{}) {}",
                self.current_list_index + 1,
                self.lists.len(),
                &new_name,
            );
            execute!(
                stdout(),
                cursor::Show,
                Clear(ClearType::CurrentLine),
                Print(print_input)
            )?;
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char(char) => new_name.push(char),
                    KeyCode::Backspace => {
                        new_name.pop();
                    }
                    KeyCode::Enter => break,
                    KeyCode::Esc => return Ok(()),
                    _ => (),
                }
            }
        }
        self.lists[self.current_list_index].rename_list(new_name);
        Ok(())
    }

    fn delete_current_list(&mut self) -> Result<()> {
        self.goto_empty_line()?;
        let message = format!(
            "[{}] This will delete this list, are you sure? y/N",
            "!".bright_red()
        );
        execute!(stdout(), Print(message))?;

        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => (),
                _ => return Ok(()),
            }
        }

        if self.lists.len() > 1 {
            self.lists.remove(self.current_list_index);
            self.current_list_index = self.current_list_index.saturating_sub(1);
        }

        Ok(())
    }

    fn create_new_task(&mut self) -> Result<()> {
        self.goto_empty_line()?;
        execute!(stdout(), cursor::Show, cursor::SetCursorStyle::SteadyBlock,)?;

        let mut description = String::new();

        loop {
            execute!(
                stdout(),
                RestorePosition,
                cursor::MoveDown((self.lists[self.current_list_index].length() + 1) as u16),
                Clear(ClearType::FromCursorDown),
                Print(format!("\r{} {}", "[ ]", &description))
            )?;
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char(char) => description.push(char),
                    KeyCode::Backspace => {
                        description.pop();
                    }
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Enter => break,
                    _ => (),
                }
            }
        }

        self.lists[self.current_list_index].add_task(description);
        Ok(())
    }

    fn reword_current_task(&mut self) -> Result<()> {
        if self.current_task_index >= self.lists[self.current_list_index].length() {
            return Ok(());
        }

        execute!(stdout(), cursor::Show, cursor::SetCursorStyle::SteadyBlock,)?;

        let task = self.lists[self.current_list_index]
            .tasks_iter()
            .nth(self.current_task_index)
            .expect("We know this task exists so this can't fail");

        let mut description = task.description();

        loop {
            execute!(
                stdout(),
                RestorePosition,
                cursor::MoveDown((self.current_task_index + 1) as u16),
                Clear(ClearType::CurrentLine),
                Print(format!(
                    "\r{} {}",
                    match task.status() {
                        true => format!("[{}]", "✔".bright_green()),
                        false => "[ ]".to_string(),
                    },
                    &description
                ))
            )?;
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char(char) => description.push(char),
                    KeyCode::Backspace => {
                        description.pop();
                    }
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Enter => break,
                    _ => (),
                }
            }
        }

        self.lists[self.current_list_index].reword_task(self.current_task_index, description);
        Ok(())
    }

    fn sort_current_list(&mut self) {
        self.lists[self.current_list_index].sort_list();
    }

    fn sort_all_lists(&mut self) {
        self.lists = self
            .lists
            .iter_mut()
            .map(|list| {
                list.sort_list();
                list.to_owned()
            })
            .collect();
    }

    fn delete_current_task(&mut self) {
        self.lists[self.current_list_index].delete_task(self.current_task_index);
        self.current_task_index = self.current_task_index.saturating_sub(1);
    }

    fn delete_completed_tasks(&mut self) {
        self.lists[self.current_list_index].delete_completed_tasks();
        self.current_task_index = 0;
    }

    fn delete_completed_tasks_on_all_lists(&mut self) {
        for list in &mut self.lists {
            list.delete_completed_tasks();
        }
        self.current_task_index = 0;
    }

    fn goto_empty_line(&mut self) -> Result<()> {
        execute!(
            stdout(),
            RestorePosition,
            cursor::MoveDown((self.lists[self.current_list_index].length() + 1) as u16),
        )?;
        Ok(())
    }
}
