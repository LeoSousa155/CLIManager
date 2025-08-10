use super::ToDoFile;
use super::Command;

pub struct ToggleCommand {
    pub index: usize
}

impl Command for ToggleCommand {
    fn execute(self, todo_file: &ToDoFile) {
        if let Ok(todo) = todo_file.load() {
            let mut todo_list = todo;
            todo_list.toggle_todo(self.index);

            todo_list.print_all_todos();
            let _ = todo_file.save(&todo_list);
            return;
        }
        println!("Todo file not found");
    }
}