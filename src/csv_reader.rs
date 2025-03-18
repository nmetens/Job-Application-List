// Author: Nathan Metens
// Class: Rust 523
// Professor: Bart Massey

//! Read the data inside of a csv file that conforms to a
//! job application. The csv file path is passed into the
//! method. The method checks for headers, then loops through
//! the file catpuring each line and unwraps their data.

use csv::Reader;
use std::error::Error;
use rusqlite::params;

/// Reads a csv file.
///
/// Reads the data inside a csv file expecting the following format: (id,job_title,hourly_rate,applied,link).
///
/// # Arguments
/// * `file`: A string slice (`&str`) representing the name of the csv file to read.
/// * `app`: A mutable reference to the `Applications` object from the application module which has the list of jobs.
/// 
/// # Returns
/// Returns the `Result` containing the Ok(()).
/// Returns an error of type `Box<dyn Error>` on failure when the csv file is unreadable.
/// 
/// # CSV Format
/// The csv file must follow this structure:
/// ```csv
/// id,job_title,hourly_rate,applied,link
/// 1,Bus Driver,25,0,http://linke1.com
/// 2,Waiter,16,1,http://linke1.com
/// 3,Engineer,65,1,http://linke1.com
/// ```
pub fn read_csv_file(
    file: &str, // The CSV file to add to the database.
    connection: &rusqlite::Connection, // The databse connection.
) -> Result<(), Box<dyn Error>> {

    let mut csv_reader = Reader::from_path(file)?; // Get the reader to the file.

    // Prepare the SQL statement for inserting jobs into the database:
    let mut stmt = connection.prepare(
        "INSERT INTO jobs (id, job_title, hourly_rate, applied, link)
        VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;

    // Loop through each line in the csv file and capture all the data to att to the database.
    for job in csv_reader.records() {
        match job {
            Ok(record) => {
                // The incremental id of each job:
                let id: i64 = record
                    .get(0)
                    .and_then(|s| s.parse::<i64>().ok())
                    .unwrap_or(0); // Default to 0 (false) if None or parsing fails.

                // The job title:
                let job_title = record
                    .get(1)
                    .expect("Failed to read title")
                    .to_string();

                // Paid hourly rate in dolars:
                let hourly_rate: f32 = record
                    .get(2)
                    .and_then(|s| s.parse::<f32>().ok()) // Parse if Some, return None if parse fails.
                    .unwrap_or(0.0); // Default to 0.0 if None or parsing fails.

                // 1 or 0 for yes or no:
                let applied: bool = record
                    .get(3)
                    .and_then(|s| s.parse::<i64>().ok()) // Try to parse as i64.
                    .map(|n| n == 1) // Convert i64 (1 or 0) to bool (true or false).
                    .unwrap_or(false); // Default to false if parsing fails.

                // The link to the application:
                let link: String = record
                    .get(4)
                    .unwrap_or("No Link")
                    .to_string();

                // Insert the job into the database:
                stmt.execute(params![id, job_title, hourly_rate, applied as i64, link])?;
            }
            Err(e) => eprintln!("Error reading application.csv file: {}", e),
        }
    }

    Ok(())
}

/// Testing the read_csv_file method.
/// This testing suite creates a new database connection
/// and a test csv file where two jobs are inserted inside.
/// Then the read_csv_file method extracts the jobs from 
/// the csv file and assesrts that their values are as expected.
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::fs::File;
    use std::io::Write;

    // Testing the read csv file method that extracts jobs inside the csv file.
    #[test]
    fn test_read_csv_file() {
        // Create a new database connection:
        let connection = Connection::open_in_memory().expect("Failed to create in-memory database");
        
        // Create a new jobs table with the appropriate values:
        connection
            .execute(
                "CREATE TABLE jobs (
                    id INTEGER PRIMARY KEY,
                    job_title TEXT NOT NULL,
                    hourly_rate REAL NOT NULL,
                    applied INTEGER NOT NULL,
                    link TEXT NOT NULL
                )",
                [],
            )
            .expect("Failed to create jobs table");

        let csv_filename = "test_application.csv";

        // Create a new csv file:
        let mut file = File::create(csv_filename).expect("Failed to create test CSV file");

        // Write the headers! Very important!
        writeln!(file, "job_id,job_title,hourly_rate,applied,link").unwrap(); // Job 1

        // Add the data into the file:
        writeln!(file, "1,Software Engineer,50.0,1,https://example.com").unwrap(); // Job 1
        writeln!(file, "2,Data Scientist,45.5,0,https://example.com").unwrap(); // Job 2

        file.flush().unwrap();  // Ensure all lines are written before reading.

        // Call the method we are testing and capture the result of () without error.
        let result = read_csv_file(csv_filename, &connection);

        assert!(result.is_ok(), "read_csv_file() should return Ok(())");

        // Count the total rows in the jobs table:
        let count: i64 = connection
            .query_row("SELECT COUNT(*) FROM jobs", [], |row| row.get(0))
            .expect("Failed to count rows");

        assert_eq!(count, 2, "Database should have 2 job entries after reading the CSV.");
    }
}
