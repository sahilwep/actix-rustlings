use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

// extract path info using serde
#[get("/user/{user_id}/{friend}")]   //  <- define path parameter
async fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}",
        info.friend, info.user_id
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(index)      // impl for custom type_safe extractor
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}