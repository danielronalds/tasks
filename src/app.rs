mod serialisation;
mod task;

pub use crate::app::serialisation::{deserialise, new_tasks_data, serialise};

use crate::app::task::{List, Task};

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

/// Prints a string followed by a new line and carriage return to the stdout using Crossterm.
/// Works in raw mode
///
/// # Arguments
///
/// * `text` - The text to write to stdout
///
fn println<T: ToString>(text: T) -> Result<()> {
    let text = format!("{}\n\r", text.to_string());
    execute!(stdout(), Print(text))?;
    Ok(())
}

/// Waits for a key event, returning true if the user confirms the action. No by DefaultUserShape
///
/// # Returns
///
/// `true` if the key pressed is 'y' or 'Y'
fn get_confirmation() -> Result<bool> {
    match read()? {
        Event::Key(key) => match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => Ok(true),
            _ => Ok(false),
        },
        _ => Ok(false),
    }
}

/// The application
pub struct TasksApp {
    lists: Vec<List>,
    current_list_index: usize,
    current_task_index: usize,
    clipboard: Vec<Task>,
}

impl TasksApp {
    /// Builds a new instance of the tasks app
    ///
    /// # Arguments
    ///
    /// * `lists` - The lists the app should have to start with
    pub fn new(lists: Vec<List>) -> Self {
        Self {
            lists,
            current_list_index: 0,
            current_task_index: 0,
            clipboard: vec![],
        }
    }

    /// Runs the program
    ///
    /// # Returns
    ///
    /// The finished state of the lists after the program has run
    pub fn run(&mut self) -> Result<Option<Vec<List>>> {
        // Saving the start position of the app
        execute!(
            stdout(),
            SavePosition,
            cursor::SetCursorStyle::SteadyUnderScore
        )?;
        enable_raw_mode()?;

        let mut save_changes = true;

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
                    KeyCode::Char('L') => self.move_current_task_to_next_list(),
                    KeyCode::Left | KeyCode::Char('h') => self.move_to_prev_list(),
                    KeyCode::Char('H') => self.move_current_task_to_prev_list(),
                    KeyCode::Char('p') => self.paste_clipboard(1),
                    KeyCode::Char('P') => self.paste_clipboard(0),
                    KeyCode::Char('D') => self.delete_current_list()?,
                    KeyCode::Char('d') => {
                        if let Event::Key(key) = read()? {
                            match key.code {
                                KeyCode::Char('d') => self.delete_current_task(),
                                KeyCode::Char('A') => self.delete_all_tasks(),
                                KeyCode::Char('c') => self.delete_completed_tasks(),
                                KeyCode::Char('C') => self.delete_completed_tasks_on_all_lists(),
                                _ => (),
                            }
                        }
                    }
                    KeyCode::Char('y') => {
                        if let Event::Key(key) = read()? {
                            match key.code {
                                KeyCode::Char('y') => self.yank_current_task(),
                                KeyCode::Char('A') => self.yank_current_list(),
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
                    KeyCode::Char('G') => self.goto_bottom(),
                    KeyCode::Char('?') => self.draw_help()?,
                    KeyCode::Char(' ') => {
                        self.lists[self.current_list_index].toggle_task(self.current_task_index)
                    }
                    KeyCode::Char('q') => break,
                    KeyCode::Char('Q') => {
                        self.goto_empty_line()?;
                        execute!(
                            stdout(),
                            Print(format!(
                                "[{}] This will exit without saving, are you sure? y/N ",
                                "!".bright_red()
                            ))
                        )?;

                        if !get_confirmation()? {
                            continue;
                        }

                        save_changes = false;
                        break;
                    }
                    KeyCode::Char('1') => self.move_to_list(0),
                    KeyCode::Char('2') => self.move_to_list(1),
                    KeyCode::Char('3') => self.move_to_list(2),
                    KeyCode::Char('4') => self.move_to_list(3),
                    KeyCode::Char('5') => self.move_to_list(4),
                    KeyCode::Char('6') => self.move_to_list(5),
                    KeyCode::Char('7') => self.move_to_list(6),
                    KeyCode::Char('8') => self.move_to_list(7),
                    KeyCode::Char('9') => self.move_to_list(8),
                    _ => (),
                }
            }
        }

        disable_raw_mode()?;
        execute!(
            stdout(),
            cursor::SetCursorStyle::DefaultUserShape,
            cursor::Show,
            RestorePosition,
            Clear(ClearType::FromCursorDown)
        )?;

        match save_changes {
            true => Ok(Some(self.lists.clone())),
            false => Ok(None),
        }
    }

