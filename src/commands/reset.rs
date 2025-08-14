use super::ToDoFile;
use super::Command;


pub struct ResetCommand {}


impl Command for ResetCommand {
    fn execute(&self, todo_file: &ToDoFile) {
        if let Ok(todo) = todo_file.load() {
            let mut todo_list = todo;
            todo_list.reset();
            todo_list.print_all_todos();
            let _ = todo_file.save(&todo_list);
            return;
        }   
        println!("Todo file not found");
    }
}