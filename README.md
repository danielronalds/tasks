# tasks

A modal CLI task manager with vim-like keybindings

## Installation

To install it you'll need cargo

```
cargo install --git https://github.com/danielronalds/tasks.git
```

## Keybinds

| Key | Action |
| --- | ------ |
| j/k | Move between tasks |
| h/l | Move between lists |
| H/L | Move current task between lists |
| space | Toggle current tasks status |
| n | Create new task |
| N | Create new list |
| r | Reword current task |
| R | Rename current list |
| dd | Delete current task |
| dA | Delete all tasks from the current list |
| dc | Delete completed tasks from the current list |
| dC | Delete completed tasks from the all lists |
| D | Delete current list |
| yy | Yank current task |
| yA | Yank all tasks in the current list |
| p | Paste task/s in the clipboard below |
| P | Paste task/s in the clipboard above |
| s | Sorts the current list |
| S | Sorts all lists |
| G | Goto to the last task in the list |
| 1-9 | Move to the list corresponding to the number pressed |
| ? | Show help menu |
| q | Quit |
| q | Quit without saving changes |

**Note** Arrow keys can also be used