    /// Draws the given list of the app
    ///
    /// # Arguments
    ///
    /// * `list` - The list to draw
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

    /// Draws the help menu
    fn draw_help(&self) -> Result<()> {
        execute!(stdout(), RestorePosition, Clear(ClearType::FromCursorDown))?;

        println(format!("Tasks v{}", env!("CARGO_PKG_VERSION")))?;
        println("Keybinds")?;
        let keybinds = vec![
            "j/k      Move between tasks",
            "h/l      Move between lists",
            "H/L      Move current task between lists",
            "space    Toggle current tasks status",
            "n        Create new task",
            "N        Create new list",
            "r        Reword current task",
            "R        Rename current list",
            "dd       Delete current task",
            "dA       Delete all tasks from the current list",
            "dc       Delete completed tasks from the current list",
            "dC       Delete completed tasks from the all lists",
            "D        Delete current list",
            "yy       Yank current task",
            "yA       Yank all tasks in the current list",
            "p        Paste tasks in the clipboard below",
            "P        Paste tasks in the clipboard above",
            "s        Sorts the current list",
            "S        Sorts all lists",
            "G        Goto to the last task in the last",
            "1-9      Move to the list corresponding to the number pressed",
            "?        Show this menu",
            "q        Quit",
            "Q        Quit without saving",
        ];

        for keybind in keybinds {
            println(keybind)?;
        }

        println("\nPress any key to return")?;
        read()?;

        Ok(())
    }

    // Moves the current_list_index to the next list
    fn move_to_next_list(&mut self) {
        if self.current_list_index + 1 < self.lists.len() {
            self.current_list_index += 1;
            self.current_task_index = 0;
        }
    }

    // Moves the current_list_index to the previous list
    fn move_to_prev_list(&mut self) {
        self.current_list_index = self.current_list_index.saturating_sub(1);
        if self.current_task_index >= self.lists[self.current_list_index].length() {
            self.current_task_index = 0;
        }
    }

    /// Moves to the given list if it exists
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the list to move to
    fn move_to_list(&mut self, index: usize) {
        if index < self.lists.len() {
            self.current_list_index = index;
        }
    }

    /// Moves the task cursor down
    fn move_to_next_task(&mut self) {
        if self.current_task_index + 1 < self.lists[self.current_list_index].length() {
            self.current_task_index += 1;
        }
    }

    /// Moves the task cursor up
    fn move_to_prev_task(&mut self) {
        self.current_task_index = self.current_task_index.saturating_sub(1);
    }

    /// Gets the current task that the current_task_index is pointing to
    fn get_current_task(&self) -> Option<Task> {
        self.lists[self.current_list_index]
            .tasks_iter()
            .map(|x| x.to_owned())
            .nth(self.current_task_index)
    }

    /// Moves the current task to the next list, if there is one
    fn move_current_task_to_next_list(&mut self) {
        if self.current_list_index + 1 >= self.lists.len() {
            return;
        }

        let (description, status) = match self.get_current_task() {
            Some(task) => (task.description(), task.status()),
            None => return,
        };

        self.lists[self.current_list_index].delete_task(self.current_task_index);
        self.move_to_next_list();
        self.lists[self.current_list_index].add_task(description);
        self.current_task_index = self.lists[self.current_list_index].length() - 1;

        if status {
            self.lists[self.current_list_index].toggle_task(self.current_task_index);
        }
    }

