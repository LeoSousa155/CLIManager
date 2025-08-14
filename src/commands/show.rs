use super::{ToDoFile, ToDoList};
use super::Command;


pub struct ShowCommand {
    pub options: Option<String>
}


impl ShowCommand {
    pub fn new(args: &Vec<String>) -> Self {
        let options =  if args.len() > 2 { 
            Some(args[2].clone()) } 
        else {
            None 
        };
        ShowCommand { options }
    }


    fn show_todos(&self, todo: &ToDoList) {
        if self.options == Option::None {
            todo.print_all_todos();
            return;
        }
        let option = self.options.clone().unwrap();
        match option.as_str() {
            "-m"  => todo.print_completed_todos(),
            "-u"  => todo.print_incomplete_todos(),
            "-mu" => {
                    todo.print_completed_todos();
                    todo.print_incomplete_todos();
                },
            "-um" => {
                    todo.print_incomplete_todos();
                    todo.print_completed_todos();
                },
            _ => println!("Invalid argument, use 'help' to see the list of commands"),
        }
    }   
}


impl Command for ShowCommand {
    fn execute(&self, todo_file: &ToDoFile) -> () {
        if let Ok(todo) = todo_file.load() {
            self.show_todos(&todo);
            return;       
        }
        println!("Todo file not found");
    }
}