use aws_sdk_dynamodb::{Error};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::client::fluent_builders::PutItem;


use crate::users::storage::service::{Storage, DynamoDB};

pub async fn create(
    dynamodb_service: &dyn DynamoDB
) -> Result<(PutItem), Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("Storage layer create called!");
    
    let test: &str = "test";
    let name_test: &str = "test_table";
    let test_av = AttributeValue::S(test.into());

    let result = dynamodb_service
        .put_item(name_test.to_string(), test_av)
        .await;
    
    
    Ok(result)
}