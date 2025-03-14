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
use crate::database_methods::{get_jobs, enter_data, create_table};
use tera::{Tera};
use actix_files::Files;

use crate::job::Job;

/// Log messages from the server side.
fn log(message: &str) {
   info!("{}", message); 
}


async fn add_jobs_get(tera: web::Data<Tera>) -> impl Responder {

    info!("Rendering job submission form...");

    // If no form has been submitted, render the "add.html" page
    let add_page = tera
        .render("add.html", &tera::Context::new())
        .unwrap_or_else(|_| "Error rendering template".to_string());

    HttpResponse::Ok().body(add_page)
}

async fn add_jobs_post(form: web::Form<Job>) -> impl Responder {

    info!("Received job form: {:?}", form);
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
        "No" => 0,
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

    log("Received request: GET /jobs");
    match get_jobs(&connection) {
        Ok(jobs) => {
            let mut context = tera::Context::new();
            context.insert("jobs", &jobs); // Passing the jobs list to the html.

            let renderer = tera.render("jobs.html", &context).unwrap();
            HttpResponse::Ok().content_type("text/html").body(renderer)
        }
        Err(err) => {
             error!("Error fetching jobs: {}", err);
             HttpResponse::InternalServerError().body("Error fetching jobs.")
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
    log(&("Server listening on \"".to_owned() + url + "\""));

    // Initialize Tera template engine where the html files are located:
    let tera = Tera::new("templates/**/*").unwrap();

    let server = HttpServer::new(move || {
        App::new()
            // When url: http://localhost:8000/jobs, call list_jobs() method that connects.
            // to database of displays jobs in html:
            .app_data(web::Data::new(tera.clone())) // Add Tera to Actix app data.
            .route("/", web::get().to(list_jobs))
            .route("/add", web::get().to(add_jobs_get))  // Handle GET for the form
            .route("/add", web::post().to(add_jobs_post)) // Handle POST to submit the form
            .service(Files::new("/static", "./static").show_files_listing()) // Serve the static style.css files.
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
