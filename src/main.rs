use std::time::Duration;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn my_handler() -> impl Responder {
    std::thread::sleep(Duration::from_secs(5));     //  <-- Bad practice! will cause the current worker thread to hang!
    "response"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok))).workers(2)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
