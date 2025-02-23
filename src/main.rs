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
    if let Err(e) = read_csv_file(csv_file, &apps) {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

/*#[tokio::main] // Requires the `attributes` feature of `async-std`
async fn main() -> Result<(), sqlx::Error> {
    let mut apps = application::Applications::new();
    apps.add_job();
    apps.view_apps();

    // Output all jobs to a file in 'output' directory:
    let _ = list::print_list(apps); 

    Ok(())
}*/
