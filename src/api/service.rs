use crate::users::storage::model::{User};

pub trait ServiceInterface {
    fn get_users(&self, id: &str) -> User;
}

pub struct ServiceHandler {}