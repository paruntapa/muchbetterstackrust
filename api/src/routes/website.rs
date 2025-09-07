use std::sync::{Arc, Mutex};

use poem::handler;
use poem::{
    web::{Data, Json, Path}
};
use store::store::Store;

use crate::request_inputs::CreateWebsiteInput;
use crate::request_outputs::{CreateWebsiteOutput, GetWebsiteOutput};


#[handler]
pub fn get_website(Path(id): Path<String>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<GetWebsiteOutput>{
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(id).unwrap();

    Json(GetWebsiteOutput{
        url: website.url
    })


}

#[handler]
pub fn create_website(Json(data): Json<CreateWebsiteInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();

    let website = locked_s.create_website(String::from("c870fd51-d0e3-427b-9aa9-2be8c9170a70"), data.url).unwrap();

    let response = CreateWebsiteOutput {
        id: website.id
    };

    Json(response)

}
