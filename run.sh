#!/bin/bash

. "$HOME/.cargo/env" # Activate cargo for the project.
clear # Clear the screen.
echo "" # New line.

# Command line interface menu.
case "$1" in
    sql) # Use sql on the database directly:
        if [ "$2" == "display" ]; then
            echo "Displaying Database:"
            sqlite3 jobs_data.db < display.sql
        else # Deafult, no sql command, just display all:
            echo "Displaying Database:"
            sqlite3 jobs_data.db < display.sql
        fi
        ;;
    # Call cargo methods for the project:
    build)
        cargo build
        ;;
    fmt)
        cargo fmt
        ;;
    clippy)
        cargo clippy
        ;;
    run)
        cargo run
        ;;
    test)
        cargo test
        ;;
    *) # Display the usage for the project when no 
       # command line args are passed in to bash file:
        echo "Usage: $0 {sql|build|fmt|clippy|run|test}"
        exit 1
        ;;
esac
