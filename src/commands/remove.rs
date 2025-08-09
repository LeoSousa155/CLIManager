use super::ToDoFile;
use super::Command;

pub struct RemoveCommand {
    pub index: usize
}

impl Command for RemoveCommand {
    fn execute(self, todo_file: &ToDoFile) {
        match todo_file.load() {
            Ok(todo) => {
                let mut todo_list = todo;
                todo_list.remove_todo(self.index);

                todo_list.print_all_todos();
                let _ = todo_file.save(&todo_list);
            },
            Err(e) => println!("Todo file not found: {:?}", e),
        }
    }
}