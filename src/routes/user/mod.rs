
/*

Making this pub mod create_user; makes it impossible to do use crate routes::user::*; in my main.rs which is my desired outcome to
not have a messy main.rs full of many imports for each of my routes handlers.

*/
mod create_user;

pub use create_user::*;