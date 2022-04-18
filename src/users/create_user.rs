use mockall::*;
use mockall::predicate::*;

use crate::api::handler_post_user::{UserBodyRequest};
//use crate::users::storage::create::{create};
use crate::users::storage::model::{User};

// Call storage to create user
// pub fn create_user(user: UserBodyRequest) -> i32{
//     println!("Service layer called!");

//     // transform the server req into db model request
//     let item = parse_request_to_db_model(user);

//     // call the storage method create
//     let res = create(item);

//     // return the response to the handler
//     res
// }


// Transform the server request into DB model
fn parse_request_to_db_model(req: UserBodyRequest) -> User {
    let user = User {
        first_name: req.first_name,
        last_name: req.last_name,
        email: req.email,
        password: req.password,
    };

    user
}