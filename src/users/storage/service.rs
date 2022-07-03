use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::client::fluent_builders::PutItem;

#[async_trait::async_trait]
pub trait DynamoDB {
    async fn put_item(&self) -> PutItem;
    // ... more methods
}


// The user storage service
pub struct Storage {
    pub table_users: String,
    pub client: Client
}