use super::ToDoFile;
use super::Command;

pub struct InitCommand {}

impl Command for InitCommand {
    fn execute(self, todo_file: &ToDoFile) -> () {
        match todo_file.init() {
            Ok(_) => println!("Todo file created successfully"),
            Err(_) => println!("Todo file already exists"),
        }
    }
}
