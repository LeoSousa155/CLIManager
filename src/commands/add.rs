use super::{ToDoFile, ToDo};
use super::Command;

pub struct AddCommand {
    pub name: String,
    pub description: String
}


impl Command for AddCommand {
    fn execute(self, todo_file: &ToDoFile) -> () {
        match todo_file.load() {
            Ok(todo) => {
                let mut todo_list = todo;
                todo_list.add_new_todo(ToDo::new(self.name, self.description));

                todo_list.print_all_todos();
                let _ = todo_file.save(&todo_list);
            },
            Err(e) => println!("Todo file not found: {:?}", e),
        }
    }
}