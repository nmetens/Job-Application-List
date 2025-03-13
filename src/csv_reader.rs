// Author: Nathan Metens
// Class: Rust 523
// Professor: Bart Massey

//! Read the data inside of a csv file that conforms to a
//! job application. The csv file path is passed into the
//! method. The method checks for headers, then loops through
//! the file catpuring each line and unwraps their data.

use csv::Reader;
use std::error::Error;
use crate::Job;

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
///jobs: vec![],
///
/// match read_csv_file("application.csv", &mut jobs) {
///     Ok(updated_app) => println!("CSV file loaded successfully!"),
///     Err(e) => eprintln!("Failed to read CSV: {}", e),
/// }
/// // Assumes that the application.csv file doesn't change: 
/// assert_eq!(jobs.get(0).expect("ERROR").get_title(), "Bus Driver");
/// ```
pub fn read_csv_file<'a>(
    file: &'a str,
    jobs: &'a mut Vec<Job>,
) -> Result<&'a Vec<Job>, Box<dyn Error>> {
    let mut csv_reader = Reader::from_path(file)?;

    for job in csv_reader.records() {
        match job {
            Ok(record) => {
                let job_id: u32 = record
                    .get(3)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(0); // Default to 0 (false) if None or parsing fails.
                let job_title = record.get(1).expect("Failed to read title").to_string();
                let hourly_rate: f32 = record
                    .get(2)
                    .and_then(|s| s.parse::<f32>().ok()) // Parse if Some, return None if parse fails.
                    .unwrap_or(0.0); // Default to 0.0 if None or parsing fails.
                let applied: String = record
                    .get(3)
                    .and_then(|s| s.parse::<u32>().ok()) // Try to parse as u32
                    .map(|n| n.to_string()) // Convert u32 to String
                    .unwrap_or_else(|| "0".to_string()); // Default to "0" if parsing fails
                let link: String = record.get(4).unwrap_or("N/A").to_string();

                let new_job = Job::new(Some(job_id), job_title, hourly_rate, applied, Some(link));
                jobs.push(new_job); // Add the job to the application.
            }
            Err(e) => eprintln!("Error reading job file: {}", e),
        }
    }
    Ok(jobs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use application::*;

    #[test]
    fn test_read_csv_file() {
        let mut app = Applications::new();

        match read_csv_file("application.csv", &mut app) {
            Ok(_updated_app) => println!("CSV file loaded successfully!"),
            Err(e) => eprintln!("Failed to read CSV: {}", e),
        }
        assert_eq!(app.get_jobs().get(0).expect("ERROR").get_title(), "Bus Driver");
        assert_eq!(app.get_jobs().get(3).expect("ERROR").get_hourly(), 19.0);
    }
}
