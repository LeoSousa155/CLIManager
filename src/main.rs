use std::env;

mod todo;
mod todo_list;
mod todo_file;
mod commands;

use todo_file::ToDoFile;
use commands::Command;


fn main() -> std::io::Result<()> {
    
    clearscreen::clear().expect("failed to clear screen");

    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();

    if args.len() < 2 {
        println!("Please pass arguments to the command. Use 'help' to see the list of commands");
        return Ok(());
    }

    let todo_file = ToDoFile::new(String::from("./todo.json"));

    match command {
        "init"   => commands::InitCommand{}.execute(&todo_file),
        "add"    => commands::AddCommand{ name: args[2].clone(), description: args[3].clone() }.execute(&todo_file),
        "toggle" => commands::ToggleCommand{ index: args[2].parse::<usize>().unwrap() }.execute(&todo_file),
        "remove" => commands::RemoveCommand{ index: args[2].parse::<usize>().unwrap() }.execute(&todo_file),
        "clear"  => commands::ClearCommand{}.execute(&todo_file),
        "reset"  => commands::ResetCommand{}.execute(&todo_file),
        "show"   => commands::ShowCommand{ options: args[2].clone() }.execute(&todo_file),
        "daltonic" => commands::DaltonicCommand{ active: args[2].parse::<bool>().unwrap() }.execute(&todo_file),
        "help" => commands::HelpCommand{}.execute(&todo_file),
        "edit"   => commands::EditCommand{ 
                index: args[2].parse::<usize>().unwrap(),
                field: args[3].clone(),
                value: args[4].clone() 
            }.execute(&todo_file),
        "swap"   => commands::SwapCommand{
                index1: args[2].parse::<usize>().unwrap(),
                index2: args[3].parse::<usize>().unwrap()
            }.execute(&todo_file),
        _ => println!("Invalid command, use 'help' to see the list of commands"),
    }

    Ok(())
}
