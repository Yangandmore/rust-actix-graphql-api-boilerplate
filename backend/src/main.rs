use actix_web::{HttpServer, App, web, HttpResponse};

mod dbs;
mod gql;
mod models;
mod services;
mod util;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::to(|| HttpResponse::Ok().body("helloworld！")))
    })
        .bind("127.0.0.1")?
        .run()
        .await
}
