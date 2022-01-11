use actix_web::{HttpServer, App, web, HttpResponse, guard};
use crate::util::constant::{CFG, ENV_ADDRESS, ENV_PORT, ENV_GQL_VER, ENV_GIQL_VER};
use crate::gql::{build_schema, graphql, graphiql};

mod dbs;
mod gql;
mod models;
mod util;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let schema = build_schema().await;

    println!("{}",
        format!(
            "{}:{}",
            CFG.get(ENV_ADDRESS).unwrap(),
            CFG.get(ENV_PORT).unwrap())
    );

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(
                web::resource(CFG.get(ENV_GQL_VER).unwrap())
                    .guard(guard::Post())
                    .to(graphql)
            )
            .service(
                web::resource(CFG.get(ENV_GIQL_VER).unwrap())
                    .guard(guard::Get())
                    .to(graphiql)
            )
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
