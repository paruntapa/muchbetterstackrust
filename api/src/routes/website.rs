use std::sync::{Arc, Mutex};

use poem::handler;
use poem::{
    web::{Data, Json, Path}
};
use store::store::Store;

use crate::auth_middleware::UserId;
use crate::request_inputs::CreateWebsiteInput;
use crate::request_outputs::{CreateWebsiteOutput, GetWebsiteOutput};


#[handler]
pub fn get_website(
    Path(id): Path<String>, 
    Data(s): Data<&Arc<Mutex<Store>>>,
    UserId(user_id): UserId
) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(id, user_id).unwrap();

    Json(GetWebsiteOutput{
        url: website.url
    })
}

#[handler]
pub fn create_website(
    Json(data): Json<CreateWebsiteInput>, 
    Data(s): Data<&Arc<Mutex<Store>>>,
    UserId(user_id): UserId
) -> Json<CreateWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();

    let website = locked_s.create_website(user_id, data.url).unwrap();

    let response = CreateWebsiteOutput {
        id: website.id
    };

    Json(response)
}
