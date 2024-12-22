use std::io::Read;
use std::io::Write;
use std::fs;
use serde_json::{Value};
use serde::{Deserialize, Serialize};


trait Colorize {
    fn green(&self) -> String;
    fn red(&self) -> String;
}

impl Colorize for String {
    fn green(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", self)
    }

    fn red(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", self)
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    name: String,
    description: String,
    completed: bool,
}

impl ToDo {
    pub fn new(name: String, description: String) -> ToDo {
        ToDo {
            name,
            description,
            completed: false,
        }
    }
    
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }

    pub fn print(&self, size: usize, index: usize) {
        let formatted_index = format!("{:>size$}", index, size=size);
        if self.completed 
        { println!("{}-[X] {} : {}", formatted_index, self.name.green(), self.description); } 
        else              
        { println!("{}-[ ] {} : {}", formatted_index, self.name.red(), self.description); }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDoList {
    todos: Vec<ToDo>,
}

impl ToDoList {
    fn new() -> ToDoList {
        ToDoList { todos: Vec::new() }
    }

    pub fn  add_todo(&mut self, todo: ToDo) {
        self.todos.push(todo);
    }

    pub fn remove_todo(&mut self, index: usize) {
        self.todos.remove(index);
    }

    // Modifying functions

    pub fn change_todo_name(&mut self, index: usize, name: String) {
        self.todos[index].set_name(name);
    }

    pub fn change_todo_description(&mut self, index: usize, description: String) {
        self.todos[index].set_description(description);
    }

    pub fn toggle_todo(&mut self, index: usize) -> bool{
        self.todos[index].toggle_completed();
        println!("completed: {}", self.todos[index].completed);
        return self.todos[index].completed;
    }

    //Printing functions

    pub fn print_all_todos(&self) {
        let index_len = (self.todos.len() as f64 + 0.1).log10().ceil() as usize;

        for (index, todo) in self.todos.iter().enumerate() {
            todo.print(index_len, index+1);
        }
    }

    pub fn print_completed_todos(&self) {
        let index_len = (self.todos.len() as f64 + 0.1).log10().ceil() as usize;

        for (index, todo) in self.todos.iter().enumerate() {
            if todo.completed {
                todo.print(index_len, index+1);
            }
        }
    }

    pub fn print_incomplete_todos(&self) {
        let index_len = (self.todos.len() as f64 + 0.1).log10().ceil() as usize;

        for (index, todo) in self.todos.iter().enumerate() {
            if !todo.completed {
                todo.print(index_len, index+1);
            }
        }
    }
}

pub struct ToDoFile {
    file_name: String
}

impl ToDoFile {
    pub fn new(file_name: String) -> ToDoFile {
        ToDoFile { file_name }
    }

    pub fn init(&self) -> Result<(), std::io::Error> {
        let mut file = fs::File::create_new(&self.file_name)?;
        file.write_all(b"{}")?;
        return Ok(());
    }

    pub fn load(&self) -> Result<ToDoList, Box<dyn std::error::Error>> {
        let mut file = fs::File::open(&self.file_name)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
    
        // Desserializar o JSON para um Value
        let todos_values: Value = serde_json::from_str(&buffer)?;
    
        // Inicializar uma nova lista de tarefas
        let mut todos = ToDoList::new();
    
        // Garantir que o Value é um array e iterar sobre ele
        if let Some(array) = todos_values["todos"].as_array() {
            for todo in array {
                // Ler os campos do JSON
                let name = todo["name"].as_str().unwrap_or("").to_string();
                let description = todo["description"].as_str().unwrap_or("").to_string();
                let completed = todo["completed"].as_bool().unwrap_or(false);
    
                // Criar uma nova tarefa e ajustá-la conforme necessário
                let mut todo = ToDo::new(name, description);
                if completed {
                    todo.toggle_completed();
                }
                // Adicionar a tarefa à lista
                todos.add_todo(todo);
            }
        }
    
        Ok(todos)
    }
    

    pub fn save(&self, todos: &ToDoList) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string_pretty(todos)?;
        let mut file = fs::File::create(&self.file_name)?;
        file.write_all(serialized.as_bytes())?;
        return Ok(());
    }
}