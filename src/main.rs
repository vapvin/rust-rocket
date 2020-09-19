#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

mod other {
    #[get("/world")]
    pub fn world() -> &'static str {
        "Hello, world!"
    }
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, outside world!"
}

use other::world;

#![feature(proc_macro_hygiene, decl_macro)]

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/hello", routes![world]).launch();
}