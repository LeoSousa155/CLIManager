use super::ToDoFile;
use super::Command;


pub struct HelpCommand {}


impl Command for HelpCommand {
    fn execute(&self, _todo_file: &ToDoFile) {
        println!("
            List of commands:
            init                            - create a new todo file
            add     'name' 'description'    - add a new todo
            toggle  'index'                 - mark a todo as completed
            remove  'index'                 - remove a todo
            reset                           - reset the todo file (mark all todos as incomplete)
            show                            - show all todos
            |---show -m                     - show completed todos
            |---show -u                     - show incomplete todos
            |---show -mu                    - show completed todos before incomplete todos
            edit    'index' 'field' 'value' - edit a todo (field should be 'name, n, description or d)
            daltonic 'true/false'           - set daltonic mode
            help                            - show this help message
        ");
    }
}