mod service;

use serde::{Serialize, Deserialize};

#[rbatis::crud_enable(table_name: "user")]
#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct User {
    id: i32,
    username: String,
    password: String
}
