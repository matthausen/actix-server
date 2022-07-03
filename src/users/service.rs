use crate::users::storage::model::{User};

// Storage Interface
pub trait UserStorage {
    fn get(&self, id: &str) -> User;
    fn create(&self, new_user: User) -> User;
}

// The storage service
pub struct UserService {
    storage: dyn UserStorage,
}