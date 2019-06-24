#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate config;

use rocket_contrib::serve::StaticFiles;

mod routes;

use crate::routes::get;
use config::*;
use std::net::*;
use trust_dns_resolver::{Resolver, Name};
use trust_dns_resolver::config::*;
use std::collections::HashMap;
use traak::{establish_connection, update_or_create_server, delete_old_servers, fetch_snapshots};

fn load_servers() -> Result<(), ConfigError> {
    let connection = establish_connection();
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let mut settings = config::Config::default();
    let mut server_ids: Vec<String> = vec![];

    settings.merge(config::File::with_name("settings")).unwrap();

    for server in settings.get_array("servers")? {
        let server = server.try_into::<HashMap<String, String>>()?;
        let id = server.get("id").unwrap();
        let name = server.get("name").unwrap();
        let address = server.get("address").unwrap();

        server_ids.push(id.to_string());

        let lookup = resolver.lookup_srv(&["_minecraft._tcp.", &address].concat());
        if lookup.is_ok() {
            let response = lookup.unwrap();
            let record = response.iter().next().unwrap();

            let ip = record.target().to_string().trim_matches('.').to_string();
            let port = record.port();

            let addr = &[ip, ":".to_string(), port.to_string()].concat();

            let _ = update_or_create_server(&connection, &id, &name, addr);

            continue;
        }


        let _ = update_or_create_server(&connection, &id, &name, &[address, ":25565"].concat());
    }

    delete_old_servers(&connection, &server_ids);
    fetch_snapshots(&connection).unwrap();

    Ok(())
}

fn main() {
    load_servers().unwrap();

    rocket::ignite()
        .mount("/", routes![get::index])
        .mount("/dist/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/dist/static")))
        .launch();
}