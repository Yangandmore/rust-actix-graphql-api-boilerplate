pub mod queries;

use async_graphql::Schema;

type ActixSchema = Schema<
    queries::QueryRoot,
    async_graphql::EmptyMutation,
    async_graphql::EmptySubscription,
>;