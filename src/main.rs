mod job; // References job.rs file
mod application; // References application.rs file
mod list; // For saving list data to a file

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // Job application database file:
    let database_file: &str = "jobs_data.db";
    
    // Create an SQLite database file. Open the database
    // file if it already exists.
    let connection = Connection::open(database_file)?;

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
