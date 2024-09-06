use crate::helpers::validate_password::is_valid_password;
use crate::middlewares::jwt::{generate_token, validate_token, TokenStruct};
use crate::models::PartialUser;
use crate::params::user::AuthUser;
use crate::{responses::message::ErrorMessages, AppState};
use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse,
};
use sqlx;
use std::borrow::Cow;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user).service(login_user);
}

#[post("/user")]
async fn create_user(state: Data<AppState>, body: Json<AuthUser>) -> HttpResponse {
    // TODO: Por alguna razón dejo de chequear la contraseña.
    if !is_valid_password(&body.password) {
        return HttpResponse::BadRequest().json(ErrorMessages {
            error_code: 400,
            message: "Password must be at least 8 characters long, contain at least one uppercase letter, and one special character.".to_string(),
        });
    }

    match sqlx::query_as::<_, PartialUser>(
        "INSERT INTO User (email, password, role_id) VALUES ($1, $2, $3) RETURNING id, role_id, email",
    )
    .bind(&body.email)
    .bind(&body.password)
    .bind(2) // 1 = Administrator, 2 = User 
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => {
            let iss = "ElectroShop";
            let sub = "Client";
            let duration_in_minutes: i64 = 525600;
            let user_id = user.id;

            let token = generate_token(iss.to_string(), sub.to_string(), duration_in_minutes, user_id as usize);
            let result = validate_token(token.clone());
            match result {
                Ok(_) => HttpResponse::Ok().json(TokenStruct{
                    token: token.to_string()
                }),
                Err(_) => HttpResponse::Unauthorized().json(ErrorMessages{
                    error_code: 401,
                    message: "Invalid token".to_string(),
                })
            }
        }
        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code() == Some(Cow::Borrowed("2067")) {
                    return HttpResponse::BadRequest().json(ErrorMessages {
                        error_code: 400,
                        message: "The email is already registered".to_string(),
                    });
                }
            }
            eprintln!("Error: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorMessages {
                error_code: 500,
                message: "Error creating user".to_string(),
            })
        }
    }
}

#[post("/login")]
async fn login_user(state: Data<AppState>, body: Json<AuthUser>) -> HttpResponse {
    match sqlx::query_as::<_, PartialUser>(
        "SELECT * FROM User WHERE email = $1 and password = $2",
    )
    .bind(&body.email)
    .bind(&body.password)
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => {
            let iss = "ElectroShop";
            let sub = "Client";
            let duration_in_minutes: i64 = 525600;
            let user_id = user.id;

            let token = generate_token(
                iss.to_string(),
                sub.to_string(),
                duration_in_minutes,
                user_id as usize,
            );
            let result = validate_token(token.clone());
            match result {
                Ok(_) => HttpResponse::Ok().json(TokenStruct {
                    token: token.to_string(),
                }),
                Err(_) => HttpResponse::Unauthorized().json(ErrorMessages {
                    error_code: 401,
                    message: "Invalid token".to_string(),
                }),
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            HttpResponse::Unauthorized().json(ErrorMessages {
                error_code: 401,
                message: "Invalid credentials.".to_string(),
            })
        }
    }
}
