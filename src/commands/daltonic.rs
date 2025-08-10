use super::ToDoFile;
use super::Command;


pub struct DaltonicCommand {
    pub active: bool
}


impl Command for DaltonicCommand {
    fn execute(self, todo_file: &ToDoFile) -> () {
        if let Ok(todo) = todo_file.load() {
            let mut todo_list = todo;
            todo_list.set_daltonic_mode(self.active);
            println!("Daltonic mode set to: {}", self.active);
            let _ = todo_file.save(&todo_list);
            return;
        }
        println!("Todo file not found");
    }
}