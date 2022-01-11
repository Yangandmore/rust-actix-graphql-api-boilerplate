pub mod queries;

use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use actix_web::{web, HttpResponse};
use crate::dbs::sql;
use crate::log::get_logger;
use crate::gql::queries::QueryRoot;
use crate::util::constant::{CFG, ENV_GQL_VER};

type ActixSchema = Schema<
    queries::QueryRoot,
    async_graphql::EmptyMutation,
    async_graphql::EmptySubscription,
>;

pub async fn build_schema() -> ActixSchema {
    let sql = sql().await;
    let logger = get_logger();

    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(sql)
        .data(logger)
        .finish()
}

pub async fn graphql(schema: web::Data<ActixSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> std::io::Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(
                playground_source(
                    GraphQLPlaygroundConfig::new(CFG.get(ENV_GQL_VER).unwrap())
                        .subscription_endpoint(CFG.get(ENV_GQL_VER).unwrap())
                )
            )
    )

}

