use std::io;
use rocket::response::{NamedFile};

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("dist/index.html")
}