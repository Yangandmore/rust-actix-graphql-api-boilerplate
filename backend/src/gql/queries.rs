use async_graphql::Context;
use slog::{Logger, info};

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn add(&self, ctx: &Context<'_>, a: i32, b: i32) -> i32 {
        let result = ctx.data_unchecked::<Logger>();

        info!(result, "");

        a + b
    }

    async fn get(&self) -> &str {
        "hello!"
    }

}