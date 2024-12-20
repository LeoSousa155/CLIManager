use std::env;
//use std::fs;

mod todo;

use todo::{ ToDo, ToDoList };


fn verify_command_arguments(args: &Vec<String>) {
    // Check if the command is valid
    // Check if the command has the correct number of arguments
    // Check if the command arguments are valid
    println!("Command: {:?}", args);
    if args.len() < 2 {
        println!("Please pass arguments to the command.");
        return;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    verify_command_arguments(&args);
}
