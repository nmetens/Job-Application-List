// Author: Nathan Metens
// Class: Rust 523
// Professor: Bart Massey

//! Read the data inside of a csv file that conforms to a
//! job application. The csv file path is passed into the
//! method. The method checks for headers, then loops through
//! the file catpuring each line and unwraps their data.

use crate::application;
use csv::Reader;
use std::error::Error;

/// Reads a csv file.
///
/// Reads the data inside a csv file expecting the following format: (id,job_title,hourly_rate,applied).
///
/// # Arguments
/// * `file`: A string slice (`&str`) representing the name of the csv file to read.
/// * `app`: A mutable reference to the `Applications` object from the application module which has the list of jobs.
/// 
/// # Returns
/// Returns the `Result` containing the updated app reference on success.
/// Returns an error of type `Box<dyn Error>` on failure when the csv file is unreadable.
/// 
/// # CSV Format
/// The csv file must follow this structure:
/// ```csv
/// id,job_title,hourly_rate,applied
/// 1,Bus Driver,25,0
/// 2,Waiter,16,1
/// 3,Engineer,65,1
/// ```
///
/// # Example
/// ```
/// let mut app = Applications {
///     jobs: vec![],
///     total_jobs: 0,
/// };
///
/// match read_csv_file("jobs.csv", &mut app) {
///     Ok(updated_app) => println!("CSV file loaded successfully!"),
///     Err(e) => eprintln!("Failed to read CSV: {}", e),
/// }
/// ```
pub fn read_csv_file<'a>(
    file: &'a str,
    app: &'a mut application::Applications,
) -> Result<&'a application::Applications, Box<dyn Error>> {
    let mut csv_reader = Reader::from_path(file)?;

    for job in csv_reader.records() {
        match job {
            Ok(record) => {
                let job_title = record.get(1).unwrap_or("N/A");
                let hourly_rate: f32 = record
                    .get(2)
                    .and_then(|s| s.parse::<f32>().ok()) // Parse if Some, return None if parse fails.
                    .unwrap_or(0.0); // Default to 0.0 if None or parsing fails.
                let applied: u32 = record
                    .get(3)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(0); // Default to 0 (false) if None or parsing fails.

                app.add_job(job_title, hourly_rate, applied); // Add the job to the application.
            }
            Err(e) => eprintln!("Error reading job file: {}", e),
        }
    }
    Ok(app)
}
