use crate::task::Task;
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    event::{read, Event, KeyCode},
    cursor::{RestorePosition, SavePosition},
    execute, ExecutableCommand, Result,
};
use std::io::{stdout, Write};

pub struct App {
    tasks: Vec<Task>,
}

impl App {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }

    pub fn run(&mut self) -> Result<()> {
        // Saving the start position of the app
        execute!(stdout(), SavePosition)?;
        enable_raw_mode()?;
        loop {
            self.draw();

            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    _ => ()
                }
            }
        }
        execute!(stdout(), RestorePosition)?;
        disable_raw_mode()?;

        Ok(())
    }

    fn draw(&self) {}
}
