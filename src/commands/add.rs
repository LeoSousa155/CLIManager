use super::{ToDoFile, ToDo};
use super::Command;


pub struct AddCommand {
    name: String,
    description: String
}


impl AddCommand {
    pub fn new(args: &Vec<String>) -> Result<Self, String> {
        if args.len() < 4 { 
            return Err("Error: 'add' requires a name and a description.".to_string()); 
        }
        let name = args[2].clone();
        let description = args[3].clone();
        Ok(AddCommand { name, description })
    }
}


impl Command for AddCommand {
    fn execute(&self, todo_file: &ToDoFile) -> () {
        if let Ok(todo) = todo_file.load() {
             let mut todo_list = todo;
            todo_list.add_new_todo(ToDo::new(self.name.clone(), self.description.clone()));

            todo_list.print_all_todos();
            let _ = todo_file.save(&todo_list);
            return;
        }
        println!("Todo file not found");
    }
}