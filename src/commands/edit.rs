use super::ToDoFile;
use super::Command;

pub struct EditCommand {
    pub index: usize,
    pub field: String,  // trocar por um enum Field { Name, Description }
    pub value: String,
}

impl Command for EditCommand {
    fn execute(self, todo_file: &ToDoFile) -> () {

        if let Ok(todo) = todo_file.load() {
            /*
            esta verificação tem que ser feita no parsing do comando, não aqui

            if args.len() != 5 {
                println!("Please pass the index of the todo, the name and the description of the todo");
                return Ok(());
            } 
            */

            let mut todo_list = todo;


            // este match deve substituir as strings por variantes de um enum criado no parsing dos argumentos
            match  self.field.as_str() {
                "name" | "n"        => todo_list.change_todo_name(self.index, self.value),
                "description" | "d" => todo_list.change_todo_description(self.index, self.value),
                _ => println!("Invalid field"),
            }

            todo_list.print_all_todos();
            let _ = todo_file.save(&todo_list);
            return;
        }

        println!("Todo file not found");
    }
}
