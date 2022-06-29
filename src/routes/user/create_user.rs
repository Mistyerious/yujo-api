
use rocket::*;

#[get("/")]
pub async fn create_user() -> String {
    return String::from("Hello, World!")
}