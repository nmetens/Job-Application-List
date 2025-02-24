mod job; // References job.rs file
mod application; // References application.rs file
mod list; // For saving list data to a file
mod csv_reader; // For reading csv file
use crate::csv_reader::read_csv_file;

use rusqlite::{Connection, Result};

/** SQL Method that creates a new table with the values
    of the struct passed in. */
fn create_table(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
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
fn enter_data(connection: &rusqlite::Connection, a_job: job::Job) -> Result<(), rusqlite::Error> {
    // Enter data into database:
    connection.execute(
        "INSERT INTO ?table (job_title, hourly_rate, applied) VALUES (?1, ?2, ?3)",
        (a_job.get_title(), a_job.get_hourly(), a_job.get_applied()),
    )?;
    Ok(())
}
fn get_data(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    let mut statement = connection.prepare("SELECT id, job_title, hourly_rate, applied FROM jobs")?;
    let job_iterater = statement.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, i32>(3)?))
    })?;

    for job in job_iterater {
        let (id, job_title, hourly_rate, applied) = job?;
        println!("Job {} - Title: {}, Rate: {}, Applied: {}", id, job_title, hourly_rate, applied);
    }

    Ok(())
}

/** Main method where an application is created, then a table inside a database,
    where jobs are poppulated into tables and stored in the database. */
fn main() -> Result<()> {
    let mut apps = application::Applications::new();

    // Job application database file:
    let database_file: &str = "jobs_data.db";
    
    // Create an SQLite database file. Open the database
    // file if it already exists.
    let connection = Connection::open(database_file)?;

    // Create a table:
    if let Err(e) = create_table(&connection) {
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
        if let Err(e) = enter_data(&connection, job) {
            eprintln!("Error: {}", e);
        }
    }

    if let Err(e) = get_data(&connection) {
        eprintln!("Error: {}", e);
    }

    Ok(())
}
