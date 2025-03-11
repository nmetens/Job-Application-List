// Author: Nathan Metens
// Class: Rust 523
// Professor: Bart Massey

//! # Job Module
//!
//! This is the public jobs module. It contains the 
//! Job struct and its implementations for a job. Each
//! Jon object is encapsulated by the application object.

/// Represting a Job in the application.
///
/// The `Job` struct holds the details for a job.
///
/// # Fields
/// - `title`: The job title as a `String` (position applying to).
/// - `hourly`: The hourly rate (pay) in floating point of the job.
/// - `applied`: The number (1 or 0) for whether or not the job has been applied to.
/// 
/// # Example
/// ```
/// let job = Job {
///     id: 1,
///     title: "Bus Driver".to_string(),
///     hourly: 27.50,
///     applied: 1, // True
///     link: None,
/// };
/// ```
#[derive(Clone)] // Clone trait to make copied of a Job object.
pub struct Job {
    id: u32,
    title: String,
    hourly: f32,
    applied: u32,
    link: String,
}

impl Job {
    // Constructor:
    //pub fn new(job_id: u32, title: String, hourly: f32) -> Self {
    pub fn new(id: u32, title: String, hourly: f32, applied: u32, link: String) -> Self {
        Self {
            id,
            title,
            hourly,
            applied,
            link,
        } // Return self
    }

    // Display job info:
    pub fn _display(&self) -> String {
        //println!("Job ID: {}. Title: {}. Pay: {}. Applied: {}",
        //self.job_id, self.title, self.hourly, self.applied);
        //format!("Job ID: {0}. Title: {1}. Pay: {2}. Applied: {3}",
        format!(
            "Title: {0}. Pay: {1}. Applied: {2}",
            //self.job_id, self.title, self.hourly, self.applied).to_string() // Return the String
            self.title,
            self.hourly,
            self.applied
        )
        .to_string() // Return the String
    }

    pub fn get_id(&self) -> u32 { self.id.clone() }

    pub fn get_link(&self) -> String { self.link.clone() }

    // Update application status:
    pub fn _applied(&mut self, applied: u32) {
        self.applied = applied;
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_hourly(&self) -> f32 {
        self.hourly.clone()
    }

    pub fn get_applied(&self) -> u32 {
        self.applied.clone()
    }
}

/// Testing the Job struct from objects.
///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_object() {
        let title = "Train Driver".to_string();
        let _job = Job::new(title, 30.50, 1);
    }

    #[test]
    fn test_get_title() {
        let job = Job::new("Train Driver".to_string(), 30.50, 1);
        assert_eq!(job.title, "Train Driver".to_string());
        assert_eq!(job.get_title(), "Train Driver".to_string());
    }

    #[test]
    fn test_get_hourly() {
        let job = Job::new("Train Driver".to_string(), 30.50, 1);
        assert_eq!(job.hourly, 30.50);
        assert_eq!(job.get_hourly(), 30.50);
    }

    #[test]
    fn test_get_applied() {
        let job = Job::new("Train Driver".to_string(), 30.50, 1);
        assert_eq!(job.applied, 1);
        assert_eq!(job.get_applied(), 1);
    }
}
