// src/db.rs

use chrono::Local;
use rusqlite::{params, Connection, Result};

pub struct TodoDb {
    conn: Connection,
}
impl TodoDb {
    // Method to establish a connection to the SQLite database
    pub fn new() -> Result<Self> {
        let conn = Connection::open("todos.db")?;
        Ok(TodoDb { conn })
    }

    // Method to create the todos table in the database
    pub fn create_todos_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS todo_list (
                id INTEGER PRIMARY KEY,
                todo TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    // Method to insert a new todo into the database
    pub fn insert_todo(&self, todo: &str) -> Result<()> {
        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.conn.execute(
            "INSERT INTO todo_list (todo, created_at) VALUES (?1, ?2)",
            params![todo, &created_at],
        )?;
        Ok(())
    }

    // Method to query todos from the database
    pub fn query_todos(&self) -> Result<Vec<(String, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT todo, created_at FROM todo_list")?;
        let todo_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo?);
        }
        Ok(todos)
    }
}
