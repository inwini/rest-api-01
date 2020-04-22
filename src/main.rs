use actix_web::{web, App, HttpResponse, HttpServer, Responder, };
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

/// This handler uses json extractor
async fn r3ds(item: web::Json<MyObj>) -> HttpResponse {
    println!("[POST] Value model: {:?}", &item);
    HttpResponse::Ok().json(&item.0.name) // <- send response
}

async fn ob1() -> impl Responder {
    HttpResponse::Ok().body("General Kenobi!!")
}

async fn ob2() -> impl Responder {
    HttpResponse::Ok().body("I have bad feeling about this.")
}

async fn r2ds() -> impl Responder {
    println!("R2DS has been called");
    HttpResponse::Ok().body("Got it")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running on");
    HttpServer::new(|| {
        App::new().service(web::scope("/api")
            .route("/hello/there", web::get().to(ob1))
            .route("/bad/feeling", web::get().to(ob2))
        ).service(web::scope("/app")
            .route("/may/the/force", web::post().to(r3ds))
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}