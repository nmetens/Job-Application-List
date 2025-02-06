/** This file holds the application struct and its
  * implementation. Each Application will hold a list
  * of jobs as well as the total jobs in that current
  * application.
  */

use std::io; // To get user input for app
use crate::job::Job; // To use the job.rs methods module

// Holds a list of jobs:
//#[derive(Copy)] // Allows copies of struct objects
pub struct Applications {
    jobs: Vec<Job>,
    total_jobs: u32, // How many jobs are in the app
}

impl Applications {
    // Construct a new application:
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            total_jobs: 0,
        }
    }

    pub fn add_job(&mut self) {
        println!("Enter the job title: ");
        let mut job_title = String::new();

        io::stdin()
            .read_line(&mut job_title)
            .expect("Failed to read");
        job_title = (job_title.trim()).to_string(); // Remove '\n' from input

        println!("Enter the hourly rate: ");
        let mut job_rate_str = String::new();
        let mut job_rate: f32 = 0.0;

        // Ask the user for an hourly rate
        // until a valid one is inputed:
        let mut valid_rate: bool = false;
        while !valid_rate {
            job_rate_str.clear(); // Clear each time before reading new value
            io::stdin() // Read the input
                .read_line(&mut job_rate_str)
                .expect("Failed to read");

            // If the input is correct, assign it to an int,
            // otherwise display an error message and try again:
            job_rate = match job_rate_str.trim().parse::<f32>() {
                Ok(rate) => { 
                    valid_rate = true; // Update validity of rate
                    rate // Return the rate
                },
                Err(_) => {
                    println!("Invalid hourly pay rate. Try again: ");
                    continue;
                }
            };
        }
        
        self.jobs.push(Job::new(self.total_jobs, job_title, job_rate));
        self.total_jobs += 1;
    }

    // Loop through jobs list and display each job:
    pub fn view_apps(&mut self) -> String {
        let mut data: String = "".to_string();
        for job in &self.jobs {
            data += &(job.display() + "\n");
        }
        data
    }
}
