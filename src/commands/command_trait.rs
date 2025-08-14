use super::ToDoFile;


pub trait Command {
    fn execute(&self, todo_file: &ToDoFile) -> ();
}