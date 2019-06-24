extern crate diesel;

use crate::models::{Server, NewServerSnapshot};
use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{BufWriter, BufReader};
use std::thread;
use bytebuffer::ByteBuffer;
use std::time::Duration;
use diesel::sql_types::Timestamp;
use chrono::naive::NaiveDateTime;
use integer_encoding::*;
use diesel::SqliteConnection;
use std::fs::read;
use json;
use diesel::query_dsl::RunQueryDsl;
use crate::establish_connection;

pub fn create_snapshot<'a>(id: String, address: String, time: NaiveDateTime) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(&address)?;
    let mut input_stream = stream.try_clone()?;
    let mut output_stream = BufWriter::new(&mut stream);
    let mut packet = ByteBuffer::new();

    //  handshake packet https://wiki.vg/Server_List_Ping#Handshake
    packet.write(&0_u32.encode_var_vec())?;              // packet id 0x00
    packet.write(&47_u32.encode_var_vec())?;             // protocol version
    packet.write(&address.len().encode_var_vec())?;      // server address string length
    packet.write(&address.as_bytes())?;                  // server address
    packet.write(&25565_u16.to_be_bytes())?;             // server port
    packet.write(&1_u32.encode_var_vec())?;              // next state - 1 for status
    packet.flush()?;

    output_stream.write(&packet.to_bytes().iter().len().encode_var_vec())?;
    output_stream.write(&packet.to_bytes())?;
    output_stream.flush()?;

    packet.clear();

    // https://wiki.vg/Server_List_Ping#Request
    packet.write(&0_u32.encode_var_vec())?; // packet id 0x00
    output_stream.write(&packet.to_bytes().iter().len().encode_var_vec())?;
    output_stream.write(&packet.to_bytes())?;

    thread::spawn(move || {
        use crate::schema::server_snapshots;
        let mut client_buffer = Vec::new();
        let connection = establish_connection();

        'outer: loop {
            match input_stream.read_to_end(&mut client_buffer) {
                Ok(n) => {
                    if n == 0 {
                        thread::sleep(Duration::new(1, 0));
                    } else {
                        let response = String::from_utf8_lossy(&client_buffer);

                        let data = json::parse(&response[response.find('{').unwrap()..]
                            .trim_matches(char::from(0))).unwrap();

                        let players = &data["players"]["online"];

                        let new_snapshot = NewServerSnapshot {
                            server: &id,
                            time: &time,
                            players: &players.as_i32().unwrap()
                        };

                        diesel::insert_into(server_snapshots::table)
                            .values(&new_snapshot)
                            .execute(&connection)
                            .expect("Failed to save server snapshot.");

                        println!("{}", format!("Created server snapshot for {0}.", &id));

                        break 'outer
                    }
                },
                Err(error) => println!("{:?}", error),
            }
        }
    });

    Ok(())
}