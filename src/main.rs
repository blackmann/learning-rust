use std::fs::File;
use std::io::{Error, Read, Write};
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
        /// The ID of the task. Use `view` to check IDs
        id: i32,
        /// The new title
        #[clap(short = 't')]
        title: Option<String>,
        /// The new status
        #[clap(arg_enum, short = 's')]
        status: Option<TaskStatus>,
    },
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Copy, ArgEnum, Debug)]
enum TaskStatus { Completed, Pending, Removed, All }


#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: i32,
    title: String,
    status: TaskStatus,
}

#[derive(Debug)]
struct State {
    source: String,
    todos: Vec<Task>,
}

impl State {
    fn new(source: &str) -> Result<State, Error> {
        let mut file = File::options().read(true).create(true).write(true).open(source)?;
        let mut content: String = String::new();
        file.read_to_string(&mut content)?;

        // it's a new day
        if content.len() == 0 {
            return Ok(State {
                source: String::from(source),
                todos: Vec::new(),
            });
        }

        let todos: Vec<Task> = serde_json::from_str(&content).unwrap();

        Ok(State {
            source: String::from(source),
            todos,
        })
    }

    fn add(&mut self, title: String) {
        self.todos.insert(self.todos.len(), Task {
            id: match self.todos.last() {
                Some(last) => last.id + 1,
                None => 1
            },
            title,
            status: TaskStatus::Pending,
        });

        self.save();
    }

    fn remove(&mut self, id: i32) {
        let todo = self.todos.iter_mut().find(|task| { task.id == id });

        match todo {
            Some(found) => { found.status = TaskStatus::Removed }
            None => ()
        }

        self.save();
    }

    fn update(&mut self, id: i32, title: Option<String>, status: Option<TaskStatus>) {
        let todo = self.todos.iter_mut().find(|task| { task.id == id });

        match todo {
            Some(found) => {
                found.title = title.unwrap_or((&found.title).to_string());
                found.status = status.unwrap_or(found.status);
            }

            None => ()
        }

        self.save();
    }

    fn render(&self, status: Option<TaskStatus>) {
        println!("[S]   ID TASK");
        println!("-------------------------------------------------");
        for todo in &self.todos {
            if todo.status != TaskStatus::Removed {
                println!("[{}] {:>4} {}",
                         if todo.status == TaskStatus::Completed { "x" } else { " " },
                         todo.id,
                         todo.title)
            }
        }

        println!(" ^ ");
        println!(" | ");
        println!(" Status: [x] = complete. Run `todo -h` to know how to update.");
    }

    fn save(&self) -> Result<(), Error> {
        let serialized: String = serde_json::to_string(&self.todos).unwrap();

        let mut file = File::options().write(true)
            .truncate(true).create(true).open(&self.source)?;
        file.write_all(serialized.as_bytes())?;

        println!("Saved ✅");

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let cli: Cli = Cli::parse();

    let mut state = State::new("todos.json")?;

    match cli.command {
        Commands::Add { title } => state.add(title),
        Commands::Remove { id } => state.remove(id),
        Commands::View { status } => state.render(status),
        Commands::Update { id, title, status } => {
            state.update(id, title, status)
        }
    }

    Ok(())
}
