/** This file holds the application struct and its
  * implementation. Each Application will hold a list
  * of jobs as well as the total jobs in that current
  * application.
  */

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

    //pub fn add_job(&mut self) {
    /** Method that takes a string and an int for setting the job_title 
        and the job_rate from csv file data: */
    pub fn add_job(&mut self, job_title: &str, job_rate: f32) {
        //self.jobs.push(Job::new(self.total_jobs, job_title.to_string(), job_rate as f32));
        self.jobs.push(Job::new(job_title.to_string(), job_rate as f32));
        self.total_jobs += 1;
    }

    // Loop through jobs list and display each job:
    pub fn _view_apps(&mut self) -> String {
        let mut data: String = "".to_string();
        for job in &self.jobs {
            data += &(job._display() + "\n");
        }
        data
    }

    pub fn get_jobs(&self) -> Vec<Job> {
        self.jobs.clone()
    }
}