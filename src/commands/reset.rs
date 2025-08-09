use super::ToDoFile;
use super::Command;


pub struct ResetCommand {}


impl Command for ResetCommand {
    fn execute(self, todo_file: &ToDoFile) {
        match todo_file.load() {
            Ok(todo) => {
                let mut todo_list = todo;
                todo_list.reset();

                todo_list.print_all_todos();
                let _ = todo_file.save(&todo_list);
            },
            Err(_) => println!("Todo file not found"),
        }
    }
}