    /// Moves the current task to the list previous to the current one, if there is one
    fn move_current_task_to_prev_list(&mut self) {
        if self.current_list_index == 0 {
            return;
        }

        let (description, status) = match self.get_current_task() {
            Some(task) => (task.description(), task.status()),
            None => return,
        };

        self.lists[self.current_list_index].delete_task(self.current_task_index);
        self.move_to_prev_list();
        self.lists[self.current_list_index].add_task(description);
        self.current_task_index = self.lists[self.current_list_index].length() - 1;

        if status {
            self.lists[self.current_list_index].toggle_task(self.current_task_index);
        }
    }

    /// Creates a new list
    fn create_new_list(&mut self) -> Result<()> {
        execute!(stdout(), RestorePosition, Clear(ClearType::FromCursorDown))?;
        let prompt = format!(
            "({}/{}) ",
            self.current_list_index + 2,
            self.lists.len() + 1
        );

        let prompt_length = prompt.len() as u16;

        let name = typing_line(prompt, prompt_length, String::new())?;

        if let Some(name) = name {
            if let Ok(list) = List::new(name) {
                self.lists.insert(self.current_list_index + 1, list);
                self.current_list_index += 1;
            }
        }
        Ok(())
    }

    /// Renames the current list
    fn rename_current_list(&mut self) -> Result<()> {
        execute!(stdout(), RestorePosition,)?;

        let prompt = format!("({}/{}) ", self.current_list_index + 1, self.lists.len());
        let prompt_length = prompt.len() as u16;
        let new_name = typing_line(
            prompt,
            prompt_length,
            self.lists[self.current_list_index].name(),
        )?;

        if let Some(new_name) = new_name {
            self.lists[self.current_list_index].rename_list(new_name);
        }
        Ok(())
    }

    /// Deletes the current list
    fn delete_current_list(&mut self) -> Result<()> {
        self.goto_empty_line()?;
        let message = format!(
            "[{}] This will delete this list, are you sure? y/N",
            "!".bright_red()
        );
        execute!(stdout(), Print(message))?;

        if !get_confirmation()? {
            return Ok(());
        }

        if self.lists.len() > 1 {
            self.lists.remove(self.current_list_index);
            self.current_list_index = self.current_list_index.saturating_sub(1);
        }

        Ok(())
    }

    /// Creates a new task and attempts to add it to the list
    fn create_new_task(&mut self) -> Result<()> {
        self.goto_empty_line()?;
        execute!(stdout(), Clear(ClearType::FromCursorDown))?;

        let description = typing_line("[ ] ", 4, String::new())?;

        if let Some(description) = description {
            self.lists[self.current_list_index].add_task(description);
        }
        Ok(())
    }

    /// Rewords the current task
    fn reword_current_task(&mut self) -> Result<()> {
        if self.current_task_index >= self.lists[self.current_list_index].length() {
            return Ok(());
        }

        execute!(
            stdout(),
            RestorePosition,
            cursor::MoveDown((self.current_task_index + 1) as u16),
        )?;

        let task = self.lists[self.current_list_index]
            .tasks_iter()
            .nth(self.current_task_index)
            .expect("We know this task exists so this can't fail");

        let description = typing_line(
            match task.status() {
                true => format!("[{}] ", "✔".bright_green()),
                false => "[ ] ".to_string(),
            },
            4,
            task.description(),
        )?;

        if let Some(description) = description {
            self.lists[self.current_list_index].reword_task(self.current_task_index, description);
        }
        Ok(())
    }

    /// Sorts the current list
    fn sort_current_list(&mut self) {
        self.lists[self.current_list_index].sort_list();
    }

    /// Sorts all the lists in the app
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

