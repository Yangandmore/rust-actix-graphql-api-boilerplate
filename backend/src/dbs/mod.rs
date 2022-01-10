use rbatis::rbatis::Rbatis;
use rbatis::core::db::DBPoolOptions;
use crate::util::constant::{CFG, ENV_SQL_URI};

pub async fn sql() -> Rbatis {
    let rb = Rbatis::new();

    let mut opts = DBPoolOptions::new();

    rb.link_opt(CFG.get(ENV_SQL_URI).unwrap(), &opts).await.unwrap();

    rb
}