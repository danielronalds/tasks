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
}

impl App {
    pub fn new(lists: Vec<List>) -> Self {
        Self { lists }
    }

    pub fn run(&mut self) -> Result<()> {
        // Saving the start position of the app
        execute!(
            stdout(),
            SavePosition,
            cursor::SetCursorStyle::SteadyUnderScore
        )?;
        enable_raw_mode()?;

        let mut current_task_index: usize = 0;

        let mut current_list_index: usize = 0;

        loop {
            execute!(stdout(), RestorePosition, Clear(ClearType::FromCursorDown))?;
            self.draw(&self.lists[current_list_index])?;
            execute!(
                stdout(),
                RestorePosition,
                cursor::MoveDown((current_task_index + 1) as u16),
                cursor::MoveRight(1)
            )?;

            if self.lists[current_list_index].length() == 0 {
                execute!(stdout(), cursor::Hide)?;
            } else {
                execute!(stdout(), cursor::Show)?;
            }

            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') => {
                        if current_task_index + 1 < self.lists[current_list_index].length() {
                            current_task_index += 1;
                        }
                    }
                    KeyCode::Char('k') => {
                        current_task_index = current_task_index.saturating_sub(1);
                    }
                    KeyCode::Char('l') => {
                        if current_list_index + 1 < self.lists.len() {
                            current_list_index += 1;
                            current_task_index = 0;
                        }
                    }
                    KeyCode::Char('h') => {
                        current_list_index = current_list_index.saturating_sub(1);
                        if current_task_index >= self.lists[current_list_index].length() {
                            current_task_index = 0;
                        }
                    }
                    KeyCode::Char('D') => {
                        if self.lists.len() > 1 {
                            self.lists.remove(current_list_index);
                            current_list_index = current_list_index.saturating_sub(1);
                        }
                    }
                    KeyCode::Char('d') => {
                        self.lists[current_list_index].delete_task(current_task_index);
                        current_task_index = current_task_index.saturating_sub(1);
                    }
                    KeyCode::Char('N') => {
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
                        self.lists.insert(current_list_index + 1, list);
                        current_list_index += 1;
                    }
                    KeyCode::Char('n') => {
                        execute!(
                            stdout(),
                            RestorePosition,
                            cursor::MoveDown((self.lists[current_list_index].length() + 1) as u16),
                        )?;

                        let mut description = String::new();

                        loop {
                            execute!(
                                stdout(),
                                RestorePosition,
                                cursor::MoveDown(
                                    (self.lists[current_list_index].length() + 1) as u16
                                ),
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

                        self.lists[current_list_index].add_task(description);
                    }
                    KeyCode::Char(' ') => {
                        self.lists[current_list_index].toggle_task(current_task_index)
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
}
