use chrono::naive::NaiveDateTime;
use super::schema::servers;
use super::schema::server_snapshots;

#[derive(Queryable)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub address: String,
    pub rank: i32,
    pub record: i32,
    pub versions: String,
}

#[derive(Queryable)]
pub struct ServerSnapshot {
    pub id: i32,
    pub server: Server,
    pub time: NaiveDateTime,
    pub players: i32
}

#[derive(Insertable)]
#[table_name="server_snapshots"]
pub struct NewServerSnapshot<'a> {
    pub server: &'a str,
    pub time: &'a NaiveDateTime,
    pub players: &'a i32
}

#[derive(Insertable)]
#[table_name="servers"]
pub struct NewServer<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub address: &'a str,
}