use crate::users::storage::model::{User};
use crate::users::storage::service::Storage;
use crate::users::service::{UserService};

impl UserService {
    pub async fn create_user(&self, user: User) {
        println!("User Service called!");

        // call the storage method create
        let res = Storage::create(user).await;
        println!("Added item to table. {:?}", res);   
    }
}

