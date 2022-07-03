use actix_web::http::{StatusCode};
use actix_web::{error, post, web, Error, HttpResponse};
use futures::StreamExt;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::users::storage::model::{User};
use crate::users::service::{UserService};

const CONTENT_TYPE: &str = "application/json; charset=utf-8";
const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[derive(Serialize, Deserialize)]
pub struct UserBodyRequest {
    pub first_name: String,
    pub last_name: String,
    pub age: String,
    pub utype: String,
}

#[post("/api/v1/user")]
async fn post_user(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let server_req = serde_json::from_slice::<UserBodyRequest>(&body)?;

    let new_user = parse_request_to_db_model(server_req);

    let res = UserService::create_user(new_user).await;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(CONTENT_TYPE)
        .json(res))
}

// Transform the server request into DB model
fn parse_request_to_db_model(req: UserBodyRequest) -> User {
    let new_uuid = Uuid::new_v4().to_string();
    
    User {
        id: new_uuid,
        first_name: req.first_name,
        last_name: req.last_name,
        age: req.age,
        utype: req.utype
    }
}