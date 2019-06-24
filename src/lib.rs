#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod minecraft;

use diesel::SqliteConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::thread;
use std::env;
use crate::models::{Server, NewServer};
use std::thread::{Thread, JoinHandle};
use std::time::Duration;
use chrono::prelude::{Utc};
use crate::minecraft::create_snapshot;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn update_or_create_server<'a>(conn: &SqliteConnection, id: &'a str, name: &'a str, address: &'a str) -> usize {
    use schema::servers;

    let new_server = NewServer {
        id,
        name,
        address,
    };

    diesel::insert_into(servers::table)
        .values(&new_server)
        .execute(conn)
        .unwrap_or(update_server(&conn, &id, &name, &address))
}

fn update_server<'a>(conn: &SqliteConnection, id: &'a str, _name: &'a str, _address: &'a str) -> usize {
    use schema::servers::dsl::{servers, name, address};

    diesel::update(servers.find(id))
        .set(name.eq(_name))
        .execute(conn)
        .expect("Error updating server.");

    diesel::update(servers.find(id))
        .set(address.eq(_address))
        .execute(conn)
        .expect("Error updating server.")
}

pub fn delete_old_servers<'a>(conn: &SqliteConnection, current_servers: &Vec<String>) -> usize {
    use schema::servers::dsl::*;

    let mut to_delete: Vec<String> = vec![];
    let all_servers = servers.load::<Server>(conn).expect("Failed to load servers.");
    for server in all_servers {
        if !current_servers.contains(&server.id) {
            to_delete.push(server.id)
        }
    }

    diesel::delete(servers.filter(id.eq_any(to_delete))).execute(conn).expect("Failed to delete old servers: ")
}

pub fn fetch_snapshots<'a>(conn: &SqliteConnection) -> std::io::Result<()> {
    use schema::servers::dsl::{servers};
    let all_servers = servers.load::<Server>(conn).expect("Failed to load servers.");

    let task = thread::spawn(move || {
        loop {
            let time = Utc::now().naive_utc();
            for server in &all_servers {
                create_snapshot(server.id.to_string(), server.address.to_string(), time);
            }

            thread::sleep(Duration::new(60, 0));
        }
    });

    Ok(())
}