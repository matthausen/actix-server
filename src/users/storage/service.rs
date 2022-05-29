use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::client::fluent_builders::PutItem;

use crate::users::storage::model::{User};

#[async_trait::async_trait]
pub trait DynamoDB {
    async fn put_item(
        &self,
        table_name: String,
        item: AttributeValue,
    ) -> PutItem;
    // ... more methods
}


// The storage service
pub struct Storage {
}