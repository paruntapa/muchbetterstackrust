use poem::{
    get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server
};


use request_inputs::CreateWebsiteInput;
use request_outputs::CreateWebsiteOutput;

use store::store::Store;

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("hello: {website_id}")
}

#[handler]
fn create_website(Json(_data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let s = Store::default();

    let id = s.create_website();

    let response = CreateWebsiteOutput {
        id
    };

    Json(response)

}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //specify app
    let app: Route = Route::new()
    .at("/website/:website_id", get(get_website))
    .at("/website", post(create_website));

    //creates and run the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}