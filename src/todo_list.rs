use serde::{Deserialize, Serialize};


use crate::todo::ToDo;


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ToDoList {
    daltonic_mode: bool,
    todos: Vec<ToDo>,
}


impl ToDoList {
    pub fn total_tasks(&self) -> usize {
        self.todos.len()
    }

    pub fn completed_tasks(&self) -> usize {
        self.todos.iter().filter(|todo| todo.is_marked()).count()
    }

    
    pub fn set_daltonic_mode(&mut self, daltonic_mode: bool) {
        self.daltonic_mode = daltonic_mode;
    }
    
    
    pub fn  add_new_todo(&mut self, todo: ToDo) {
        self.todos.push(todo);
    }


    pub fn remove_todo(&mut self, index: usize) -> Result<(), &'static str> {
        if index <= 0 || index > self.todos.len() {
            return Err("Error: Index out of bounds");
        }
        self.todos.remove(index-1);
        Ok(())
    }


    pub fn clear(&mut self) {
        self.todos.clear();
    }

    // Modifying functions

    pub fn swap_todos(&mut self, index1: usize, index2: usize) -> Result<(), &'static str> {
        if 0 >= index1 || index1 > self.todos.len()  {
            return Err("Index1 out of bounds");
        }
        if 0 >= index2 || index2 > self.todos.len() {
            return Err("Index2 out of bounds");
        }
        self.todos.swap(index1-1, index2-1);
        Ok(())
    }


    pub fn change_todo_name(&mut self, index: usize, name: String) -> Result<(), &'static str>{
        if let Some(todo) = self.todos.get_mut(index-1) {
            todo.set_name(name);
            Ok(())
        } else {
            Err("Todo not found")
        }
    }


    pub fn change_todo_description(&mut self, index: usize, description: String) -> Result<(), &'static str>{
        if let Some(todo) = self.todos.get_mut(index-1) {
            todo.set_description(description);
            Ok(())
        } else {
            Err("Todo not found")
        }
    }


    pub fn toggle_todo(&mut self, index: usize) -> Result<(), &'static str> {
        if let Some(todo) = self.todos.get_mut(index-1) {
            todo.toggle_mark();
            Ok(())
        } else {
            Err("Todo not found")
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
        println!("All todos ({}/{}):", self.completed_tasks(), self.total_tasks());
        for (index, todo) in self.todos.iter().enumerate() {
            todo.print(index_len, index+1, self.daltonic_mode);
        }
    }


    pub fn print_completed_todos(&self) {
        let index_len = (self.todos.len() as f64 + 0.1).log10().ceil() as usize;
        println!("Completed todos ({}/{}):", self.completed_tasks(), self.total_tasks());
        for (index, todo) in self.todos.iter().enumerate() {
            if todo.is_marked() {
                todo.print(index_len, index+1,self.daltonic_mode);
            }
        }
    }


    pub fn print_incomplete_todos(&self) {
        let index_len = (self.todos.len() as f64 + 0.1).log10().ceil() as usize;
        let total = self.total_tasks();
        let incompleted = total - self.completed_tasks();
        println!("Incomplete todos ({}/{}):", incompleted, total);
        for (index, todo) in self.todos.iter().enumerate() {
            if !todo.is_marked() {
                todo.print(index_len, index+1, self.daltonic_mode);
            }
        }
    }
}
