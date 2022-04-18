use actix_web::http::{StatusCode};
use actix_web::{error, post, web, Error, HttpResponse};
use futures::StreamExt;

use serde::{Deserialize, Serialize};

//use crate::users::create_user::{create_user};

const CONTENT_TYPE: &str = "application/json; charset=utf-8";
const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[derive(Serialize, Deserialize)]
pub struct UserBodyRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}


#[post("/api/v1/user")]
async fn post_user(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    println!("handler create_user called");

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let server_req = serde_json::from_slice::<UserBodyRequest>(&body)?;

    //create_user(server_req);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(CONTENT_TYPE)
        .json("serverReq"))
}