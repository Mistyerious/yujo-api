
use rocket::*;

#[get("/")]
pub fn create_user() -> String {
    return String::from("Hello, World!")
}