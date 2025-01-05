#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{
    env,
    io::prelude::*,
    fs,
};

mod todo;

use todo::{ ToDo, ToDoList };

fn verify_command_arguments(args: &Vec<String>) {
    // Check if the command is valid
    // Check if the command has the correct number of arguments
    // Check if the command arguments are valid
    if args.len() < 2 {
        println!("Please pass arguments to the command.");
        return;
    }
}


fn main() -> std::io::Result<()> {

    //limpar o terminal antes de mostrar o resultado do comando
    clearscreen::clear().expect("failed to clear screen");

    let args: Vec<String> = env::args().collect();
    verify_command_arguments(&args);

    let todo_file = todo::ToDoFile::new(String::from("./todo.json"));

    match args[1].as_str() {
        "init" => match todo_file.init() {
            Ok(_) => println!("Todo file created successfully"),
            Err(_) => println!("Todo file already exists"),
        },

        "add" => match todo_file.load() {
            Ok(todo) => {
                let mut todo_list = todo;
                todo_list.add_todo(ToDo::new(args[2].clone(), args[3].clone()));

                todo_list.print_all_todos();
                let _ = todo_file.save(&todo_list);
            },
            Err(e) => println!("Todo file not found: {:?}", e),
        },
        
        "toggle" => match todo_file.load() {
            Ok(todo) => {
                let mut todo_list = todo;
                todo_list.toggle_todo(args[2].parse::<usize>().unwrap() - 1);

                todo_list.print_all_todos();
                let _ = todo_file.save(&todo_list);
            },
            Err(e) => println!("Todo file not found: {:?}", e),
        },
        
        "remove" => match todo_file.load() {
            Ok(todo) => {
                let mut todo_list = todo;
                todo_list.remove_todo(args[2].parse::<usize>().unwrap());

                todo_list.print_all_todos();
                let _ = todo_file.save(&todo_list);
            },
            Err(e) => println!("Todo file not found: {:?}", e),
        },

        "reset" => match todo_file.load() {
            Ok(todo) => {
                let mut todo_list = todo;
                todo_list.reset();

                todo_list.print_all_todos();
                let _ = todo_file.save(&todo_list);
            },
            Err(_) => println!("Todo file not found"),
        },
        
        "show" => match todo_file.load() {
            Ok(todo) => {
                if args.len() == 2 {
                    todo.print_all_todos();
                    return Ok(());
                }
                match args[2].as_str() {
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
        },

        "help" => {
            println!("List of commands:
                init                         - create a new todo file
                add     'name' 'description' - add a new todo
                toggle  'index'              - mark a todo as completed
                remove  'index'              - remove a todo
                reset                        - reset the todo file (mark all todos as incomplete)
                show                         - show all todos
                |---show -m                  - show completed todos
                |---show -u                  - show incomplete todos
                |---show -mu                 - show completed todos before incomplete todos
                ----show -um                 - show incomplete todos before completed todos
                help                         - show this help message
            ");
        },

        _ => println!("Invalid command, use 'help' to see the list of commands"),
    }

    Ok(())
}
