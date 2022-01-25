use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: i32,
    title: String,
    completed: bool,
    removed: bool,
}

struct State {
    todos: Vec<Task>,
}

impl State {
    fn new(source: &str) {}

    fn add(title: &str) {}

    fn remove() {}

    fn render() {}

    fn update() {}
}

fn print_usage() {
    let command_width = 7;

    println!("Usage: todo <action> <args>\n");
    println!("[Actions]");
    println!("  {:w$} - Add a new task. The argument should be the task description", "add", w = command_width);
    println!("  {:w$} - Remove a task. Argument is task id. Get task id by running 'todo view'", "remove", w = command_width);
    println!("  {:w$} - View all available tasks.", "view", w = command_width);
    println!("  {:w$} - Update a todo item. Example 'todo update 12 -t \"new title\" -c true|false'", "update", w = command_width)
}

fn main() {
    print_usage();
}
