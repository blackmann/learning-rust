use clap::{ArgEnum, Parser, Subcommand};
use serde::{Serialize, Deserialize};

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
    Add { title: String },

    /// Remove a task
    Remove { id: i32 },

    /// View all saved tasks.
    View {
        #[clap(arg_enum)]
        status: Option<TaskStatus>
    },

    /// Update properties of a task
    Update {
        id: i32,
        title: Option<String>,
        #[clap(arg_enum)]
        status: Option<TaskStatus>,
    },
}

#[derive(Clone, Serialize, Deserialize, ArgEnum, Debug)]
enum TaskStatus { Completed, Pending, Removed, All }


#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: i32,
    title: String,
    status: TaskStatus,
}

struct State {
    todos: Vec<Task>,
}

impl State {
    fn new(source: &str) {}

    fn add(&self, title: String) {}

    fn remove(&self, id: i32) {}

    fn render(&self, status: Option<TaskStatus>) {}

    fn update(&self, id: i32, title: Option<String>, status: Option<TaskStatus>) {}
}

fn main() {
    let cli: Cli = Cli::parse();

    let state = State {
        todos: Vec::new()
    };

    match cli.command {
        Commands::Add { title } => state.add(title),
        Commands::Remove { id } => state.remove(id),
        Commands::View { status } => state.render(status),
        Commands::Update { id, title, status } => {
            state.update(id, title, status)
        }
    }
}
