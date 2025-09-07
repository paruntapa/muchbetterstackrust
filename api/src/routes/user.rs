use std::sync::{Arc, Mutex};

use poem::{handler, web::{Data, Json}};
use store::store::Store;

use crate::{request_inputs::CreateUserInput, request_outputs::{CreateUserOutput, SignInOutput}};

#[handler]
pub fn sign_up(
    Json(data): Json<CreateUserInput>, 
    Data(s): Data<&Arc<Mutex<Store>>>
) -> Json<CreateUserOutput> {
    let mut locked_s = s
    .lock()
    .unwrap();

    let id = locked_s
    .sign_up(data.username, data.password)
    .unwrap();

    let response = CreateUserOutput {
        id: id
    };

    Json(response)
}

#[handler]
pub fn sign_in(
    Json(data): Json<CreateUserInput>, 
    Data(s): Data<&Arc<Mutex<Store>>>
) -> Json<SignInOutput>{
    let mut locked_s = s
    .lock()
    .unwrap();

    locked_s
    .sign_in(data.username, data.password)
    .unwrap();

    let response = SignInOutput {
        jwt: String::from("sundola")
    };

    Json(response)
}