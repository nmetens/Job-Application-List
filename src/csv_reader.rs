use std::error::Error;
use csv::Reader;
use crate::application; // To use the job.rs methods module

/** Read the data inside of a csv file that conforms to a
    job application. The csv file path is passed into the
    method. The method checks for headers, then loops through
    the file catpuring each line and unwraps their data.
    */
pub fn read_csv_file<'a>(file: &'a str, mut app: &'a application::Applications) -> Result<&'a application::Applications, Box<dyn Error>> {
    let mut csv_reader = Reader::from_path(file)?;
 
    // Read and print header row
    //let headers = rdr.headers()?.clone();
    //println!("Headers: {:?}", headers);

    for job in csv_reader.records() {
        match job {
            Ok(record) => {
                let job_id = record.get(0).unwrap_or("N/A");
                let job_title = record.get(1).unwrap_or("N/A");
                let hourly_rate = record.get(2).unwrap_or("N/A");
                let applied = record.get(3).unwrap_or("N/A");
                

                println!(
                    "Job ID: {}, Title: {}, Hourly Rate: {}, Applied: {}",
                    job_id, job_title, hourly_rate, applied
                );
            }
            Err(e) => eprintln!("Error reading record: {}", e),
        }
    }

    Ok(app)
}
