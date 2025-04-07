# CLIManager: Your Command-Line Task Manager

CLIManager is a simple yet powerful command-line application designed to help you manage your daily tasks efficiently. With CLIManager, you can easily add, track, edit, and organize your to-dos directly from your terminal.

## Features

*   **Task Management:** Add, complete, remove, and edit tasks with ease.
*   **Persistence:** Your tasks are saved in a `todo.json` file, ensuring they persist across sessions.
*   **Organization:** Organize your tasks with features like toggling completion status, clearing all tasks, and resetting the list.
*   **Filtering:** View completed, incomplete, or all tasks.
*   **Reordering:** Swap the position of tasks in your list.
*   **Customization:** Enable a daltonic mode for improved accessibility.
*   **User-Friendly:** Clear and concise command-line interface.

## Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/LeoSousa155/CLIManager
    cd CLIManager
    ```
    
2.  **Build the project:**
    ```bash
    cargo build --release
    ```

3.  **Run the application:**
    ```bash
    ./target/release/CLIManager
    ```
    (Or add the path to your `PATH` environment variable to run it from anywhere)

## Usage

CLIManager is controlled through a set of simple commands. Here's a breakdown of each command and its usage:

### Initialization

*   **`init`:** Creates a new `todo.json` file in the current directory to store your tasks.
    ```bash
    ./target/release/CLIManager init
    ```
    **Note:** If the file already exists, it will print a message indicating that.

### Adding Tasks

*   **`add 'name' 'description'`:** Adds a new task to your to-do list.
    *   `'name'`: The name of the task (e.g., "Grocery Shopping").
    *   `'description'`: A brief description of the task (e.g., "Buy milk, eggs, and bread").
    ```bash
    ./target/release/CLIManager add "Grocery Shopping" "Buy milk, eggs, and bread"
    ```

### Toggling Task Completion

*   **`toggle 'index'`:** Marks a task as complete or incomplete.
    *   `'index'`: The index number of the task in the list (starting from 0).
    ```bash
    ./target/release/CLIManager toggle 0
    ```

### Removing Tasks

*   **`remove 'index'`:** Removes a task from the list.
    *   `'index'`: The index number of the task to remove.
    ```bash
    ./target/release/CLIManager remove 1
    ```

### Editing Tasks

*   **`edit 'index' 'field' 'value'`:** Edits the name or description of a task.
    *   `'index'`: The index number of the task to edit.
    *   `'field'`: The field to edit, either `name` (or `n`) or `description` (or `d`).
    *   `'value'`: The new value for the field.
    ```bash
    ./target/release/CLIManager edit 0 name "Updated Task Name"
    ./target/release/CLIManager edit 1 d "New description for the task"
    ```

### Clearing All Tasks

*   **`clear`:** Removes all tasks from the list.
    ```bash
    ./target/release/CLIManager clear
    ```

### Swapping Tasks

*   **`swap 'index1' 'index2'`:** Swaps the positions of two tasks in the list.
    *   `'index1'`: The index of the first task.
    *   `'index2'`: The index of the second task.
    ```bash
    ./target/release/CLIManager swap 0 2
    ```

### Resetting Tasks

*   **`reset`:** Marks all tasks as incomplete.
    ```bash
    ./target/release/CLIManager reset
    ```

### Showing Tasks

*   **`show`:** Displays all tasks in the list.
    ```bash
    ./target/release/CLIManager show
    ```
*   **`show -m`:** Displays only completed tasks.
    ```bash
    ./target/release/CLIManager show -m
    ```
*   **`show -u`:** Displays only incomplete tasks.
    ```bash
    ./target/release/CLIManager show -u
    ```
*   **`show -mu`:** Displays completed tasks followed by incomplete tasks.
    ```bash
    ./target/release/CLIManager show -mu
    ```
*   **`show -um`:** Displays incomplete tasks followed by completed tasks.
    ```bash
    ./target/release/CLIManager show -um
    ```

### Daltonic Mode

*   **`daltonic 'true/false'`:** Enables or disables daltonic mode. This mode might change the way tasks are displayed to improve accessibility for users with color vision deficiencies.
    ```bash
    ./target/release/CLIManager daltonic true
    ./target/release/CLIManager daltonic false
    ```

### Help

*   **`help`:** Displays a list of available commands and their descriptions.
    ```bash
    ./target/release/CLIManager help
    ```

## Contributing

If you'd like to contribute to CLIManager, please feel free to fork the repository and submit a pull request.

## License

This project is licensed under the MIT License.

## Author

Leonardo Sousa - [Linkedin](https://www.linkedin.com/in/leonardo-sousa-8a7304262/)