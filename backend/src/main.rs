use actix_web::{HttpServer, App, web, HttpResponse};
use crate::util::constant::{CFG, ENV_ADDRESS, ENV_PORT};

mod dbs;
mod gql;
mod models;
mod services;
mod util;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::to(|| HttpResponse::Ok().body("helloworld!")))
    })
        .bind(
            format!(
                "{}:{}",
                CFG.get(ENV_ADDRESS).unwrap(),
                CFG.get(ENV_PORT).unwrap())
        )?
        .run()
        .await
}
