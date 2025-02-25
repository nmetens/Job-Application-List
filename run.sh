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
    sqlite3 jobs_data.db < jobs_query.sql # Perform sql query on the database.
}

# Command line interface menu.
case "$1" in
    sql) # Use sql on the database directly:

        if [ "$2" == "display" ]; then
            echo "Displaying Database:"
            sqlite3 jobs_data.db < display.sql

        elif [ "$2" == "sort" ]; then
            if [ "$3" == "asc" ]; then
                echo "Displaying Jobs in ASCENDING order with lowest paying first:"
                # Call query function, passing in the sql query as a param:
                query "SELECT * FROM jobs ORDER BY hourly_rate ASC"

            elif [ "$3" == "desc" ]; then
                echo "Displaying Jobs in DESCENDING order with highest paying first:"
                query "SELECT * FROM jobs ORDER BY hourly_rate DESC"
            fi

        elif [ "$2" == "total" ]; then
            echo "Displaying Total Jobs:"
            query "SELECT COUNT(*) as 'Total Jobs' FROM jobs"

        elif [ "$2" == "applied" ]; then
            if [ "$3" == "yes" ]; then
                echo "Displaying Jobs applied to:"
                query "SELECT * FROM jobs WHERE applied = 1"

            elif [ "$3" == "no" ]; then
                echo "Displaying Jobs NOT yet applied to:"
                query "SELECT * FROM jobs WHERE applied = 0"
            fi    

        else # Deafult, no sql command, just display all:
            echo "DEAFULT - Displaying Database:"
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
    docs)
        cargo doc --open
        ;;
    *) # Display the usage for the project when no 
       # command line args are passed in to bash file:
        echo "Usage: $0 <command>"
        echo ""
        echo "Commands:"
        echo "  sql <query>     - Execute SQL-related query (e.g., display queries)"
        echo "  build           - Build the project in rust"
        echo "  fmt             - Check for rust format"
        echo "  clippy          - Rust clippy"
        echo "  run             - Run the project in rust"
        echo "  test            - Run test cases in rust"
        echo "  docs            - Open the project documentation"
        echo ""
        echo "Examples:"
        echo "  $0 sql display      # Display database job table"
        echo "  $0 sql sort asc     # Display the database job table in asc order based on hourly_ rate (lowest paying first)"
        echo "  $0 sql sort desc    # Display the database job table in desc order based on hourly_ rate (highest paying first)"
        echo "  $0 sql total        # Display the total jobs in the table"
        echo "  $0 sql applied yes  # Display the total jobs in the table applied to"
        echo "  $0 sql applied no   # Display the total jobs in the table NOT applied to"
        echo "  $0 build            # Build the project"
        echo "  $0 run              # Run the project"
        echo ""

        exit 1
        ;;
esac
