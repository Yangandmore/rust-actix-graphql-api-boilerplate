use async_graphql::{Context, Error, ErrorExtensions};
use rbatis::rbatis::Rbatis;
use crate::models::user::User;
use rbatis::crud::CRUD;

pub async fn all_users(
    sql: &Rbatis
) -> std::result::Result<Vec<User>, Error> {
    let users = sql.fetch_list::<User>("").await.unwrap();

    if users.len() >= 0 {
        Ok(users)
    } else {
        Err(
            Error::new("user error")
                .extend_with(|_, e| e.set("details", "error"))
        )
    }
}