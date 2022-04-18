use aws_sdk_dynamodb::{Error};

use crate::users::storage::service::{Storage, DynamoDB};


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