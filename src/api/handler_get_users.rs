use actix_session::{Session};
use actix_web::http::{StatusCode};
use actix_web::{
    get,
    web,
    HttpRequest, 
    HttpResponse, 
    Result,
    Responder,
};


use crate::api::service::{ServiceHandler};
use crate::users::service::{UserService};

const CONTENT_TYPE: &str = "application/json; charset=utf-8";


#[get("/api/v1/users")]
async fn get_all_users(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("handler get_users called");

    let id: &'static str = "1";
    let res = UserService::get_users(&UserService{}, id);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(CONTENT_TYPE)
        .json(res))
}