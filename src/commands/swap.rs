use super::ToDoFile;
use super::Command;


pub struct SwapCommand {
    pub index1: usize,
    pub index2: usize,
}


impl SwapCommand {
    pub fn new(args: &Vec<String>) -> Result<Self, String> {
        if args.len() < 4 { 
            return Err("Error: 'swap' requires two indexes.".to_string());
        } 
        let index1 = match args[2].parse::<usize>() {
            Ok(idx) => idx,
            Err(_) => return Err("Error: The first index must be a valid number.".to_string())
        };
        let index2 = match args[3].parse::<usize>() {
            Ok(idx) => idx,
            Err(_) => return Err("Error: The second index must be a valid number.".to_string())
        };
        Ok(SwapCommand{ index1, index2 })
    }
}


impl Command for SwapCommand {
    fn execute(&self, todo_file: &ToDoFile) -> () {
        if let Ok(todo) = todo_file.load() {
            let mut todo_list = todo;
            if let Err(e) = todo_list.swap_todos(self.index1, self.index2) {
                println!("Error: {:?}", e);
                return;
            }
            todo_list.print_all_todos();
            let _ = todo_file.save(&todo_list); 
            return;
        }
        println!("Todo file not found");
    }
}