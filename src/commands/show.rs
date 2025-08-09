use super::ToDoFile;
use super::Command;


pub struct ShowCommand {
    pub options: String
}


impl Command for ShowCommand {
    fn execute(self, todo_file: &ToDoFile) -> () {
        match todo_file.load() {
            Ok(todo) => {
                /*
                verificar no parsing do comando

                if args.len() == 2 {
                    todo.print_all_todos();
                    return Ok(());
                } 
                */

                match self.options.as_str() {
                    "-m" => todo.print_completed_todos(),
                    "-u" => todo.print_incomplete_todos(),
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
            },
            Err(_) => println!("Todo file not found"),
        }
    }
}