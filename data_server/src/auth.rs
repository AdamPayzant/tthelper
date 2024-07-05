use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use diesel::{r2d2, PgConnection};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};

use crate::db::user_mgmt;
use user_mgmt::UserMgmtError;

const TOKEN_LIFESPAN: i64 = 60 * 60 * 24 * 7; // 1 week
pub static KEY: [u8; 16] = *include_bytes!("../secret.key");

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct TokenRes {
    token: String,
    token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientErrResponse {
    error_details: String,
}

pub fn verify_token(t: String, session: &Session) -> bool {
    let token = jsonwebtoken::decode::<UserToken>(
        &t,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
    .unwrap()
    .claims;

    match session.get::<UserToken>(&token.user).unwrap() {
        Some(entry) => {
            // Sanity check, if they don't match something has gone horribly, horribly wrong
            if !token.user.eq(&entry.user) {
                error!("Username does not match token db entry");
                return false;
            }

            if Utc::now().timestamp() > entry.exp {
                session.remove(&token.user);
                return false;
            }

            true
        }
        None => false,
    }
}

pub async fn login(
    auth_data: web::Json<AuthData>,
    sesion: Session,
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    match user_mgmt::authenticate_user(
        &mut conn,
        auth_data.username.as_str(),
        auth_data.password.as_str(),
    ) {
        Ok(true) => {
            debug!("User authenticated, generating token");
            let now = Utc::now().timestamp();
            let payload = UserToken {
                iat: now,
                exp: now + TOKEN_LIFESPAN,
                user: auth_data.username.clone(),
            };
            let token = jsonwebtoken::encode(
                &Header::default(),
                &payload,
                &EncodingKey::from_secret(&KEY),
            )
            .unwrap();

            match sesion.insert(auth_data.username.clone(), payload) {
                Ok(_) => {
                    return Ok(HttpResponse::Ok().json(TokenRes {
                        token: token,
                        token_type: "bearer".to_owned(),
                    }));
                }
                Err(_) => Ok(HttpResponse::InternalServerError().json(ClientErrResponse {
                    error_details: "Failed to generate token".to_owned(),
                })),
            }
        }
        Ok(false) => Ok(HttpResponse::Unauthorized().json(ClientErrResponse {
            error_details: "Failed to authenticate user".to_owned(),
        })),
        Err(e) => match e {
            UserMgmtError::UserNotFoundError => {
                Ok(HttpResponse::Unauthorized().json(ClientErrResponse {
                    error_details: "Failed to authenticate user".to_owned(),
                }))
            }
            _ => Ok(HttpResponse::InternalServerError().json(ClientErrResponse {
                error_details: "Failed accessing internal db".to_owned(),
            })),
        },
    }
}

pub async fn logout(
    token: web::Json<UserToken>,
    session: Session,
) -> actix_web::Result<impl Responder> {
    session.remove(&token.user);
    Ok(HttpResponse::Ok().finish())
}

pub async fn test_auth(
    token: web::Json<String>,
    session: Session,
) -> actix_web::Result<impl Responder> {
    if verify_token(token.0, &session) {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}
