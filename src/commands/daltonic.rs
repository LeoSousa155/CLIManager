use super::ToDoFile;
use super::Command;


pub struct DaltonicCommand {
    pub active: bool
}


impl DaltonicCommand {
    pub fn new(args: &Vec<String>) -> Result<Self, String> {
        if args.len() < 3 { 
            return Err("Error: 'daltonic' requires to be 'true' or 'false'.".to_string());
        }
        let active = match args[2].parse::<bool>() {
            Ok(active) => active,
            Err(_) => return Err("Error: Daltonic mode can only be true or false".to_string())
        };
        Ok(DaltonicCommand { active })
    }
}


impl Command for DaltonicCommand {
    fn execute(&self, todo_file: &ToDoFile) -> () {
        if let Ok(todo) = todo_file.load() {
            let mut todo_list = todo;
            todo_list.set_daltonic_mode(self.active);
            println!("Daltonic mode set to: {}", self.active);
            let _ = todo_file.save(&todo_list);
            todo_list.print_all_todos();
            return;
        }
        println!("Todo file not found");
    }
}