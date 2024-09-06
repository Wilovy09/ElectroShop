use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenStruct {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub user_id: usize,
}

pub fn get_secret_key() -> String {
    dotenv().ok();
    env::var("SECRET_KEY").expect("SECRET_KEY must be set")
}

pub fn generate_token(iss: String, sub: String, duration_minutes: i64, user_id: usize) -> String {
    let header = Header::new(Algorithm::HS512);
    let encoding_key = EncodingKey::from_secret(get_secret_key().as_bytes());

    let exp = (Utc::now() + Duration::minutes(duration_minutes)).timestamp() as usize;
    let iat = Utc::now().timestamp() as usize;

    let my_claims = Claims {
        iss,
        sub,
        exp,
        iat,
        user_id,
    };

    encode(&header, &my_claims, &encoding_key).unwrap()
}

pub fn validate_token(token: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS512);
    let decoding_key = DecodingKey::from_secret(get_secret_key().as_bytes());
    let result = decode::<Claims>(&token, &decoding_key, &validation);
    match result {
        Ok(c) => Ok(c.claims),
        Err(e) => Err(e)
    }
}
