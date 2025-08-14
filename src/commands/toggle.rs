use super::ToDoFile;
use super::Command;


pub struct ToggleCommand {
    pub index: usize
}   


impl ToggleCommand {
    pub fn new(args: &Vec<String>) -> Result<Self, String> {
        if args.len() < 3 {
            return Err("Error: 'toggle' requires an index.".to_string()); 
        }
        let index = match args[2].parse::<usize>() {
            Ok(index) => index,
            Err(_) => return Err("Error: The index must be a valid number.".to_string())
        };
        Ok(ToggleCommand{ index })
    }
}


impl Command for ToggleCommand {
    fn execute(&self, todo_file: &ToDoFile) {
        if let Ok(todo) = todo_file.load() {
            let mut todo_list = todo;
            todo_list.toggle_todo(self.index);
            todo_list.print_all_todos();
            let _ = todo_file.save(&todo_list);
            return;
        }
        println!("Todo file not found");
    }
}