use std::io::Read;
use std::io::Write;
use std::fs;
use serde_json::{Value};


use crate::todo::ToDo;
use crate::todo_list::ToDoList;


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
                    todo.toggle_mark();
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