use crate::users::storage::service::{Storage};
use crate::users::storage::model::{User};


// Storage Interface
pub trait StorageInterface {
    fn get(&self, id: &str) -> User;
}

// The storage service
pub struct UserService {
    // Config
    // Traits
    // StorageService
}