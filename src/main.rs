mod cli;
mod db;

use std::error::Error;

use cli::Cli;
use db::TodoDb;
use rusqlite::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let db = TodoDb::new()?;
    db.create_todos_table()
        .expect("Failed to create todos table");
    let mut cli = Cli::new(db);
    match cli.run() {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
