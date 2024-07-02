use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use log::{error, warn};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use crate::db::models;
use crate::db::schema;

pub enum UserMgmtError {
    UnknownError,
    UsernameTakenError,
    UserNotFoundError,
}

pub fn create_user(
    db_con: &mut PgConnection,
    username: &str,
    password: &str,
) -> Result<(), UserMgmtError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Error hashing password")
        .to_string();

    let new_user = models::NewUser {
        username: username,
        password: password_hash.as_str(),
        salt: salt.as_str(),
    };

    match diesel::insert_into(schema::users::dsl::users)
        .values(new_user)
        .execute(db_con)
    {
        Ok(1) => Ok(()),
        Ok(_) => {
            // TODO: Check if the user still got added
            error!("Unexpected number of rows affected when inserting user");
            Err(UserMgmtError::UnknownError)
        }
        Err(err) => match err {
            DieselError::DatabaseError(err_kind, _) => match err_kind {
                DatabaseErrorKind::UniqueViolation => Err(UserMgmtError::UsernameTakenError),
                _ => Err(UserMgmtError::UnknownError),
            },
            _ => Err(UserMgmtError::UnknownError),
        },
    }
}

pub fn authenticate_user(
    db_con: &mut PgConnection,
    username: &str,
    password: &str,
) -> Result<bool, UserMgmtError> {
    use schema::users::dsl;

    let user_search: Vec<models::User> = match dsl::users
        .filter(dsl::username.eq(username))
        .limit(2)
        .select(models::User::as_select())
        .load(db_con)
    {
        Ok(res) => res,
        Err(_) => return Err(UserMgmtError::UserNotFoundError),
    };

    // This is extremely problematic if this occurs
    if user_search.len() > 1 {
        return Err(UserMgmtError::UnknownError);
    }

    let argon2 = Argon2::default();

    let pw_hash_str = user_search[0].password.as_str();
    let pw_salt = user_search[0].salt.as_str();

    let challenge = argon2
        .hash_password(
            password.as_bytes(),
            &SaltString::from_b64(pw_salt).expect("Error parsing salt"),
        )
        .expect("Error hashing challenge")
        .to_string();

    Ok(challenge.as_str() != pw_hash_str)
}

pub fn delete_user(db_con: &mut PgConnection, username: &str) -> Result<(), UserMgmtError> {
    use schema::users::dsl;

    // TODO: We actually need to delete information associated with this user
    let num_deleted =
        match diesel::delete(dsl::users.filter(dsl::username.eq(username))).execute(db_con) {
            Ok(n) => n,
            Err(_) => return Err(UserMgmtError::UnknownError),
        };

    if num_deleted > 1 {
        warn!("To many users impacted by delete");
    }

    Ok(())
}
