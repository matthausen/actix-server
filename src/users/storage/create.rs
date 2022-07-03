use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::{Client};

use aws_smithy_http::result::{SdkError};
use crate::users::storage::model::{User};
use crate::users::storage::service::{Storage};


impl Storage {
    pub async fn create(
        &self,
        new_user: User
    ) -> Result<(), SdkError<aws_sdk_dynamodb::error::PutItemError>> {
        let client = &self.client;

        let item = User {
            id: String::from("123"),
            first_name: String::from("John"),
            last_name: String::from("Brown"),
            age: String::from("10"),
            utype: String::from("test-2"),
        };
        
        let id = AttributeValue::S(item.id);
        let type_av = AttributeValue::S(item.utype);
        let age_av = AttributeValue::S(item.age);
        let first_av = AttributeValue::S(item.first_name);
        let last_av = AttributeValue::S(item.last_name);
        
    
        match client
            .put_item()
            .table_name(&self.table_users)
            .item("Id", id)
            .item("account_type", type_av)
            .item("age", age_av)
            .item("first_name", first_av)
            .item("last_name", last_av)
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}