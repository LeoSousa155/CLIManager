use super::ToDoFile;
use super::Command;


pub struct ClearCommand {}


impl Command for ClearCommand {
    fn execute(&self, todo_file: &ToDoFile) -> () {
        if let Ok(todo) = todo_file.load() {
            let mut todo_list = todo;
            todo_list.clear();
            println!("All todos cleared");
            let _ = todo_file.save(&todo_list);
            return;
        }
         println!("Todo file not found");
    }
}