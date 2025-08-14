use std::io::Read;
use std::io::Write;
use std::fs;

use crate::todo_list::ToDoList;


pub struct ToDoFile {
    file_name: String
}


impl ToDoFile {
    pub fn new(file_name: String) -> ToDoFile {
        ToDoFile { file_name }
    }


    pub fn init(&self) -> Result<(), std::io::Error> {
        let initial_todo_list = ToDoList::default();
        let serialized = serde_json::to_string_pretty(&initial_todo_list)?;
        let mut file = fs::File::create_new(&self.file_name)?;
        file.write_all(serialized.as_bytes())?;
        return Ok(());
    }

    
    pub fn load(&self) -> Result<ToDoList, Box<dyn std::error::Error>> {
        let mut file = fs::File::open(&self.file_name)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let todos = serde_json::from_str(&buffer)?;
        Ok(todos)
    }
    

    pub fn save(&self, todos: &ToDoList) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string_pretty(todos)?;
        let mut file = fs::File::create(&self.file_name)?;
        file.write_all(serialized.as_bytes())?;
        return Ok(());
    }
}