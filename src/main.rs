// Author: Nathan Metens
// Class: Rust 523
// Professor: Bart Massey

mod csv_reader;
mod job; // References job.rs file
mod database_methods;

// Logging used for the server side to 
// see GET and POST requests:
use log::{info, LevelFilter, error};
use env_logger::Builder;
use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io::Write;
use rusqlite::{Connection};
use crate::database_methods::{get_jobs, enter_data, create_table, remove_data, update_applied};
use crate::job::JobRemovalForm;
use tera::{Tera, Context};
use actix_files::Files;

use crate::job::Job;

async fn rem_job(form: web::Form<JobRemovalForm>, tera: web::Data<Tera>) -> impl Responder {

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
    let mut context = Context::new();

    // Render the remove page again with error message:
    let page = tera
        .render("home.html", &context)
        .unwrap_or_else(|_| "Error rendering template".to_string());

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

async fn add_job(form: web::Form<Job>) -> impl Responder {

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

async fn list_jobs(tera: web::Data<Tera>) -> impl Responder {
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

    info!("Received request: GET /jobs");
    match get_jobs(&connection) {
        Ok(jobs) => {
            let mut context = tera::Context::new();
            context.insert("jobs", &jobs); // Passing the jobs list to the html.

            info!("Fetched Jobs From jobs.html Form: {:?}", &jobs);

            let renderer = tera.render("jobs.html", &context).unwrap();
            HttpResponse::Ok().content_type("text/html").body(renderer)
        }
        Err(err) => {
             error!("Error fetching jobs: {}", err);
             HttpResponse::InternalServerError().body("Error fetching jobs.")
        }
    }
}

use serde::Serialize;
#[derive(Serialize)]
struct ApiResponse {
    success: bool,
}

// Define the structure to accept the job ID and new status
#[derive(serde::Deserialize)]
pub struct JobStatusUpdate {
    pub id: i64,
    pub applied: bool,
}

async fn update(form: web::Json<JobStatusUpdate>) -> impl Responder {
    println!("Received update request: id={}, applied={}", form.id, form.applied); // ✅ Debugging log
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

    let job_id = form.id;
    let job_applied = form.applied.clone();

    //let result = update_applied(&connection, job_applied, job_id);
    match update_applied(&connection, job_applied, job_id) {
        Ok(_) => {
            info!("Successfully updated application status in database.");
            HttpResponse::Ok().json(ApiResponse { success: true }) // ✅ Return JSON
        },
        Err(err) => {
            eprintln!("Error updating application status in database: {}", err);
            HttpResponse::InternalServerError().json(ApiResponse { success: false })
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host: &str = "127.0.0.1"; // localhost
    let port: &str = "8000"; // listen on port 8000 for requests to server.

    let url: &str = &(host.to_owned() + ":" +  port); // Where the server is listening.

    // Job application database file:
    let database_file: &str = "jobs_data.db";

    // Create an SQLite database file. Open the database
    // file if it already exists.
    let connection = match Connection::open(database_file) {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error opening the database: {}", err);
            std::process::exit(1); // Stop execution if the table fails to create
        }
    };

    // Create the table if it doesn't exist:
    if let Err(err) = create_table(&connection) {
        error!("Error creating table: {}", err);
        std::process::exit(1); // Stop execution if the table fails to create
    }

    // Set RUST_LOG=info to allow server-side loggin:
    if env::var("RUST_LOG").is_err() { env::set_var("RUST_LOG", "info"); }

    // Create a custom logging system with specific filters to simplify loggin:
    Builder::new()
        // Define a custom log format to display only the message:
        .format(|buf, record| {
            writeln!(buf, "{}", record.args()) // No timestamp or level or workers.
        })
        // Filter internal Actix logs that are unnecessary:
        .filter(Some("actix_web"), LevelFilter::Error) // Suppress actix_web logs.
        .filter(Some("actix_rt"), LevelFilter::Error) // Suppress Actix runtime logs.
        .filter(Some("actix_server"), LevelFilter::Error) // Suppress Actix server logs.
        .filter(None, LevelFilter::Info) // Allow only my info-level logs.
        .init();

    // Only log my messages, no verbose internal actix logs:
    info!("Server listening on \"{}", url);

    // Initialize Tera template engine where the html files are located:
    let tera = Tera::new("templates/**/*").unwrap();

    let server = HttpServer::new(move || {
        App::new()
            // When url: http://localhost:8000/jobs, call list_jobs() method that connects.
            // to database of displays jobs in html:
            .app_data(web::Data::new(tera.clone())) // Add Tera to Actix app data.
            .service(Files::new("/static", "./static").show_files_listing()) // Serve the static style.css files.
            .route("/", web::get().to(list_jobs))
            .route("/add", web::post().to(add_job)) // POST for adding jobs.
            .route("/rem", web::post().to(rem_job)) // POST for removing jobs.
            .route("/update", web::post().to(update))
    });

    // Fix: Properly handle the `.bind()` result
    let server = match server.bind(&url) {
        Ok(server) => server,
        Err(err) => {
            eprintln!("Failed to bind server to {}: {}", url, err);
            return Err(err);
        }
    };

    server.run().await
}
