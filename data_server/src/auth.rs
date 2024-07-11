use actix_session::Session;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
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
pub struct AuthorizedData<T> {
    user: String,
    token: String,
    data: T,
}

impl<T> AuthorizedData<T> {
    pub fn get_verified_request(&self, session: &Session) -> Result<&T, ()> {
        if verify_token(self.user.clone(), self.token.clone(), session) {
            Ok(&self.data)
        } else {
            Err(())
        }
    }

    pub fn get_user(&self) -> String {
        self.user.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub sub: String,
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

pub fn verify_token(user: String, token: String, session: &Session) -> bool {
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.sub = Some(user);
    match jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(&KEY), &validation) {
        Ok(_) => true,
        Err(_) => false,
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
                sub: auth_data.username.clone(),
            };
            let token = jsonwebtoken::encode(
                &Header::default(),
                &payload,
                &EncodingKey::from_secret(&KEY),
            )
            .unwrap();

            debug!("Return token: {}", token);

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
        Ok(false) => {
            debug!("User {} login attempt rejected", auth_data.username);
            Ok(HttpResponse::Unauthorized().json(ClientErrResponse {
                error_details: "Failed to authenticate user".to_owned(),
            }))
        }
        Err(e) => match e {
            UserMgmtError::UserNotFoundError => {
                debug!("Unknown user {} attempted to log in", auth_data.username);
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
    token: web::Json<String>,
    session: Session,
) -> actix_web::Result<impl Responder> {
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    let val = match jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &validation,
    ) {
        Ok(e) => e,
        Err(_) => {
            return Ok(HttpResponse::Ok().finish());
        }
    };

    session.remove(&val.claims.sub);
    Ok(HttpResponse::Ok().finish())
}

#[get("/auth/{user}/{token}")]
pub async fn test_auth(
    req: HttpRequest,
    // token: web::Json<String>,
    session: Session,
) -> actix_web::Result<impl Responder> {
    let user: String = req.match_info().get("user").unwrap().parse().unwrap();
    let token: String = req.match_info().get("token").unwrap().parse().unwrap();

    if verify_token(user, token, &session) {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}
