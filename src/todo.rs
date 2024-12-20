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

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }

    pub fn print(&self) {
        if self.completed 
        { println!("-[X] {}: {}", self.name, self.description); } 
        else              
        { println!("-[ ] {}: {}", self.name, self.description); }
    }
}


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

    pub fn print(&self) {
        for todo in &self.todos {
            todo.print();
        }
    }

    pub fn remove_todo(&mut self, index: usize) {
        self.todos.remove(index);
    }
}