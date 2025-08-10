use super::ToDoFile;
use super::Command;

pub struct SwapCommand {
    pub index1: usize,
    pub index2: usize,
}

impl Command for SwapCommand {
    fn execute(self, todo_file: &ToDoFile) -> () {

        if let Ok(todo) = todo_file.load() {
             /*
            Esta verificação tem que ser feita no parser dos argumentos do programa

            if args.len() != 4 {
                println!("Please pass the index of the todo to be moved and the index of the todo to be moved to");
                return Ok(());
            }
            */

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