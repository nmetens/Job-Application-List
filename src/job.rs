// Author: Nathan Metens
// Class: Rust 523
// Professor: Bart Massey

//! # Job Module
//!
//! This is the public jobs module. It contains the
//! Job struct and its implementations for a job. Each
//! Job object is encapsulated by the application object.

/// Represting a Job in the application.
///
/// The `Job` struct holds the details for a job.
///
/// # Fields
/// - `title`: The job title as a `String` (position applying to).
/// - `hourly`: The hourly rate (pay) in floating point of the job.
/// - `applied`: The number (1 or 0) for whether or not the job has been applied to.
/// - `link`: The link to the job application.
use serde::{Deserialize, Serialize}; // Serialize trait to pass a job into tera in main.

// Clone trait to make copied of a Job object, and Deserialize/Serialize for tera.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Job {
    id: Option<i64>,
    title: String,
    hourly: f32,
    applied: String,
    link: Option<String>, // link is an optional field.
}

// Struct with only the id. Its purpose is to
// help remove a job from the database:
#[derive(serde::Deserialize)]
pub struct JobRemovalForm {
    pub id: i64,
}

// Define the structure to accept the job ID and new status.
// Used to update the application status of a job:
#[derive(serde::Deserialize)]
pub struct JobStatusUpdate {
    pub id: i64,
    pub applied: bool,
}

// Used to get and post JSON for the javascript onclick method.
#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
}

/// The Job struct creates a job with all relavant fields:
impl Job {
    // Constructor:
    pub fn new(
        id: Option<i64>,
        title: String,
        hourly: f32,
        applied: String,
        link: Option<String>,
    ) -> Self {
        Self {
            id,
            title,
            hourly,
            applied,
            link,
        } // Return self
    }

    /// Getter methods becuase all data in each Job object is private:
    pub fn _get_id(&self) -> i64 {
        self.id.expect("Failed to create job id.")
    }

    pub fn get_link(&self) -> String {
        self.link.clone().expect("No Link.")
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_hourly(&self) -> f32 {
        self.hourly
    }

    pub fn get_applied(&self) -> String {
        self.applied.clone()
    }
}

/// Testing the Job struct from objects.
/// These tests create new objects, and test
/// each method on the struct.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_object() {
        let job = Job {
            id: Some(1),
            title: "Train Driver".to_string(),
            hourly: 30.50,
            applied: "1".to_string(),
            link: Some("http://example.com".to_string()),
        };

        assert_eq!(job.id, Some(1));
        assert_eq!(job.title, "Train Driver");
        assert_eq!(job.hourly, 30.50);
        assert_eq!(job.applied, "1");
        assert_eq!(job.link, Some("http://example.com".to_string()));
    }

    #[test]
    fn test_get_title() {
        let job = Job {
            id: Some(2),
            title: "Engineer".to_string(),
            hourly: 25.0,
            applied: "0".to_string(),
            link: Some("http://job.com".to_string()),
        };

        assert_eq!(job.title, "Engineer");
    }

    #[test]
    fn test_applied_status() {
        let job_applied = Job {
            id: Some(3),
            title: "Programmer".to_string(),
            hourly: 50.0,
            applied: "1".to_string(),
            link: None,
        };

        let job_not_applied = Job {
            id: Some(4),
            title: "Designer".to_string(),
            hourly: 40.0,
            applied: "0".to_string(),
            link: None,
        };

        assert_eq!(job_applied.applied, "1");
        assert_eq!(job_not_applied.applied, "0");
    }

    #[test]
    fn test_optional_link() {
        let job_with_link = Job {
            id: Some(5),
            title: "Pilot".to_string(),
            hourly: 60.0,
            applied: "0".to_string(),
            link: Some("http://pilot.com".to_string()),
        };

        let job_without_link = Job {
            id: Some(6),
            title: "Chef".to_string(),
            hourly: 20.0,
            applied: "1".to_string(),
            link: None,
        };

        assert_eq!(job_with_link.link, Some("http://pilot.com".to_string()));
        assert_eq!(job_without_link.link, None);
    }
}
