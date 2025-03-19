// Author: Nathan Metens
// Class: Rust 523
// Professor: Bart Massey

//! This module containes the methods used to create, access, add to,
//! remove, check if empty, and count for the database. Each method is
//! public and requires a connection to an active rusqlite::Connection
//! object in order to modify the database.

use crate::job;
use crate::job::Job;
use log::info;

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
        rusqlite::params![
            a_job.get_title(),
            a_job.get_hourly(),
            a_job.get_applied(),
            a_job.get_link()
        ], // Proper parameter format
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
pub fn get_jobs(connection: &rusqlite::Connection) -> Result<Vec<Job>, rusqlite::Error> {
    let mut statement =
        connection.prepare("SELECT id, job_title, hourly_rate, applied, link FROM jobs")?;

    // Iterate through the database and gather all the lines of data, creating the Job:
    let job_iterator = statement.query_map([], |row| {
        let id: i64 = row.get::<_, i64>(0)?; // id
        let title: String = row.get::<_, String>(1)?; // title
        let hourly: f32 = row.get::<_, f32>(2)?; // hourly

        // Properly handle the Result and convert applied value to "Yes" or "No"
        let applied: i64 = row.get::<_, i64>(3)?; // applied
        let applied_status = if applied == 1 {
            "Yes".to_string()
        } else {
            "No".to_string()
        };

        // link
        //let link: String = row.get::<_, String>(4).ok().unwrap_or("No Link".to_string());
        let link = row.get::<_, String>(4).map(|s| s.to_string());

        // Return a new Job instance with applied as "Yes"/"No" instead of "1/0":
        Ok(Job::new(
            Some(id),
            title,
            hourly,
            applied_status,
            Some(link.expect("No Link")),
        ))
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
pub fn _drop_table(
    connection: &rusqlite::Connection,
    table_name: &str,
) -> Result<(), rusqlite::Error> {
    let query = format!("DROP TABLE IF EXISTS {}", table_name);
    connection.execute(&query, [])?;
    println!("Table '{}' has been dropped.", table_name);
    Ok(())
}

/// Update the applied status for a job in the database.
///
/// # Arguments
/// * `connection` - A reference to an active SQLite connection (rusqlite::Connection).
/// * `new_status` - A bool to signify yes or no that the application status is changed.
/// * `job_id` - The id of the job being updated.
///
/// # Returns
/// * `Ok(())` if query exists on success.
/// * `Err(rusqlite::Error)` if an error occurs.
pub fn update_applied(
    connection: &rusqlite::Connection,
    new_status: bool,
    job_id: i64,
) -> rusqlite::Result<()> {
    connection.execute(
        "UPDATE jobs SET applied = ? WHERE id = ?",
        (new_status as i32, job_id),
    )?;
    Ok(())
}

/// Count all the rows in the database.
///
/// # Arguments
/// * `connection` - A reference to an active SQLite connection (rusqlite::Connection).
///
/// # Returns
/// * `i64` The total rows counted in the database.
/// * `Err(rusqlite::Error)` if an error occurs.
pub fn count_rows(connection: &rusqlite::Connection) -> Result<i64, rusqlite::Error> {
    // Prepare the query to count rows in the 'jobs' table:
    let mut statement = connection.prepare("SELECT COUNT(*) FROM jobs")?;

    // Execute the query and get the result as a single value (the row count):
    let count: i64 = statement.query_row([], |row| row.get(0))?;

    Ok(count)
}

/// Check if the database is empty.
///
/// # Arguments
/// * `connection` - A reference to an active SQLite connection (rusqlite::Connection).
///
/// # Returns
/// * `bool` The true or false value depending on the total rows in the database. 0 is true, any rows is false.
/// * `Err(rusqlite::Error)` if an error occurs.
pub fn database_empty(connection: &rusqlite::Connection) -> Result<bool, rusqlite::Error> {
    let row_count = count_rows(connection)?;
    Ok(row_count == 0)
}

/// These are all the tests for each function in this module.
/// In each test, a new database connection is created with the
/// database schema and it is then populated and tested against
/// the methods above.
///
/// There are helper methods to reduce the size of the code.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::Job;
    use rusqlite::Connection;

    // Helper function to set up an in-memory database and create the jobs table:
    fn setup_database() -> Connection {
        let connection = Connection::open_in_memory().expect("Failed to open in-memory database");
        create_table(&connection).expect("Failed to create table");
        connection
    }

    // Helper function to insert a job:
    fn insert_job(connection: &Connection, job: &Job) {
        enter_data(&connection, &job).expect("Failed to insert job");
    }

    // Helper function to count the number of jobs in the database:
    fn count_jobs(connection: &Connection) -> i64 {
        count_rows(&connection).expect("Failed to count rows")
    }

    // Create an empty database and check that it returns the Ok().
    // No jobs are added to the table, just creating a new database and a jobs table:
    #[test]
    fn test_create_table() {
        // Create an in-memory database for testing.
        let connection = setup_database();

        // Call the create_table function to create the 'jobs' table.
        let result = create_table(&connection);

        // Ensure the result is Ok, indicating the table creation was successful.
        assert!(result.is_ok(), "Table creation failed: {:?}", result);

        // Now, check if the table was created by attempting to fetch the table's information.
        let query = "
            SELECT name
            FROM sqlite_master
            WHERE type='table' AND name='jobs'
        ";

        let table_exists: Result<String, rusqlite::Error> =
            connection.query_row(query, (), |row| row.get(0)); // Use () for no parameters.

        // Assert that the table 'jobs' exists in the database.
        match table_exists {
            Ok(table_name) => assert_eq!(table_name, "jobs", "Table 'jobs' does not exist."),
            Err(_) => panic!("Table 'jobs' was not created successfully."),
        }
    }

    // Create a database and add a new job into it.
    // Check that the data is as expected:
    #[test]
    fn test_enter_data() {
        // Step 1: Set up an in-memory SQLite database for testing:
        let connection = setup_database();

        // Step 2: Create the table using the create_table method (if not already created):
        create_table(&connection).expect("Failed to create table");

        // Step 3: Create a Job object with sample data:
        let job = Job::new(
            Some(0),
            "Software Engineer".to_string(),
            45.0,
            "1".to_string(),
            Some("https://example.com".to_string()),
        );

        // Step 4: Call the enter_data method to insert the job into the database:
        let result = enter_data(&connection, &job);

        // Step 5: Ensure the insertion was successful:
        assert!(result.is_ok(), "Failed to insert data: {:?}", result);

        // Step 6: Query the database to ensure the job was inserted:
        let query = "
            SELECT job_title, hourly_rate, applied, link
            FROM jobs
            WHERE job_title = ?1
        ";

        let mut stmt = connection.prepare(query).expect("Failed to prepare query");
        let job_iter = stmt
            .query_map(rusqlite::params!["Software Engineer"], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, f32>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })
            .expect("Failed to execute query");

        // Step 7: Validate that the data inserted is correct:
        for job_result in job_iter {
            match job_result {
                Ok((job_title, hourly_rate, applied, link)) => {
                    assert_eq!(job_title, "Software Engineer", "Job title doesn't match.");
                    assert_eq!(hourly_rate, 45.0, "Hourly rate doesn't match.");
                    assert_eq!(applied, 1, "Applied value doesn't match.");
                    assert_eq!(link, "https://example.com", "Link doesn't match.");
                }
                Err(e) => {
                    panic!("Failed to fetch job data: {}", e);
                }
            }
        }
    }

    // Create a new database, add a job, and remove it:
    #[test]
    fn test_remove_data() {
        // Step 1: Set up an in-memory SQLite database for testing:
        let connection = Connection::open_in_memory().expect("Failed to open in-memory database");

        // Step 2: Create the table using the create_table method (if not already created):
        create_table(&connection).expect("Failed to create table");

        // Step 3: Insert a job for testing:
        let job = Job::new(
            Some(0),
            "Software Engineer".to_string(),
            45.0,
            "1".to_string(),
            Some("https://example.com".to_string()),
        );

        // Insert the job into the database:
        //enter_data(&connection, &job).expect("Failed to insert job");
        insert_job(&connection, &job);

        // Step 4: Verify the job was inserted:
        let query = "SELECT COUNT(*) FROM jobs WHERE job_title = ?";
        let count: i64 = connection
            .query_row(query, rusqlite::params!["Software Engineer"], |row| {
                row.get(0)
            })
            .expect("Failed to query job count");

        // Assert that the job is inserted:
        assert_eq!(count, 1, "Job should be in the database.");

        // Step 5: Call remove_data to remove the job by its id (assuming id is 1 here):
        let result = remove_data(&connection, 1);

        // Step 6: Assert the job was removed successfully:
        assert!(result.is_ok(), "Failed to remove job: {:?}", result);
        assert!(result.unwrap(), "Job should be removed.");

        // Step 7: Verify the job is removed from the database:
        let count_after_removal: i64 = connection
            .query_row(query, rusqlite::params!["Software Engineer"], |row| {
                row.get(0)
            })
            .expect("Failed to query job count after removal");

        // Assert that the job is removed:
        assert_eq!(
            count_after_removal, 0,
            "Job should be removed from the database."
        );

        // Step 8: Attempt to remove a non-existent job (id 99999):
        let result_non_existent = remove_data(&connection, 99999);

        // Assert that no job was removed (returns false):
        assert!(
            result_non_existent.is_ok(),
            "Failed to check non-existent job removal"
        );
        assert!(
            !result_non_existent.unwrap(),
            "No job should be removed for non-existent id."
        );
    }

    // Create database, insert jobs, and get the jobs:
    #[test]
    fn test_get_jobs() {
        // Step 1: Set up an in-memory SQLite database for testing:
        let connection = Connection::open_in_memory().expect("Failed to open in-memory database");

        // Step 2: Create the table using the create_table method (if not already created):
        create_table(&connection).expect("Failed to create table");

        // Step 3: Insert some jobs for testing:
        let job1 = Job::new(
            Some(0),
            "Software Engineer".to_string(),
            50.0,
            "1".to_string(),
            Some("https://example1.com".to_string()),
        );

        let job2 = Job::new(
            Some(0),
            "Product Manager".to_string(),
            60.0,
            "0".to_string(),
            Some("https://example2.com".to_string()),
        );

        // Insert jobs into the database:
        enter_data(&connection, &job1).expect("Failed to insert job1");
        enter_data(&connection, &job2).expect("Failed to insert job2");

        // Step 4: Call get_jobs to fetch all jobs from the database:
        let jobs_result = get_jobs(&connection);

        // Step 5: Assert that get_jobs was successful and jobs are returned:
        assert!(
            jobs_result.is_ok(),
            "Failed to fetch jobs: {:?}",
            jobs_result
        );
        let jobs = jobs_result.unwrap();
        assert_eq!(jobs.len(), 2, "There should be 2 jobs in the result.");

        // Step 6: Verify the content of the jobs:
        let job_titles: Vec<String> = jobs.iter().map(|j| j.get_title()).collect();
        assert!(job_titles.contains(&"Software Engineer".to_string()));
        assert!(job_titles.contains(&"Product Manager".to_string()));

        // Step 7: Verify the applied status is correctly set:
        let applied_status: Vec<String> = jobs.iter().map(|j| j.get_applied()).collect();
        assert!(applied_status.contains(&"Yes".to_string()));
        assert!(applied_status.contains(&"No".to_string()));

        // Step 8: Verify the links:
        let links: Vec<String> = jobs.iter().filter_map(|j| Some(j.get_link())).collect();
        assert!(links.contains(&"https://example1.com".to_string()));
        assert!(links.contains(&"https://example2.com".to_string()));
    }

    // Make sure that if a table exists, the drop method can delete it successfully:
    #[test]
    fn test_drop_table() {
        let connection = setup_database();

        _drop_table(&connection, "jobs").expect("Failed to drop table");

        let result = connection.prepare("SELECT COUNT(*) FROM jobs");
        assert!(
            result.is_err(),
            "The table should be dropped and not accessible"
        );
    }

    // Ensure that the application status for a job can change:
    #[test]
    fn test_update_applied() {
        let connection = setup_database();

        let job = Job::new(
            Some(0),
            "Software Engineer".to_string(),
            50.0,
            "0".to_string(), // Initially not applied.
            Some("https://example.com".to_string()),
        );
        insert_job(&connection, &job);

        update_applied(&connection, true, 1).expect("Failed to update applied status");

        let jobs = get_jobs(&connection).expect("Failed to fetch jobs");
        assert_eq!(jobs.len(), 1, "There should be 1 job in the result.");
        assert_eq!(
            jobs[0].get_applied(),
            "Yes",
            "The applied status should be updated to 'Yes'"
        );
    }

    // Count the rows and check for accuracy:
    #[test]
    fn test_count_rows() {
        let connection = setup_database();

        let job1 = Job::new(
            Some(0),
            "Software Engineer".to_string(),
            50.0,
            "0".to_string(),
            Some("https://example1.com".to_string()),
        );
        let job2 = Job::new(
            Some(0),
            "Product Manager".to_string(),
            60.0,
            "1".to_string(),
            Some("https://example2.com".to_string()),
        );
        insert_job(&connection, &job1);
        insert_job(&connection, &job2);

        let count = count_jobs(&connection);
        assert_eq!(count, 2, "There should be 2 jobs in the table.");
    }

    // Check if database is empty:
    #[test]
    fn test_database_empty() {
        let connection = setup_database();
        assert!(
            database_empty(&connection).expect("Failed to check if database is empty"),
            "The database should be empty initially."
        );

        let job = Job::new(
            Some(0),
            "Software Engineer".to_string(),
            50.0,
            "0".to_string(),
            Some("https://example.com".to_string()),
        );
        insert_job(&connection, &job);

        assert!(
            !database_empty(&connection).expect("Failed to check if database is empty"),
            "The database should not be empty after insertion."
        );
    }
}
