use actix_session::{storage::RedisSessionStore, Session, SessionMiddleware};
use actix_web::{
    get, middleware, put,
    web::{self, get, resource, route, Html},
    App, HttpResponse, HttpServer, Responder,
};
use diesel::{r2d2, PgConnection};
use dotenvy::dotenv;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::env;

mod db;
use db::dbms;

// Services
mod auth;
mod pf2;
use pf2::pf2_services;

use log::{Level, Metadata, Record};

#[get("/")]
async fn index(session: Session) -> actix_web::Result<impl Responder> {
    // TODO: Send API guide
    Ok(HttpResponse::Ok().body("test"))
}

#[derive(Debug, Serialize, Deserialize)]
struct UserRegistration {
    username: String,
    password: String,
}

#[put("/user")]
async fn register_user(
    user_data: web::Json<UserRegistration>,
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
) -> actix_web::Result<impl Responder> {
    use db::user_mgmt::{create_user, UserMgmtError};
    let mut conn = pool.get().unwrap();
    match create_user(&mut conn, &user_data.username, &user_data.password) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => match e {
            UserMgmtError::UsernameTakenError => {
                Ok(HttpResponse::NotAcceptable().body("Username taken"))
            }
            _ => Ok(HttpResponse::InternalServerError().body("Unknown error occured")),
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("trace"));
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL NOT SET!");
    let redis_addr = env::var("REDIS_ADDR").expect("REDIS_ADDR NOT SET!");
    let mut server_addr = env::var("SERVER_ADDR").expect("SERVER_ADDR NOT SET!");
    match env::var("SERVER_PORT") {
        Ok(port) => {
            server_addr.push_str(":");
            server_addr.push_str(port.as_str());
        }
        Err(_) => (),
    };

    info!(
        "Begining server initialization with the following params:
    db_url={}
    redis_addr={}
    server_addr={}",
        db_url, redis_addr, server_addr
    );

    let db_pool = dbms::init_db_pool(db_url.as_str());

    let private_key = actix_web::cookie::Key::generate();
    let store = match RedisSessionStore::new(redis_addr).await {
        Ok(s) => s,
        Err(e) => {
            error!("Recieved error: {}, ending startup", e);
            panic!("Recieved error: {}\nFailed to initialize Redis server", e);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(SessionMiddleware::builder(store.clone(), private_key.clone()).build())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(
                web::resource("/auth")
                    .route(web::post().to(auth::login))
                    .route(web::delete().to(auth::logout))
                    .route(web::get().to(auth::test_auth)),
            )
            .service(register_user)
            .service(
                web::resource("/character")
                    .route(web::get().to(pf2_services::get_character_list))
                    .route(web::put().to(pf2_services::create_new_character)),
            )
            .service(pf2_services::get_full_character)
            .service(pf2_services::update_character_ability)
            .service(pf2_services::update_character_skill)
    })
    .bind(server_addr)?
    .run()
    .await
}
