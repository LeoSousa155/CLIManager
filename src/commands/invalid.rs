use super::ToDoFile;
use super::Command;


pub struct InvalidCommand {}


impl Command for InvalidCommand {
    fn execute(&self, _todo_file: &ToDoFile) {
        println!("Invalid command, use 'help' to see the list of commands")
    }
}