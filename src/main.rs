use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;

#[macro_use]
extern crate serde_derive;

// Model: User struct with id, name, email
#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String
}

// DATABASE_URL
const DB_URL: &str = env!("DATABASE_URL");

// constants
const OK_RESP: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT_FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL_SERVER_ERROR\r\n\r\n";

// main function
fn main() {
    // Set database
    if let Err(e) = set_database() {
        println!("Error: {}", e);
        return;
    }

    // Start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server started at port 8080");

    // Handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

// set_database function
fn set_database() -> Result<(), PostgresError> {
    // Connect to database
    let mut client = Client::connect(DB_URL, NoTls)?;

    // Create table
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )"
    )?;
    Ok(())
}