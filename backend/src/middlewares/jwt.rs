use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use crate::AppState;
use actix_web::{dev::ServiceRequest,error, Error};
use actix_web::web::Data;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::Error as SqlxError;
use actix_web_grants::authorities::AttachAuthorities;

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenStruct {
    pub token: String,
    pub refresh: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshStruct {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String,
    pub user_id: i64,
}

pub fn get_secret_key() -> String {
    dotenv().ok();
    env::var("SECRET_KEY").expect("SECRET_KEY must be set")
}

pub fn generate_token(
    iss: String,
    sub: String,
    duration_minutes: i64,
    token_type: String,
    user_id: usize,
) -> String {
    let header = Header::new(Algorithm::HS512);
    let encoding_key = EncodingKey::from_secret(get_secret_key().as_bytes());
    let exp = (Utc::now() + Duration::minutes(duration_minutes)).timestamp() as usize;
    let iat = Utc::now().timestamp() as usize;
    let my_claims = Claims {
        iss,
        sub,
        exp,
        iat,
        token_type,
        user_id: user_id as i64,
    };
    encode(&header, &my_claims, &encoding_key).unwrap()
}

pub fn validate_token(token: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS512);
    let decoding_key = DecodingKey::from_secret(get_secret_key().as_bytes());
    let result = decode::<Claims>(&token, &decoding_key, &validation);
    match result {
        Ok(c) => Ok(c.claims),
        Err(e) => Err(e),
    }
}

pub async fn validator(
    req: ServiceRequest,
    credenciales: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(credenciales) = credenciales else {
        return Err((error::ErrorBadRequest("No se especificó el token"), req));
    };
    let token = credenciales.token();
    let state = match req.app_data::<Data<AppState>>() {
        Some(data) => data,
        None => return Err((error::ErrorInternalServerError("No se pudo obtener el estado."), req)),
    };
    match validate_token(token.to_owned()) {
        Ok(token) => {
            if token.token_type == "refresh" {
                return Err((error::ErrorUnauthorized("Token inválido."), req));
            }
            match sqlx::query!("SELECT role_id FROM User WHERE id = $1", token.user_id)
                .fetch_one(&state.db)
                .await
            {
                Ok(record) => {
                    let role_id = record.role_id;
                    match role_id {
                        1 => req.attach(vec!["Administrador".to_string()]),
                        2 => req.attach(vec!["Cliente".to_string()]),
                        _ => req.attach(vec!["Usuario".to_string()]),
                    }
                    Ok(req)
                }
                Err(SqlxError::RowNotFound) => {
                    Err((error::ErrorNotFound("Usuario no encontrado."), req))
                }
                Err(_) => {
                    Err((
                        error::ErrorInternalServerError("Error al consultar la base de datos."),
                        req,
                    ))
                }
            }
        }
        Err(_) => {
            Err((error::ErrorForbidden("No tiene acceso."), req))
        }
    }
}
