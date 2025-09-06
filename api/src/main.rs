use poem::{
    get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server
};

use request_inputs::CreateWebsiteInput;
use request_outputs::CreateWebsiteOutput;

use store::store::Store;

use crate::{request_inputs::CreateUserInput, request_outputs::{CreateUserOutput, GetWebsiteOutput, SignInOutput}};

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(id): Path<String>) -> Json<GetWebsiteOutput>{
    let mut s = Store::default().unwrap();
    let website = s.get_website(id).unwrap();

    Json(GetWebsiteOutput{
        url: website.url
    })


}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput>{
    let mut s = Store::default().unwrap();

    let id = s.sign_up(data.username, data.password).unwrap();

    let response = CreateUserOutput {
        id: id
    };

    Json(response)
}

#[handler]
fn sign_in(Json(data): Json<CreateUserInput>) -> Json<SignInOutput>{
    let mut s = Store::default().unwrap();

    s.sign_in(data.username, data.password).unwrap();

    let response = SignInOutput {
        jwt: String::from("sundola")
    };

    Json(response)
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let mut s = Store::default().unwrap();

    let website = s.create_website(String::from("c870fd51-d0e3-427b-9aa9-2be8c9170a70"), data.url).unwrap();

    let response = CreateWebsiteOutput {
        id: website.id
    };

    Json(response)

}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //specify app
    let app: Route = Route::new()
    .at("/website/:website_id", get(get_website))
    .at("/website", post(create_website))
    .at("/sign-in", post(sign_in))
    .at("/sign-up", post(sign_up));

    //creates and run the http server
    Server::new(TcpListener::bind("0.0.0.0:8181"))
        .name("hello-world")
        .run(app)
        .await
}