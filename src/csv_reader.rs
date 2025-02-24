use std::error::Error;
use csv::Reader;
use crate::application; // To use the job.rs methods module

/** Read the data inside of a csv file that conforms to a
    job application. The csv file path is passed into the
    method. The method checks for headers, then loops through
    the file catpuring each line and unwraps their data.
    */
pub fn read_csv_file<'a>(file: &'a str, app: &'a mut application::Applications) -> Result<&'a application::Applications, Box<dyn Error>> {
    let mut csv_reader = Reader::from_path(file)?;
 
    for job in csv_reader.records() {
        match job {
            Ok(record) => {
                //let job_id = record.get(0).unwrap_or("N/A");
                let job_title = record.get(1).unwrap_or("N/A");
                let hourly_rate: f32 = record.get(2)
                    .and_then(|s| s.parse::<f32>().ok()) // Parse if Some, return None if parse fails
                    .unwrap_or(0.0); // Default to 0.0 if None or parsing fails
                let applied: u32 = record.get(3)
                    .and_then(|s| s.parse::<u32>().ok()) 
                    .unwrap_or(0); // Default to (0) false if None or parsing fails

                app.add_job(job_title, hourly_rate, applied); // Add the job to the application.
            }
            Err(e) => eprintln!("Error reading job file: {}", e),
        }
    }

    Ok(app)
}
