use crate::task::List;
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

pub struct App {
    lists: Vec<List>,
    current_list_index: usize,
    current_task_index: usize,
}

impl App {
    pub fn new(lists: Vec<List>) -> Self {
        Self {
            lists,
            current_list_index: 0,
            current_task_index: 0,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // Saving the start position of the app
        execute!(
            stdout(),
            SavePosition,
            cursor::SetCursorStyle::SteadyUnderScore
        )?;
        enable_raw_mode()?;

        loop {
            execute!(stdout(), RestorePosition, Clear(ClearType::FromCursorDown))?;
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
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') => self.move_to_next_task(),
                    KeyCode::Char('k') => self.move_to_prev_task(),
                    KeyCode::Char('l') => self.move_to_next_list(),
                    KeyCode::Char('h') => self.move_to_prev_list(),
                    KeyCode::Char('D') => self.delete_current_list(),
                    KeyCode::Char('d') => self.delete_current_task(),
                    KeyCode::Char('N') => self.create_new_list()?,
                    KeyCode::Char('n') => self.create_new_task()?,
                    KeyCode::Char(' ') => {
                        self.lists[self.current_list_index].toggle_task(self.current_task_index)
                    }
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

        Ok(())
    }

    fn draw(&self, list: &List) -> Result<()> {
        fn println<T: ToString>(text: T) -> Result<()> {
            let text = format!("{}\n\r", text.to_string());
            execute!(stdout(), Print(text))?;
            Ok(())
        }

        // At the moment only prints the first list
        println(list.name().bold())?;
        for task in list.tasks_iter() {
            println(task.to_string())?;
        }

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
        execute!(stdout(), RestorePosition, Clear(ClearType::FromCursorDown))?;

        let mut name = String::new();

        loop {
            execute!(
                stdout(),
                cursor::Show,
                Clear(ClearType::CurrentLine),
                Print(format!("\r{}", &name))
            )?;
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char(char) => name.push(char),
                    KeyCode::Backspace => {
                        name.pop();
                    }
                    KeyCode::Enter => break,
                    _ => (),
                }
            }
        }
        let list = List::new(name);
        self.lists.insert(self.current_list_index + 1, list);
        self.current_list_index += 1;
        Ok(())
    }

    fn delete_current_list(&mut self) {
        if self.lists.len() > 1 {
            self.lists.remove(self.current_list_index);
            self.current_list_index = self.current_list_index.saturating_sub(1);
        }
    }

    fn create_new_task(&mut self) -> Result<()> {
        execute!(
            stdout(),
            RestorePosition,
            cursor::MoveDown((self.lists[self.current_list_index].length() + 1) as u16),
        )?;

        let mut description = String::new();

        loop {
            execute!(
                stdout(),
                RestorePosition,
                cursor::MoveDown((self.lists[self.current_list_index].length() + 1) as u16),
                Clear(ClearType::FromCursorDown),
                Print(format!("\r{} {}", "[ ]".bold(), &description))
            )?;
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char(char) => description.push(char),
                    KeyCode::Backspace => {
                        description.pop();
                    }
                    KeyCode::Enter => break,
                    _ => (),
                }
            }
        }

        self.lists[self.current_list_index].add_task(description);
        Ok(())
    }

    fn delete_current_task(&mut self) {
        self.lists[self.current_list_index].delete_task(self.current_task_index);
        self.current_task_index = self.current_task_index.saturating_sub(1);
    }
}
