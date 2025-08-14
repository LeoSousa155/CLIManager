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
    let executer = match get_command_executer(&args) {
        Ok(cmd) => cmd,
        Err(e) => {
            println!("{}", e);
            return Ok(());
        }
    };
    let todo_file = ToDoFile::new(String::from("./todo.json"));
    executer.execute(&todo_file);
    Ok(())
}


fn get_command_executer(args: &Vec<String>) -> Result<Box<dyn Command>, String> {
    match args[1].as_str() {
        "init"     => Ok(Box::new(commands::InitCommand {})),
        "clear"    => Ok(Box::new(commands::ClearCommand {})),
        "reset"    => Ok(Box::new(commands::ResetCommand {})),
        "help"     => Ok(Box::new(commands::HelpCommand {})),
        "show"     => Ok(Box::new(commands::ShowCommand::new(&args))),
        "add"      => commands::AddCommand::new(&args)
                        .map(|cmd| Box::new(cmd) as Box<dyn Command>),
        "toggle"   => commands::ToggleCommand::new(&args)
                        .map(|cmd| Box::new(cmd) as Box<dyn Command>),
        "remove"   => commands::RemoveCommand::new(&args)
                        .map(|cmd| Box::new(cmd) as Box<dyn Command>),
        "daltonic" => commands::DaltonicCommand::new(&args)
                        .map(|cmd| Box::new(cmd) as Box<dyn Command>),
        "edit"     => commands::EditCommand::new(&args)
                        .map(|cmd| Box::new(cmd) as Box<dyn Command>),
        "swap"     => commands::SwapCommand::new(&args)
                        .map(|cmd| Box::new(cmd) as Box<dyn Command>),
        _ => Ok(Box::new(commands::InvalidCommand {})),
    }
}