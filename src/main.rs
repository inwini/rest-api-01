use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn ob1() -> impl Responder {
    HttpResponse::Ok().body("General Kenobi!!")
}

async fn ob2() -> impl Responder {
    HttpResponse::Ok().body("I have bad feeling about this.")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::scope("/api")
            .route("/hello/there", web::get().to(ob1))
            .route("/bad/feeling", web::get().to(ob2))
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}