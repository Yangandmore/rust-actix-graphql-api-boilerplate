use lazy_static::lazy_static;
use std::collections::HashMap;

pub const ENV_ADDRESS: &str = "ADDRESS";
pub const ENV_PORT: &str = "PORT";
pub const ENV_SQL_URI: &str = "SQL_URI";
pub const ENV_GQL_VER: &str = "GQL_VER";
pub const ENV_GIQL_VER: &str = "GIQL_VER";


lazy_static! {
    pub static ref CFG: HashMap<&'static str, String> = {
        dotenv::dotenv().ok();

        let mut map = HashMap::new();

        map.insert(
            ENV_ADDRESS,
            dotenv::var(ENV_ADDRESS).expect("未找到ADDRESS配置!")
        );
        map.insert(
            ENV_PORT,
            dotenv::var(ENV_PORT).expect("未找到PORT配置!")
        );
        map.insert(
            ENV_SQL_URI,
            dotenv::var(ENV_SQL_URI).expect("未找到SQL_URI配置!")
        );
        map.insert(
            ENV_GQL_VER,
            dotenv::var(ENV_GQL_VER).expect("未找到GQL_VER配置!")
        );
        map.insert(
            ENV_GIQL_VER,
            dotenv::var(ENV_GIQL_VER).expect("未找到GIQL_VER配置!")
        );
        map
    };
}