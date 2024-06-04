use std::{fs::read, io::Write};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Get {
        #[arg(short, long)]
        index: u16,
    },
    Remove {
        #[arg(short, long)]
        index: u16,
    },
    Create {
        #[arg(short, long)]
        text: String,
    },
    MakeDone {
        #[arg(short, long)]
        index: u16,
    },
    GetAll,
    RemoveAll,
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    text: String,
    index: u16,
    done: bool,
}
#[derive(Debug, Serialize, Deserialize)]
struct Todos {
    todos: Vec<Todo>,
}
fn get_dir_path() -> String {
    format!("/home/{}/.rtd/", whoami::username())
}
fn get_json_path() -> String {
    format!("{}todos.json", get_dir_path())
}
fn check_dir() -> anyhow::Result<()> {
    let username = whoami::username();
    std::fs::create_dir(format!("/home/{}/.rtd/", &username))
        .map_err(|err| eprintln!("error {err}"))
        .unwrap();
    std::fs::File::create(format!("/home/{}/.rtd/todos.json", &username))
        .map_err(|err| eprintln!("error {err}"))
        .unwrap();
    Ok(())
}
fn read_json() -> Todos {
    let _ = check_dir();
    let file = std::fs::File::open(get_json_path())
        .map_err(|err| eprintln!("error {err}"))
        .unwrap();
    let todos: Todos = serde_json::from_reader(file)
        .map_err(|err| eprintln!("error {err}"))
        .unwrap();
    todos
}
fn write_json(todos: &Todos) {
    let _ = check_dir();
    let file = std::fs::File::create(get_json_path())
        .map_err(|err| eprintln!("error {err}"))
        .unwrap();
    let _ = serde_json::to_writer(file, todos).map_err(|err| eprintln!("error {err}"));
}

fn main() {
    let args = Args::parse();
    let todos = read_json();
    if let Some(action) = args.action {
        match action {
            Commands::GetAll => {
                for (i, todo) in todos.todos.iter().enumerate() {
                    println!("{}. {}", i, &todo.text)
                }
            }
            _ => {}
        }
    }
    write_json(&todos)
}
