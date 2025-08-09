use super::ToDoFile;
use super::Command;


pub struct ClearCommand {}


impl Command for ClearCommand {
    fn execute(self, todo_file: &ToDoFile) -> () {
        match todo_file.load() {
            Ok(todo) => {
                let mut todo_list = todo;
                todo_list.clear();
                println!("All todos cleared");
                let _ = todo_file.save(&todo_list);
            },
            Err(_) => println!("Todo file not found"),
        }
    }
}