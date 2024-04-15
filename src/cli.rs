use crate::db::TodoDb;
use std::{error::Error, io};

pub struct Cli {
    db: TodoDb,
}

impl Cli {
    pub fn new(db: TodoDb) -> Self {
        Cli { db }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            println!("Task List:");
            println!("===================="); // Print a new line for better readability in the CLI
            self.list_todos()?;
            println!("===================="); // Print a new line for better readability in the CLI
            println!(); // Print a new line for better readability in the CLI
            println!("Type 'a' to add a new todo or 'q' to exit:");
            let mut action = String::new();
            io::stdin()
                .read_line(&mut action)
                .expect("Failed to read line");

            let should_exit = match action.trim() {
                "a" => {
                    self.add_todo()?;
                    false
                }
                "q" => {
                    self.exit()?;
                    true
                }
                _ => {
                    println!("Task List:");
                    println!("===================="); // Print a new line for better readability in the CLI
                    self.list_todos()?;
                    println!("===================="); // Print a new line for better readability in the CLI

                    println!(); // Print a new line for better readability in the CLI
                    println!("Invalid option, please type 'a' to add or 'q' to exit.");
                    false
                }
            };
            if should_exit {
                break;
            }
            println!(); // Print a new line for better readability in the CLI
        }
        Ok(())
    }

    fn list_todos(&self) -> Result<(), rusqlite::Error> {
        let todos = self.db.query_todos()?;
        for (todo, created_at) in todos {
            println!("{} - created at {}", todo, created_at);
        }
        Ok(())
    }

    fn add_todo(&mut self) -> Result<bool, rusqlite::Error> {
        println!("Enter your todo:");
        let mut new_todo = String::new();
        io::stdin()
            .read_line(&mut new_todo)
            .expect("Failed to read line");

        self.db.insert_todo(new_todo.trim())?;
        println!("Todo added.");
        Ok(true)
    }

    fn exit(&self) -> Result<(), rusqlite::Error> {
        println!("Exiting...");
        Ok(())
    }
}
