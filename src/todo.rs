use serde::{Deserialize, Serialize};


trait Colorize {
    fn green(&self) -> String;
    fn red(&self) -> String;
    fn blue(&self) -> String;
    fn orange(&self) -> String;
}


impl Colorize for String {
    fn green(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", self)
    }

    fn red(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", self)
    }

    fn blue(&self) -> String {
        format!("\x1b[34m{}\x1b[0m", self)
    }

    fn orange(&self) -> String {
        format!("\x1b[33m{}\x1b[0m", self)
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
    

    pub fn set_name(&mut self, name: String) { self.name = name; }
    pub fn set_description(&mut self, description: String) { self.description = description; }

    pub fn is_marked(&self) -> bool { self.completed}

    // change completion state 
    pub fn toggle_mark(&mut self) { self.completed = !self.completed; }
    pub fn set_marked(&mut self) { self.completed = true; }
    pub fn set_unmarked(&mut self) { self.completed = false; }

    pub fn print(&self, size: usize, index: usize, daltonic: Option<bool>) {
        let formatted_index = format!("{:>size$}", index, size=size);

        if daltonic.unwrap_or(false) {
            if self.completed { 
                println!("{}-[X] {} : {}", formatted_index, self.name.orange(), self.description); 
            } else { 
                println!("{}-[ ] {} : {}", formatted_index, self.name.blue(), self.description); 
            }
        } else {
            if self.completed { 
                println!("{}-[X] {} : {}", formatted_index, self.name.green(), self.description); 
            } else { 
                println!("{}-[ ] {} : {}", formatted_index, self.name.red(), self.description); 
            }
        }
        
    }
}