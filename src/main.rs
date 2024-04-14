use chrono::Local;
use rusqlite::{Connection, Result};
use std::io::{self};

fn main() -> Result<()> {
    let conn = Connection::open("todos.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo_list (
            id INTEGER PRIMARY KEY,
            todo TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    loop {
        println!("Type 'l' to list all todos, or 'a' to add a new todo:");
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        match action.trim() {
            "l" => {
                let mut stmt = conn.prepare("SELECT todo, created_at FROM todo_list")?;
                let todo_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

                for todo in todo_iter {
                    let (todo, created_at): (String, String) = todo?;
                    println!("{} - created at {}", todo, created_at);
                }
            }
            "a" => {
                println!("Enter your todo:");
                let mut new_todo = String::new();
                io::stdin()
                    .read_line(&mut new_todo)
                    .expect("Failed to read line");
                let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

                conn.execute(
                    "INSERT INTO todo_list (todo, created_at) VALUES (?1, ?2)",
                    &[new_todo.trim(), &created_at],
                )?;
                println!("Todo added.");
            }
            _ => println!("Invalid option, please type 'l' to list or 'a' to add."),
        }
        println!(); // Print a new line for better readability in the CLI
    }

    Ok(())
}
