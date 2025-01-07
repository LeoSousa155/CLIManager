use serde::{Deserialize, Serialize};


use crate::todo::ToDo;


#[derive(Serialize, Deserialize, Debug)]
pub struct ToDoList {
    todos: Vec<ToDo>,
}


impl ToDoList {
    pub fn new() -> ToDoList {
        ToDoList { todos: Vec::new() }
    }


    pub fn  add_todo(&mut self, todo: ToDo) {
        self.todos.push(todo);
    }


    pub fn remove_todo(&mut self, index: usize) {
        if 0 < index && index <= self.todos.len() {
            self.todos.remove(index-1);
        } else {
            println!("Error: Index out of bounds");
        }
    }

    // Modifying functions

    pub fn change_todo_name(&mut self, index: usize, name: String) {
        match self.todos.get_mut(index) {
            Some(todo) => todo.set_name(name),
            None => println!("Todo not found"),
        }
    }


    pub fn change_todo_description(&mut self, index: usize, description: String) {
        match self.todos.get_mut(index) {
            Some(todo) => todo.set_description(description),
            None => println!("Todo not found"),
        }
    }


    pub fn toggle_todo(&mut self, index: usize) {
        match self.todos.get_mut(index) {
            Some(todo) => todo.toggle_mark(),
            None => println!("Todo not found"),
        }
    }


    pub fn reset(&mut self) {
        for todo in &mut self.todos {
            todo.set_unmarked();
        }
    }

    //Printing functions

    pub fn print_all_todos(&self) {
        let index_len = (self.todos.len() as f64 + 0.1).log10().ceil() as usize;
        
        println!("All todos:");
        for (index, todo) in self.todos.iter().enumerate() {
            todo.print(index_len, index+1);
        }
    }


    pub fn print_completed_todos(&self) {
        let index_len = (self.todos.len() as f64 + 0.1).log10().ceil() as usize;

        println!("Completed todos:");
        for (index, todo) in self.todos.iter().enumerate() {
            if todo.is_marked() {
                todo.print(index_len, index+1);
            }
        }
    }


    pub fn print_incomplete_todos(&self) {
        let index_len = (self.todos.len() as f64 + 0.1).log10().ceil() as usize;

        println!("Incomplete todos:");
        for (index, todo) in self.todos.iter().enumerate() {
            if !todo.is_marked() {
                todo.print(index_len, index+1);
            }
        }
    }
}
