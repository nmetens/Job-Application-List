#!/bin/bash

. "$HOME/.cargo/env" # Activate cargo for the project.
clear # Clear the screen.
echo "" # New line.

# Function to query the database created from rust program:
query () {
    touch jobs_query.sql
    echo ".headers on" > jobs_query.sql
    echo ".mode column" >> jobs_query.sql
    echo "$1" >> jobs_query.sql # Takes first arg for query.
    sqlite3 jobs_data.db < jobs_query.sql
}

# Command line interface menu.
case "$1" in
    sql) # Use sql on the database directly:
        if [ "$2" == "display" ]; then
            echo "Displaying Database:"
            sqlite3 jobs_data.db < display.sql
        elif [ "$2" == "sort" ]; then
            if [ "$3" == "asc" ]; then
                query "SELECT * FROM jobs ORDER BY hourly_rate ASC"
            elif [ "$3" == "desc" ]; then
                touch jobs_query.sql
                echo ".headers on" > jobs_query.sql
                echo ".mode column" >> jobs_query.sql
                echo "SELECT * FROM jobs ORDER BY hourly_rate DESC" > jobs_query.sql
                sqlite3 jobs_data.db < jobs_query.sql
            fi
        elif [ "$2" == "total" ]; then
            touch jobs_query.sql
            echo ".headers on" > jobs_query.sql
            echo ".mode column" >> jobs_query.sql
            echo "SELECT COUNT(*) as 'Total Jobs' FROM jobs" >> jobs_query.sql
            sqlite3 jobs_data.db < jobs_query.sql
        elif [ "$2" == "applied" ]; then
            if ["$3" == "yes" ]; then
                touch jobs_query.sql
                echo ".headers on" > jobs_query.sql
                echo ".mode column" >> jobs_query.sql
                echo "SELECT COUNT(*) as 'Total Jobs' FROM jobs" >> jobs_query.sql
                sqlite3 jobs_data.db < jobs_query.sql
            fi    
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
