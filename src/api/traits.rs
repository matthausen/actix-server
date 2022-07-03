use crate::users::storage::model::{User};

pub trait UserService {
    fn create_user(&self) -> User;
}

