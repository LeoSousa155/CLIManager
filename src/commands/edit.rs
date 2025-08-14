use super::ToDoFile;
use super::Command;


enum Field {
    Name,
    Description
}


pub struct EditCommand {
    index: usize,
    field: Field,
    value: String,
}


impl EditCommand {
    pub fn new(args: &Vec<String>) -> Result<Self, String> {
        if args.len() < 5 { 
            return Err("Error: 'edit' requires an index, a field and a value.".to_string());
        }
        let index = match args[2].parse::<usize>() {
            Ok(index) => index,
            Err(_) => return Err("Error: The index must be a valid number.".to_string())
        };
        let field: Field = match args[3].as_str() {
            "name"        | "n" => Field::Name,
            "description" | "d" => Field::Description,
            _ => return Err("Error: Field value is not valid".to_string())
        };
        let value = args[4].clone();
        Ok(EditCommand{ index, field, value })
    }
}


impl Command for EditCommand {
    fn execute(&self, todo_file: &ToDoFile) -> () {
        if let Ok(todo) = todo_file.load() {
            let mut todo_list = todo;
            let result = match self.field {
                Field::Name        => todo_list.change_todo_name(self.index, self.value.clone()),
                Field::Description => todo_list.change_todo_description(self.index, self.value.clone()),
            };

            if let Err(e) = result {
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
