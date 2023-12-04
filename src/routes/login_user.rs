use argon2::{self, Config};
use chrono::Utc;
use rand::Rng;

use crate::models::session::Session;
use crate::models::users::UserId;




pub fn verify_token(token: String) -> Result<Session, handle_errors::Error> {
    let token = paseto::tokens::validate_local_token(
        &token,
         None,
         &"Token has to be exactly 32 bytes".as_bytes(),
        &paseto::tokens::TimeBackend::Chrono
    )
    .map_err(|_| handle_errors::Error::CannotDecryptToken)?;

    serde_json::from_value::<Session>(token).map_err(|_| handle_errors::Error::CannotDecryptToken)
}

fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(&password, &salt, &config).unwrap()
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(user_id: UserId) -> String {
    let current_date_time = Utc::now();
    // token expires in 5 hours
    let expired_dt = current_date_time + chrono::Duration::hours(5);

    // continue from here
    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from("Token has to be exactly 32 bytes".as_bytes()))
        .set_expiration(&expired_dt)
        .set_not_before(&Utc::now())
        .set_claim("user_id", serde_json::json!(user_id))
        .set_subject("Access granted!")
        .build()
        .expect("Failed to construct paseto token w/ builder!")
}