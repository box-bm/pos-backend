pub mod models {
    pub mod product;
    pub mod user;
}

pub mod schema;

pub mod apis {
    pub mod auth_api;
    pub mod product_api;
}

pub mod config {
    pub mod db_connection;
    pub mod env;
    pub mod jwt;
}

pub mod middleware {
    pub mod authentication;
}

pub mod extractor {
    pub mod authentication_token;
}

pub mod utils {
    pub mod passwords;
}
