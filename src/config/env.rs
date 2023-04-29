use std::env;

use dotenvy::dotenv;

pub fn get_enviroment_variable(key: String) -> String {
    dotenv().ok();
    env::var(key.clone()).expect(format!("{} must be set in your eviroment", key).as_str())
}
