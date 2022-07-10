use actix_web::{web};
use aws_sdk_dynamodb::{Client, Endpoint};
use aws_sdk_dynamodb::client::fluent_builders::PutItem;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_smithy_http::result::{SdkError};

use crate::config::load_config::{Config};
use crate::users::model::{User};


// The user storage service
pub struct Storage {
    pub table_users: String,
    pub client: Client,
}

impl Storage{
    // pub async fn new(cfg: &'static Config) -> Self {
    //     let aws_config = aws_config::load_from_env().await;
    //     let endpoint = &cfg.yml_cfg.dynamodb.endpoint[..];
    //     let table_users = cfg.yml_cfg.dynamodb.table_name.to_string();

    //     let dynamodb_local_aws_config = aws_sdk_dynamodb::config::Builder::from(&aws_config)
    //     .endpoint_resolver(
    //         Endpoint::immutable(actix_web::http::Uri::from_static(endpoint)),
    //     )
    //     .build();
    //     let client = Client::from_conf(dynamodb_local_aws_config);

    //     Storage {
    //         table_users,
    //         client,
    //     }
    // }

    pub async fn create(
        client: web::Data<Client>,
        new_user: User,
        table_name: &str,
    ) -> Result<(), SdkError<aws_sdk_dynamodb::error::PutItemError>> {
        
        let id = AttributeValue::S(new_user.id);
        let type_av = AttributeValue::S(new_user.utype);
        let age_av = AttributeValue::S(new_user.age);
        let first_av = AttributeValue::S(new_user.first_name);
        let last_av = AttributeValue::S(new_user.last_name);
        
    
        match client //self.dynamo_service
            .put_item()
            .table_name(table_name)
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