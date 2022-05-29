use aws_sdk_dynamodb::{Error};

use crate::users::storage::service::{Storage, DynamoDB};
use crate::users::storage::model::{User};


// async fn get(
//     // Take a reference to our trait object instead of the S3 client.
//     list_objects_impl: &dyn DynamoDB,
//     bucket: &str,
//     prefix: &str,
// ) -> Result<usize, Box<dyn Error + Send + Sync + 'static>> {
//     println!("Storage layer called!");
//     let mut next_token: Option<String> = None;
    
//     let result = list_objects_impl
//             .list_objects(bucket, prefix, next_token.as_deref())
//             .await?;
    
//     Ok(result)
// }

impl Storage {
    pub fn get(&self, id: &str) -> User {
        let new_email = format!("test{}@example.com", id);
        User {
            email: new_email,
            first_name: String::from("Alice"),
            last_name: String::from("Wonderland"),
            password: String::from("secret_password"),
        }
    }
}