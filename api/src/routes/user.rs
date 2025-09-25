use std::sync::{Arc, Mutex};
use std::env;

use jsonwebtoken::{EncodingKey, Header, encode};
use poem::{Error, handler, http::StatusCode, web::{Data, Json}};

use serde::{Deserialize, Serialize};

use store::store::Store;

use crate::{request_inputs::CreateUserInput, request_outputs::{CreateUserOutput, SignInOutput}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims  {
    pub sub: String,
    pub exp: usize
}

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
    dotenv::dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("Set JWT Env");
    let mut locked_s = s
    .lock()
    .unwrap();

    let user_id = locked_s
    .sign_in(data.username, data.password);

    match user_id {
        Ok(user_id) => {
           
            let my_claims = Claims {
                sub: user_id,
                exp: 111111111111111111
            };

            let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref()))
            .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

            let response = SignInOutput {
                jwt: token
            };

            Ok(Json(response))
        }        
        Err(_) => {
            return Err(Error::from_status(StatusCode::UNAUTHORIZED))
        }

    }

}