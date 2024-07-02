use dotenvy::dotenv;
use std::env;

mod db;

use db::dbms;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let url: String = env::var("DATABASE_URL").expect("DATABASE_URL NOT SET!");

    let db_pool = dbms::init_db_pool(url.as_str());
}
