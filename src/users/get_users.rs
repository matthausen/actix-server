use mockall::*;
use mockall::predicate::*;

use crate::users::service::{UserService};
use crate::users::storage::service::{Storage};
use crate::users::storage::{get};
use crate::users::storage::model::{User};


impl UserService {
    pub fn get_users(&self, id: &str) -> User {
        let user = Storage::get(&Storage{}, id);
        
        user
    }
}