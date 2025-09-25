use std::sync::{Arc, Mutex};

use poem::{Error, handler, http::{StatusCode}, web::{Data, Json}};
use store::store::Store;

use crate::{request_inputs::CreateUserInput, request_outputs::{CreateUserOutput, SignInOutput}};

#[handler]
pub fn sign_up(
    Json(data): Json<CreateUserInput>, 
    Data(s): Data<&Arc<Mutex<Store>>>
) -> Result<Json<CreateUserOutput>, Error> {
    let mut locked_s = s
    .lock()
    .unwrap();

    let id = locked_s
    .sign_up(data.username, data.password)
    .map_err(|_| Error::from_string("User already exists", StatusCode::CONFLICT))?;

    let response = CreateUserOutput {
        id
    };

    Ok(Json(response))
}

#[handler]
pub fn sign_in(
    Json(data): Json<CreateUserInput>, 
    Data(s): Data<&Arc<Mutex<Store>>>
    
) -> Result<Json<SignInOutput>, Error>{
    let mut locked_s = s
    .lock()
    .unwrap();

    let user_id = locked_s
    .sign_in(data.username, data.password);

    match user_id {
        Ok(user_id) => {
            let response = SignInOutput {
                jwt: user_id
            };

            Ok(Json(response))
        }        
        Err(_) => {
            return Err(Error::from_status(StatusCode::UNAUTHORIZED))
        }

    }

}