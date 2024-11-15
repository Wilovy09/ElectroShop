use crate::helpers::{
    hash_passwords::{hash_password, verify_password},
    validate_password::is_valid_password,
};
use crate::middlewares::jwt::{generate_token, validate_token, TokenStruct};
use crate::models::PartialUser;
use crate::params::user::AuthUser;
use crate::responses::message::Messages;
use crate::AppState;
use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse,
};
// use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx;
use std::borrow::Cow;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user).service(login_user);
}

#[post("/register")]
async fn create_user(state: Data<AppState>, body: Json<AuthUser>) -> HttpResponse {
    // TODO: Por alguna razón dejo de chequear la contraseña.
    if !is_valid_password(&body.password) {
        return HttpResponse::BadRequest().json(Messages {
            message: "Password must be at least 8 characters long, contain at least one uppercase letter, and one special character.".to_string(),
        });
    }
    let hashed_password = match hash_password(body.password.to_owned()) {
        Ok(hp) => hp,
        Err(e) => {
            return HttpResponse::InternalServerError().json(Messages {
                message: format!("Error hashing password: {e}"),
            });
        }
    };
    let email = body.email.clone();
    // Change to "Administrador" to create administrator account.
    let role = "Cliente".to_string();
    match sqlx::query!(
        "INSERT INTO User (`email`, `password`, role) VALUES ($1,$2,$3) RETURNING id",
        email,
        hashed_password,
        role
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => {
            let iss = "ElectroShop";
            let duration_in_minutes: i64 = 525600;
            let user_id = user.id as usize;

            let token = generate_token(
                iss.to_string(),
                duration_in_minutes,
                "token".to_owned(),
                user_id,
            );

            let result = validate_token(token.clone());
            match result {
                Ok(_) => HttpResponse::Ok().json(TokenStruct {
                    token: token.to_string(),
                }),
                Err(_) => HttpResponse::Unauthorized().json(Messages {
                    message: "Invalid token".to_string(),
                }),
            }
        }
        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code() == Some(Cow::Borrowed("2067")) {
                    return HttpResponse::BadRequest().json(Messages {
                        message: "The email is already registered".to_string(),
                    });
                }
            }
            eprintln!("Error: {:?}", e);
            HttpResponse::InternalServerError().json(Messages {
                message: "Error creating user".to_string(),
            })
        }
    }
}

#[post("/login")]
async fn login_user(state: Data<AppState>, body: Json<AuthUser>) -> HttpResponse {
    match sqlx::query_as::<_, PartialUser>("SELECT * FROM User WHERE email = $1")
        .bind(&body.email)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => {
            let iss = "ElectroShop";
            let duration_in_minutes: i64 = 525600;
            let user_id = user.id;

            if verify_password(body.password.clone(), user.password) {
                let token = generate_token(
                    iss.to_string(),
                    duration_in_minutes,
                    "token".to_owned(),
                    user_id as usize,
                );
                let result = validate_token(token.clone());
                match result {
                    Ok(_) => HttpResponse::Ok().json(TokenStruct {
                        token: token.to_string(),
                    }),
                    Err(_) => HttpResponse::Unauthorized().json(Messages {
                        message: "Invalid token".to_string(),
                    }),
                }
            } else {
                HttpResponse::Unauthorized().json(Messages {
                    message: "Invalid credentials.".to_string(),
                })
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            HttpResponse::Unauthorized().json(Messages {
                message: "Invalid credentials.".to_string(),
            })
        }
    }
}