    /// Pastes the task in the clipboard, if it is not None, into the current list
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset of where to paste the task from the current_task_index
    fn paste_clipboard(&mut self, offset: usize) {
        for task in &self.clipboard {
            self.lists[self.current_list_index]
                .insert_task(self.current_task_index + offset, task.clone());
            // If the task is being pasted as the first task in a list, then moving the current
            // task index results in a ui glitch
            if self.lists[self.current_list_index].length() > 1 {
                self.current_task_index += offset;
            }
        }
    }

    /// Copies the current task into the clipboard
    fn yank_current_task(&mut self) {
        if let Some(current_task) = self.get_current_task() {
            // Clearing the list, as yanking only gets the current task
            self.clipboard = vec![];
            self.clipboard.push(current_task);
        }
    }

    /// Copies all the tasks in the current list into the clipboard
    fn yank_current_list(&mut self) {
        self.clipboard = self.lists[self.current_list_index]
            .tasks_iter()
            .map(|x| x.to_owned())
            .collect();
    }

    /// Deletes the current task
    fn delete_current_task(&mut self) {
        self.yank_current_task();
        self.lists[self.current_list_index].delete_task(self.current_task_index);
        self.current_task_index = self.current_task_index.saturating_sub(1);
    }

    /// Removes all completed tasks from the current list
    fn delete_completed_tasks(&mut self) {
        self.lists[self.current_list_index].delete_completed_tasks();
        self.current_task_index = 0;
    }

    /// Removes all tasks from the current list
    fn delete_all_tasks(&mut self) {
        self.yank_current_list();
        self.lists[self.current_list_index].delete_all_tasks();
        self.current_task_index = 0;
    }

    /// Deletes all completed tasks on every list in the app
    fn delete_completed_tasks_on_all_lists(&mut self) {
        for list in &mut self.lists {
            list.delete_completed_tasks();
        }
        self.current_task_index = 0;
    }

    /// Moves the cursor to the next empty line
    fn goto_empty_line(&mut self) -> Result<()> {
        execute!(
            stdout(),
            RestorePosition,
            cursor::MoveDown((self.lists[self.current_list_index].length() + 1) as u16),
        )?;
        Ok(())
    }

    fn goto_bottom(&mut self) {
        self.current_task_index = self.lists[self.current_list_index].length() - 1;
    }
}

/// Clears the current line and provides a textbox for the user to type input into
///
/// # Arguments
///
/// * `prompt` - What the textbox prompt should be
/// * `prompt_len` - The length of the prompt
/// * `content` - The initial content of the textfield
///
/// # Returns
///
/// If no errors occured, an Option containg None if the user canceled the operation, or Some
/// containing what the user inputed
fn typing_line<T: ToString>(prompt: T, prompt_len: u16, content: String) -> Result<Option<String>> {
    execute!(stdout(), cursor::Show, cursor::SetCursorStyle::SteadyBlock)?;

    let mut output = content;
    let prompt = prompt.to_string();

    let mut cursor = output.len();

    loop {
        execute!(
            stdout(),
            Clear(ClearType::CurrentLine),
            Print(format!("\r{}{}", prompt, &output)),
            cursor::MoveToColumn(0),
            cursor::MoveRight(prompt_len + (cursor as u16))
        )?;
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char(char) => {
                    output.insert(cursor, char);
                    cursor += 1;
                }
                KeyCode::Backspace => {
                    if !output.is_empty() {
                        output.remove(cursor - 1);
                        cursor = cursor.saturating_sub(1);
                    }
                }
                KeyCode::Esc => return Ok(None),
                KeyCode::Enter => break,
                KeyCode::Left => cursor = cursor.saturating_sub(1),
                KeyCode::Right => {
                    if cursor != output.len() {
                        cursor += 1;
                    }
                }
                _ => (),
            }
        }
    }

    Ok(Some(output))
}
