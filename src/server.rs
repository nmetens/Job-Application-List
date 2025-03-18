// Author: Nathan Metens
// Class: Rust 523
// Professor: Bart Massey

///! # Server Module
///
///! This module contains all the mothods used by the server to 
///! list the jobs in the database, add a jobs to the database,
///! remove a job from the database, and change the applciation
///! status for a job in the database.

use log::{info, error};
use actix_web::{web, HttpResponse, Responder};
use rusqlite::{Connection};
use crate::database_methods::{get_jobs, enter_data, remove_data, update_applied};
use crate::job::{JobRemovalForm, JobStatusUpdate, ApiResponse};
use tera::Tera;
use crate::job::Job;

/// Remove a Job from the server.
///
/// This function:
/// - Established a connection to the database using the database methods.
/// - Captures the id from the HTML form that asks the user which job to delete.
/// - Calls the database method that removes a job by id and returns to home route.
pub async fn rem_job(form: web::Form<JobRemovalForm>) -> impl Responder {

    info!("DELETE Request to Database..."); 
    let database_file = "jobs_data.db";

    let connection = match Connection::open(database_file) {
        Ok(conn) => conn,

        Err(err) => {
            eprintln!("Error opening the database: {}", err);
            return HttpResponse::InternalServerError().body("Error opening the database.");
        }
    };

    let job_id = form.id;

    // Call remove method with the connection and the id captured from the html form:
    match remove_data(&connection, job_id) {
        Ok(true) => {
            // Redirect to the jobs list page after successful form submission:
            info!("Successful DELETE in database.");
            HttpResponse::Found().append_header(("LOCATION", "/")).finish()
        },

        Ok(false) => {
            info!("No job with id {} found in the database.", job_id);
            HttpResponse::Found().append_header(("LOCATION", "/")).finish()
        },

        Err(_) => {
            eprintln!("Error removing the job from the database.");
            HttpResponse::Found().append_header(("LOCATION", "/")).finish()
        }
    }
}

/// Add a Job to the Server's Database.
///
/// This function:
/// - Connects to the database.
/// - Gets the form data and ensures that the application status is ready to insert
///   into the database by converting the type to a string.
/// - Creates a new Job object using the Job constructor in the Job module.
/// - Calls the database method to enter the job.
/// - Checks that the result is as expected.
pub async fn add_job(form: web::Form<Job>) -> impl Responder {

    info!("POST Request to Database...");
    info!("Received Job Form: {:?}", form);
    // If the form has been submitted, process the data (POST)
    // Open the SQLite database
    let database_file = "jobs_data.db";
    let connection = match Connection::open(database_file) {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error opening the database: {}", err);
            return HttpResponse::InternalServerError().body("Error opening the database.");
        }
    };

    // Insert the new job into the database
    let applied_int = match form.get_applied().as_str() {
        "Yes" => 1,
        _ => 0, // Default to "No" if somehow invalid value is sent
    };

    let new_job = Job::new(
        None, // For autoincrement in database.
        form.get_title().clone(),
        form.get_hourly(),
        applied_int.to_string(),
        Some(form.get_link().clone())
    );
    info!("Job Link: {:?}", new_job.get_link());

    let result = enter_data(&connection, &new_job);

    match result {
        Ok(_) => {
            // Redirect to the jobs list page after successful form submission
            info!("Successful POST to database.");
            HttpResponse::Found().append_header(("LOCATION", "/")).finish()
    },
        Err(err) => {
            eprintln!("Error inserting job into the database: {}", err);
            HttpResponse::InternalServerError().body("Error inserting job into the database.")
        }
    }
}

/// List the Jobs
///
/// This method:
/// - Creates a connection to the database.
/// - Call the get_jobs method that inserts all the jobs from the database
///   into the HTML front end to be displayed.
pub async fn list_jobs(tera: web::Data<Tera>) -> impl Responder {
    // Job application database file:
    let database_file: &str = "jobs_data.db";

    // Create an SQLite database file. Open the database
    // file if it already exists.
    let connection = match Connection::open(database_file) {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error opening the database: {}", err);
            return HttpResponse::InternalServerError().body("Error opening the database.");
        }
    };

    match get_jobs(&connection) {
        Ok(jobs) => {
            info!("Jobs to render: {:?}", jobs); // Add this log to debug

            let mut context = tera::Context::new();
            context.insert("jobs", &jobs);

            match tera.render("jobs.html", &context) {
                Ok(renderer) => HttpResponse::Ok().content_type("text/html").body(renderer),
                Err(err) => {
                    error!("Template rendering error: {:?}", err);
                    HttpResponse::InternalServerError().body(format!("Error rendering template: {:?}", err))
                }
            }
        },
        Err(err) => {
             error!("Error fetching jobs: {}", err);
             HttpResponse::InternalServerError().body("Error fetching jobs.")
        }
    }
}

/// Update a Jobs Application Status in the Database:
///
/// This function:
/// - Connects to the database:
/// - Captures the form's info from the HTML front-end (id and application status).
/// - Calls the database update_applied method to change the apllication status.
///
/// This method returns JSON to the front end Javascript function so that the
/// application status can be updated automatically with a color change.
pub async fn update(form: web::Json<JobStatusUpdate>) -> impl Responder {
    println!("Received update request: id={}, applied={}", form.id, form.applied);
    // Job application database file:
    let database_file: &str = "jobs_data.db";

    // Create an SQLite database file. Open the database:
    // file if it already exists.
    let connection = match Connection::open(database_file) {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error opening the database: {}", err);
            return HttpResponse::InternalServerError().body("Error opening the database.");
        }
    };

    let job_id = form.id;
    let job_applied = form.applied;

    //let result = update_applied(&connection, job_applied, job_id);
    match update_applied(&connection, job_applied, job_id) {
        Ok(_) => {
            info!("Successfully updated application status in database.");
            HttpResponse::Ok().json(ApiResponse { success: true }) // Return JSON to the JS Method.
        },
        Err(err) => {
            eprintln!("Error updating application status in database: {}", err);
            HttpResponse::InternalServerError().json(ApiResponse { success: false })
        }
    }
}
