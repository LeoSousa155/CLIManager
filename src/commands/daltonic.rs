use super::ToDoFile;
use super::Command;


pub struct DaltonicCommand {
    pub active: bool
}


impl Command for DaltonicCommand {
    fn execute(self, todo_file: &ToDoFile) -> () {
        match todo_file.load() {
            Ok(todo) => {
                let mut todo_list = todo;
                todo_list.set_daltonic_mode(self.active);
                println!("Daltonic mode set to: {}", self.active);
                let _ = todo_file.save(&todo_list);
            },
            Err(_) => println!("Todo file not found"),
        }
    }
}