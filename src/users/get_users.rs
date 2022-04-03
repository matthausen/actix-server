// Call storage to create user

use crate::users::storage::get::{get};


pub fn get_users() {
    println!("Service layer called!");
    get();
}