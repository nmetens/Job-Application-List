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
///     applied: "Yes", // True
///     link: None,
/// };
/// ```
use serde::{Serialize, Deserialize}; // Serialize trait to pass a job into tera in main.

// Clone trait to make copied of a Job object, and Deserialize/Serialize for tera.
#[derive(Clone, Serialize, Deserialize, Debug)] 
pub struct Job {
    id: Option<i64>,
    title: String,
    hourly: f32,
    applied: bool,
    link: Option<String>, // link is an optional field.
}

// Struct with only the id. Its purpose is to 
// help remove a job from the database:
#[derive(serde::Deserialize)]
pub struct JobRemovalForm {
    pub id: i64,
}

// Define the structure to accept the job ID and new status
#[derive(serde::Deserialize)]
pub struct JobStatusUpdate {
    pub id: i64,
    pub applied: bool,
}

impl Job {
    // Constructor:
    //pub fn new(job_id: i64, title: String, hourly: f32) -> Self {
    pub fn new(id: Option<i64>, title: String, hourly: f32, applied: bool, link: Option<String>) -> Self {
        Self {
            id,
            title,
            hourly,
            applied,
            link,
        } // Return self
    }

    pub fn _get_id(&self) -> i64 { 
        self.id.clone().expect("Failed to create job id.")
    }

    pub fn get_link(&self) -> String { 
        self.link.clone().expect("No Link.")
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_hourly(&self) -> f32 {
        self.hourly.clone()
    }

    pub fn get_applied(&self) -> String {
        if self.applied { "Yes".to_string() } else { "No".to_string() }
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
        assert_eq!(job.applied, "Yes");
        assert_eq!(job.get_applied(), "Yes");
    }
}
