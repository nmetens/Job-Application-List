// Author: Nathan Metens
// Class: Rust 523
// Professor: Bart Massey

//! This is the main method.
//! The server is initialized here in main.
//! All the mothod calls used by the routes
//! to manipulate the database are found in the
//! server module which is included.

mod csv_reader;
mod database_methods;
mod job; // References job.rs file
mod server;

// Logging used for the server side to
// see GET and POST requests:
use crate::database_methods::{create_table, database_empty};
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use env_logger::Builder;
use log::{error, info, LevelFilter};
use rusqlite::Connection;
use std::env;
use std::io::Write;
use std::str::FromStr;
use tera::Tera;

// Function to validate if the port is a valid number between 1 and 65535.
fn is_valid_port(port: &str) -> bool {
    match u16::from_str(port) {
        Ok(p) => p > 0,  // Port number greater than 0.
        Err(_) => false, // Return false if the port is not a valid number.
    }
}

/// The main entry point for the Actix Web server.
///
/// This function:
/// - Initializes the SQLite database, opening it or creating it if necessary.
/// - Creates the required table in the database if it doesn't exist.
/// - Checks if the database is empty and populates it with data from a CSV file if it is.
/// - Sets up logging configuration to only display relevant log messages (suppresses unnecessary internal Actix logs).
/// - Initializes the Tera template engine for rendering HTML files.
/// - Configures an Actix Web server with routes to handle jobs listing, adding, removing, and updating jobs.
/// - Binds the server to `127.0.0.1:<port>` (where `<port>` is a command line arg) and starts it.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get command line arguments for the port:
    let args: Vec<String> = env::args().collect();

    // Set default port in case no command line args are given:
    let default_port = "8000";

    // Determine the port to use:
    let port = if args.len() > 1 && is_valid_port(&args[1]) {
        &args[1] // Use the port from arguments if valid.
    } else {
        eprintln!(
            "Invalid port number or no port provided, using default: {}",
            default_port
        );
        default_port // Use the default port if invalid or not provided
    };

    let host: &str = "127.0.0.1"; // localhost
    let url: &str = &(host.to_owned() + ":" + port); // The URL the server will bind to

    // Job application database file:
    let database_file: &str = "jobs_data.db";

    // Create an SQLite database file. Open the database.
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
        std::process::exit(1); // Stop execution if the table fails to create.
    }

    // Check if the database is empty, if it is, add the csv data to the jobs table:
    // Now, we can safely use the connection:
    match database_empty(&connection) {
        Ok(is_empty) => {
            if is_empty {
                println!("The database is empty, adding CSV data...");
                match csv_reader::read_csv_file("application.csv", &connection) {
                    Ok(_) => println!("CSV data successfully added."),
                    Err(e) => eprintln!("Error reading CSV file: {}", e),
                }
            } else {
                println!("The database is not empty.");
            }
        }
        Err(e) => eprintln!("Error checking if the database is empty: {}", e),
    }

    // Set RUST_LOG=info to allow server-side loggin:
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

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
    info!("Server listening on \"{}\"", url);

    // Initialize Tera template engine where the html files are located:
    let tera = Tera::new("templates/**/*").unwrap();

    let server = HttpServer::new(move || {
        App::new()
            // When url: http://localhost:8000/jobs, call list_jobs() method that connects.
            // to database of displays jobs in html:
            .app_data(web::Data::new(tera.clone())) // Add Tera to Actix app data.
            .service(Files::new("/static", "./static").show_files_listing()) // Serve the static style.css files.
            .route("/", web::get().to(server::list_jobs))
            .route("/add", web::post().to(server::add_job)) // POST for adding jobs.
            .route("/rem", web::post().to(server::rem_job)) // POST for removing jobs.
            .route("/update", web::post().to(server::update))
    });

    // Properly handle the `.bind()` result
    let server = match server.bind(&url) {
        Ok(server) => server,
        Err(err) => {
            eprintln!("Failed to bind server to {}: {}", url, err);
            return Err(err);
        }
    };

    server.run().await
}
