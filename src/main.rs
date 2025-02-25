// Author: Nathan Metens
// Class: Rust 523
// Professor: Bart Massey

mod application; // References application.rs file
mod csv_reader;
mod job; // References job.rs file
mod list; // For saving list data to a file // For reading csv file
use crate::csv_reader::read_csv_file;
mod database_methods;

use rusqlite::{Connection, Result};

/** Main method where an application is created, then a table inside a database,
where jobs are poppulated into tables and stored in the database. */
fn main() -> Result<()> {
    let mut apps = application::Applications::new();

    // Job application database file:
    let database_file: &str = "jobs_data.db";

    // Create an SQLite database file. Open the database
    // file if it already exists.
    let connection = Connection::open(database_file)?;

    let _ = database_methods::drop_table(&connection, "jobs"); // Remove the table each time for testing.

    // Create a table:
    if let Err(e) = database_methods::create_table(&connection) {
        eprintln!("Error: {}", e);
    }

    // Read the data inside the csv file of jobs and
    // save the jobs in the app object:
    let csv_file: &str = "application.csv";
    if let Err(e) = read_csv_file(csv_file, &mut apps) {
        eprintln!("Error: {}", e);
    }

    // Enter the jobs in the application to the database:
    for job in apps.get_jobs() {
        if let Err(e) = database_methods::enter_data(&connection, &job) {
            eprintln!("Error: {}", e);
        }
    }

    if let Err(e) = database_methods::get_data(&connection) {
        eprintln!("Error: {}", e);
    }

    Ok(())
}
