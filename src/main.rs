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
    
    if args.len() < 2 {
        println!("Please pass arguments to the command. Use 'help' to see the list of commands");
        return Ok(());
    }

    let executer: Box<dyn Command> = match args[1].as_str() {
        "init" => Box::new(commands::InitCommand{}),
        "clear" => Box::new(commands::ClearCommand{}),
        "reset" => Box::new(commands::ResetCommand{}),
        "help" => Box::new(commands::HelpCommand{}),
        "show" => Box::new(commands::ShowCommand::new(&args)),
        "add" => match commands::AddCommand::new(&args) {
            Ok(command) => Box::new(command),
            Err(e) => { println!("{}", e); return Ok(()); }
        },
        "toggle" => match commands::ToggleCommand::new(&args) {
            Ok(command) => Box::new(command),
            Err(e) => { println!("{}", e); return Ok(()); }
        },
        "remove" => match commands::RemoveCommand::new(&args) {
            Ok(command) => Box::new(command),
            Err(e) => { println!("{}", e); return Ok(()); }
        },
        "daltonic" => match commands::DaltonicCommand::new(&args) {
            Ok(command) => Box::new(command),
            Err(e) => { println!("{}", e); return Ok(()); }
        },
        "edit" => match commands::EditCommand::new(&args) {
            Ok(command) => Box::new(command),
            Err(e) => { println!("{}", e); return Ok(()); }
        },
        "swap" => match commands::SwapCommand::new(&args) {
            Ok(command) => Box::new(command),
            Err(e) => { println!("{}", e); return Ok(()); }
        },
        _ => Box::new(commands::InvalidCommand{}),
    };
    let todo_file = ToDoFile::new(String::from("./todo.json"));
    executer.execute(&todo_file);
    Ok(())
}