use std::sync::{Arc, Mutex};

use poem::{
    EndpointExt, Route, Server, get, listener::TcpListener, post
};

use store::store::Store;

use crate::routes::{user::{sign_in, sign_up}, website::{create_website, get_website}};

pub mod routes;
pub mod request_inputs;
pub mod request_outputs;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(
        Mutex::new(
            Store::new().unwrap()
        )
    );
    //Specify App
    let app= Route::new()
    .at("/website/:website_id", get(get_website))
    .at("/website", post(create_website))
    .at("/sign-in", post(sign_in))
    .at("/sign-up", post(sign_up))
    .data(s);

    //creates and run the http server
    Server::new(TcpListener::bind("0.0.0.0:8181"))
        .name("hello-world")
        .run(app)
        .await
}
