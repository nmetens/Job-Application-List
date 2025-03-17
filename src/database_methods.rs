// Author: Nathan Metens
// Class: Rust 523
// Professor: Bart Massey

//! This module containes the methods used to create, access, add to, and
//! remove from the database. Each method is public and requires a connection
//! to an active rusqlite::Connection object in order to modify the database.

use crate::job;

/// Creates a `jobs` table in the SQLite database if it does not already exist.
///
/// # Arguments
/// * `connection` - A reference to an active `rusqlite::Connection`.
///
/// # Returns
/// * `Ok(())` if the table is successfully created.
/// * `Err(rusqlite::Error)` if an SQL execution error occurs.
pub fn create_table(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS jobs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            job_title TEXT NOT NULL,
            hourly_rate REAL,
            applied INTEGER NOT NULL CHECK (applied IN (0, 1)),
            link TEXT
        )",
        (), // Empty parameters
    )?;
    Ok(())
}

/// Inserts a job into the `jobs` table.
///
/// # Arguments
/// * `connection` - A reference to an active SQLite connection.
/// * `a_job` - A reference to a `Job` struct containing job details.
///
/// # Returns
/// * `Ok(())` if the job was successfully inserted.
/// * `Err(rusqlite::Error)` if an error occurs.
pub fn enter_data(
    connection: &rusqlite::Connection,
    a_job: &job::Job,
) -> Result<(), rusqlite::Error> {
    connection.execute(
        "INSERT INTO jobs (job_title, hourly_rate, applied, link) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![a_job.get_title(), a_job.get_hourly(), a_job.get_applied(), a_job.get_link()], // Proper parameter format
    )?;
    Ok(())
}

/// Remove a job from the `jobs` table.
///
/// # Arguments
/// * `connection` - Reference to the databse.
/// * `id` - The jobs id to be removed.
pub fn remove_data(connection: &rusqlite::Connection, id: i64) -> Result<bool, rusqlite::Error> {
    let result = connection.execute("DELETE FROM jobs WHERE id = ?", rusqlite::params![id])?;

    if result > 0 {
        Ok(true) // Job was deleted
    } else {
        Ok(false) // No job found with that ID.
    }
}

/// Retrieves all job records from the `jobs` table and prints them.
///
/// # Arguments
/// * `connection` - A reference to an active SQLite connection.
///
/// # Returns
/// * `Ok(())` if the query executes successfully.
/// * `Err(rusqlite::Error)` if an error occurs.
use log::info;
use crate::job::Job;
pub fn get_jobs(connection: &rusqlite::Connection) -> Result<Vec<Job>, rusqlite::Error> {
    let mut statement =
        connection.prepare("SELECT id, job_title, hourly_rate, applied, link FROM jobs")?;

    // Iterate through the database and gather all the lines of data, creating the Job:
    let job_iterator = statement.query_map([], |row| {
        let id: i64 = row.get::<_, i64>(0)?;            // id
        let title: String = row.get::<_, String>(1)?;   // title
        let hourly: f32 = row.get::<_, f32>(2)?;        // hourly

        // Properly handle the Result and convert applied value to "Yes" or "No"
        let applied: i64 = row.get::<_, i64>(3)?;       // applied
        let applied_status = if applied == 1 { "Yes".to_string() } else { "No".to_string() };

        // link
        //let link: String = row.get::<_, String>(4).ok().unwrap_or("No Link".to_string());
        let link = row.get::<_, String>(4).map(|s| s.to_string());

        // Return a new Job instance with applied as "Yes"/"No" instead of "1/0":
        Ok(Job::new(Some(id), title, hourly, applied_status, Some(link.expect("No Link"))))
    })?;

    let mut jobs = Vec::new();
    for job in job_iterator {
        match job {
            Ok(j) => jobs.push(j),
            Err(e) => eprintln!("Error parsing job: {}", e),
        }
    }

    info!("Fetched {} jobs", jobs.len());
    Ok(jobs)
}

/// Drop the table_name from the database.
///
/// # Arguments
/// * `connection` - A reference to an active SQLite connection (rusqlite::Connection).
/// * `table_name` - The name of the table that will be dropped.
///
/// # Returns
/// * `Ok(())` if query exists on success.
/// * `Err(rusqlite::Error)` if an error occurs.
pub fn drop_table(
    connection: &rusqlite::Connection,
    table_name: &str,
) -> Result<(), rusqlite::Error> {
    let query = format!("DROP TABLE IF EXISTS {}", table_name);
    connection.execute(&query, [])?;
    println!("Table '{}' has been dropped.", table_name);
    Ok(())
}
