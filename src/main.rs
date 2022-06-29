#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
pub mod routes;
use crate::routes::user;

fn main() {
    rocket::ignite()
        .mount("/users", routes![user::create_user])
        .launch();
}