use clap::{ArgEnum, Parser, Subcommand};

// use serde::{Serialize, Deserialize};

const DESCRIPTION: &str = "Create todo list from the terminal. This is research project to learn Rust 🦀.";

#[derive(Parser)]
#[clap(author = "blackmann <mail@degreat.co.uk>", version = "1.0", about = DESCRIPTION, long_about = None)]
struct Cli {
    /// Actions to run
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add new task to the todo
    Add { name: String },

    /// Remove a task
    Remove { id: i32 },

    /// View all saved tasks.
    View {
        #[clap(arg_enum)]
        status: TaskStatus
    },

    /// Update properties of a task
    Update {
        id: i32,
        title: Option<String>,
        #[clap(arg_enum)]
        status: Option<TaskStatus>,
    },
}

#[derive(Clone, ArgEnum)]
enum TaskStatus { Completed, Pending, Removed }


// #[derive(Serialize, Deserialize, Debug)]
// struct Task {
//     id: i32,
//     title: String,
//     completed: bool,
//     removed: bool,
// }
//
// struct State {
//     todos: Vec<Task>,
// }
//
// impl State {
//     fn new(source: &str) {}
//
//     fn add(title: &str) {}
//
//     fn remove() {}
//
//     fn render() {}
//
//     fn update() {}
// }

fn main() {
    let cli = Cli::parse();
}
