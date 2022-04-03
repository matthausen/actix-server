use actix_session::{Session};
use actix_web::http::{StatusCode};
use actix_web::{
    get, 
    HttpRequest, 
    HttpResponse, 
    Result
};

use crate::users::get_users::{get_users};

const CONTENT_TYPE: &str = "application/json; charset=utf-8";


#[get("/api/v1/users")]
async fn get_all_users(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("handler get_users called");

    get_users();

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(CONTENT_TYPE)
        .body("Hello Users"))
}