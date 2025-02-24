/** This module containes the methods used to create, access, add to, and
    remove from the database. Each method is public and requires a connection
    to an active rusqlite::Connection object in order to modify the database.
    */

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
            hourly_rate INTEGER,
            applied INTEGER NOT NULL CHECK (applied IN (0, 1))
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
pub fn enter_data(connection: &rusqlite::Connection, a_job: &job::Job) -> Result<(), rusqlite::Error> {
    connection.execute(
        "INSERT INTO jobs (job_title, hourly_rate, applied) VALUES (?1, ?2, ?3)",
        rusqlite::params![a_job.get_title(), a_job.get_hourly(), a_job.get_applied()], // Proper parameter format
    )?;
    Ok(())
}

/// Retrieves all job records from the `jobs` table and prints them.
///
/// # Arguments
/// * `connection` - A reference to an active SQLite connection.
///
/// # Returns
/// * `Ok(())` if the query executes successfully.
/// * `Err(rusqlite::Error)` if an error occurs.
pub fn get_data(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    let mut statement = connection.prepare("SELECT id, job_title, hourly_rate, applied FROM jobs")?;
    
    let job_iterator = statement.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,       // id (INTEGER)
            row.get::<_, String>(1)?,    // job_title (TEXT)
            row.get::<_, i32>(2)?,       // hourly_rate (INTEGER, should not be String)
            row.get::<_, i32>(3)?        // applied (INTEGER)
        ))
    })?;

    for job in job_iterator {
        let (id, job_title, hourly_rate, applied) = job?;
        println!(
            "Job {} - Title: {}, Rate: ${}/hr, Applied: {}",
            id,
            job_title,
            hourly_rate,
            if applied == 1 { "Yes" } else { "No" }  // Convert 1/0 to Yes/No
        );
    }

    Ok(())
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
pub fn drop_table(connection: &rusqlite::Connection, table_name: &str) -> Result<(), rusqlite::Error> {
    let query = format!("DROP TABLE IF EXISTS {}", table_name);
    connection.execute(&query, [])?;
    println!("Table '{}' has been dropped.", table_name);
    Ok(())
}
