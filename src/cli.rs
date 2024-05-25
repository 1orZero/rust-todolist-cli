use crate::db::TodoDb;
use std::num::ParseIntError;
use std::{error::Error, io}; // Add missing import

pub struct Cli {
    db: TodoDb,
}

impl Cli {
    pub fn new(db: TodoDb) -> Self {
        Cli { db }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.print_task_list()?;
            println!("Type 'a' to add a new todo, 'd' to delete a todo or 'q' to exit:");

            let action = self.read_user_input();

            let should_exit = match action.trim() {
                "a" => {
                    self.add_todo()?;
                    false
                }
                "d" => {
                    self.delete_todo()?;
                    false
                }
                "q" => {
                    self.exit()?;
                    true
                }
                _ => {
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

    fn print_task_list(&self) -> Result<(), rusqlite::Error> {
        println!("====================");
        self.list_todos()?;
        println!("====================");
        println!(); // Print a new line for better readability in the CLI
        Ok(())
    }

    fn read_user_input(&self) -> String {
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
        action.trim().to_string()
    }

    fn list_todos(&self) -> Result<(), rusqlite::Error> {
        let todos = self.db.query_todos()?;
        for (id, todo, created_at) in todos {
            println!("{}. {} - created at {}", id, todo, created_at);
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

    fn delete_todo(&mut self) -> Result<(), rusqlite::Error> {
        println!("Enter the id of the todo you want to delete:");
        let mut id = String::new();
        io::stdin().read_line(&mut id).expect("Failed to read line");
        let id = id
            .trim()
            .parse::<i32>()
            .map_err(|_: ParseIntError| rusqlite::Error::InvalidQuery)?;
        self.db.remove_todo(id)?;
        println!("Todo deleted.");
        Ok(())
    }

    fn exit(&self) -> Result<(), rusqlite::Error> {
        println!("Exiting...");
        Ok(())
    }
}
