#[macro_use] extern crate rocket;

mod routes;

use crate::routes::user::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/users", routes![create_user])
}