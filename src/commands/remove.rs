use super::ToDoFile;
use super::Command;


pub struct RemoveCommand {
    pub index: usize
}


impl RemoveCommand {
    pub fn new(args: &Vec<String>) -> Result<Self, String> {
        if args.len() < 3 { 
            return Err("Error: 'remove' requires an index.".to_string());
        }
        let index =  match args[2].parse::<usize>() {
            Ok(index) => index,
            Err(_) => return Err("Error: The index must be a valid number.".to_string())
        };
        Ok(RemoveCommand { index })
    }
}


impl Command for RemoveCommand {
    fn execute(&self, todo_file: &ToDoFile) {
        if let Ok(todo) = todo_file.load() {
            let mut todo_list = todo;
            if let Err(e) = todo_list.remove_todo(self.index) {
                println!("{}", e);
                return;
            }
            todo_list.print_all_todos();
            let _ = todo_file.save(&todo_list);
            return;
        }
        println!("Todo file not found");
    }
}