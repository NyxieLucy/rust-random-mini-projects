// for a smoth basic webapp i used poem, it kinda looks nice, i wonder if i can add any graphical stuff

use poem::{get, post, handler,
    web::{Path,Json},
    Server, listener::TcpListener, Result};
use serde::{Serialize};
 use poem::Route;
 // so far i know that the functions are counted as pages here, but i'll keep reading n testing
#[handler]
fn index() -> &'static str {
    "STAY HYDRATED HOMIE"
}
#[handler] 
fn hello(Path(name): Path<String>) -> String {
    format!("Greetings, {}", name)
}

#[derive(serde::Deserialize)]
struct UserRequest {
    username: String,
}
#[derive(Serialize)]
struct UserResponse{
    message: String,
    status: String,
}
#[handler]
fn create_user(Json(body): Json<UserRequest>) -> Json<UserResponse> {
    let response = UserResponse {
        message: format!("User '{}' created successfully!", body.username),
        status: "success".to_string(),
    };
    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/", get(index))
        .at("/hello/:name", get(hello))
        .at("/user", post(create_user));

    println!("Server starting at http://127.0.0.1:3000");

    // Bind and start the server
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
