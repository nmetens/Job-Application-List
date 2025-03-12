// Author: Nathan Metens
// Class: Rust 523
// Professor: Bart Massey

//! # Application Module
//!
//! This public module holds the application struct and its
//! implementation. Each Application will hold a list of
//! jobs as well as the total jobs in that current application.
 
use crate::job::Job; // To use the job.rs methods module

/// Representing a collection of jobs.
///
/// The `Applications` struct holds a list of jobs and keeps track of the total jobs. 
///
/// # Fields
/// - `jobs`: Vector of job objects containing the job entries from a csv file.
/// - `total_jobs`: The total job entries inside the application.
///
/// # Example
/// ```
/// let apps = Applications {
///     jobs: vec![],
///     total_jobs: 0,
/// };
/// ```
pub struct Applications {
    /// List of job object entries:
    jobs: Vec<Job>,

    /// How many jobs are in the app:
    total_jobs: u32,
}

impl Applications {
    // Construct a new application:
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            total_jobs: 0,
        }
    }

    /** Method that takes a string and an int for setting the job_title
    and the job_rate from csv file data: */
    pub fn add_job(&mut self, id: u32, job_title: &str, job_rate: f32, applied: u32, link: String) {
        //self.jobs.push(Job::new(self.total_jobs, job_title.to_string(), job_rate as f32));
        self.jobs.push(Job::new(
            Some(id),
            job_title.to_string(),
            job_rate as f32,
            (applied as u32).to_string(),
            Some(link.to_string())
        ));
        self.total_jobs += 1;
    }

    pub fn _add_job_obj(&mut self, job: Job) {
        self.jobs.push(job);
        self.total_jobs += 1;
    }

    // Loop through jobs list and display each job:
    /*pub fn _view_apps(&mut self) -> String {
        let mut data: String = "".to_string();
        for job in &self.jobs {
            data += &(job._display() + "\n");
        }
        data
    }*/

    pub fn get_jobs(&self) -> Vec<Job> {
        self.jobs.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_new_object() {
        let _app = Applications::new();
    }

    #[test]
    fn test_add_job_obj() {
        let job = Job::new("Waiter".to_string(), 16.50, 0);
        let mut app = Applications::new();
        app._add_job_obj(job.clone());

        assert_eq!(app.total_jobs, 1);
        assert_eq!(app.jobs.get(0).expect("ERROR").get_title(), job.get_title());
        assert_eq!(app.jobs.get(0).expect("ERROR").get_hourly(), job.get_hourly());
        assert_eq!(app.jobs.get(0).expect("ERROR").get_applied(), job.get_applied());
    }

    #[test]
    fn test_add_job() {
        let job = Job::new("Waiter".to_string(), 16.50, 0);
        let mut app = Applications::new();
        app.add_job("Waiter", 16.50, 0);

        assert_eq!(app.total_jobs, 1);
        assert_eq!(app.jobs.get(0).expect("ERROR").get_title(), job.get_title());
        assert_eq!(app.jobs.get(0).expect("ERROR").get_hourly(), job.get_hourly());
        assert_eq!(app.jobs.get(0).expect("ERROR").get_applied(), job.get_applied());
    }

    #[test]
    fn test_get_jobs() {
        let job = Job::new("Waiter".to_string(), 16.50, 0);
        let mut app = Applications::new();
        app._add_job_obj(job.clone());

        assert_eq!(app.total_jobs, 1);
        assert_eq!(app.get_jobs().get(0).expect("ERROR").get_title(), job.get_title());
        assert_eq!(app.get_jobs().get(0).expect("ERROR").get_hourly(), job.get_hourly());
        assert_eq!(app.get_jobs().get(0).expect("ERROR").get_applied(), job.get_applied());
    }
}
