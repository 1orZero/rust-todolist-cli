mod db;

use db::TodoDb;
use rusqlite::Result;
use std::io::{self};

fn main() -> Result<()> {
    let db = TodoDb::new()?;

    db.create_todos_table()?;

    loop {
        println!("Type 'l' to list all todos, or 'a' to add a new todo:");
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        match action.trim() {
            "l" => {
                let todos = db.query_todos()?;
                for (todo, created_at) in todos {
                    println!("{} - created at {}", todo, created_at);
                }
            }
            "a" => {
                println!("Enter your todo:");
                let mut new_todo = String::new();
                io::stdin()
                    .read_line(&mut new_todo)
                    .expect("Failed to read line");

                db.insert_todo(new_todo.trim())?;
                println!("Todo added.");
            }
            _ => println!("Invalid option, please type 'l' to list or 'a' to add."),
        }
        println!(); // Print a new line for better readability in the CLI
    }
}
