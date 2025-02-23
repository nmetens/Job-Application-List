mod job; // References job.rs file
mod application; // References application.rs file
mod list; // For saving list data to a file
mod csv_reader; // For reading csv file
use crate::csv_reader::read_csv_file;

use rusqlite::{Connection, Result};

/** SQL Method that creates a new table with the values
    of the struct passed in. */
fn create_table(connection: rusqlite::Connection) -> Result<(), rusqlite::Error> {
    // Create a table of jobs if it doesn't already exist:
    connection.execute(
        "CREATE TABLE IF NOT EXISTS jobs (
            id INTEGER PRIMARY KEY,
            job_title TEXT NOT NULL,
            hourly_rate INTEGER,
            applied BOOLEAN 
         )", 
         (), // Empty list of parameters (just create table).
    )?;
    Ok(())
}

/** Pass in the database connection and the job to add to the database jobs table: */
fn enter_data(connection: rusqlite::Connection, a_job: job::Job) -> Result<(), rusqlite::Error> {
    // Enter data into database:
    connection.execute(
        "INSERT INTO ?table (job_title, hourly_rate, applied) VALUES (?1, ?2, ?3)",
        (a_job.get_title(), a_job.get_hourly(), a_job.get_applied()),
    )?;
    Ok(())
}

/** Main method where an application is created, then a table inside a database,
    where jobs are poppulated into tables and stored in the database. */
fn main() -> Result<()> {
    let mut apps = application::Applications::new();
/*  apps.add_job();
    apps.view_apps();
*/

    // Job application database file:
    let database_file: &str = "jobs_data.db";
    
    // Create an SQLite database file. Open the database
    // file if it already exists.
    let connection = Connection::open(database_file)?;

    let csv_file: &str = "application.csv";
    if let Err(e) = read_csv_file(csv_file, &mut apps) {
        eprintln!("Error: {}", e);
    }

    Ok(())
}
