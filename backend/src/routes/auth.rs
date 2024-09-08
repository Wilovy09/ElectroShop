use crate::helpers::{
    hash_passwords::{hash_password, verify_password},
    validate_password::is_valid_password,
};
use crate::middlewares::jwt::{generate_token, validate_token, RefreshStruct, TokenStruct};
use crate::models::PartialUser;
use crate::params::user::AuthUser;
use crate::{responses::message::ErrorMessages, AppState};
use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx;
use std::borrow::Cow;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(refresh)
        .service(create_user)
        .service(login_user);
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
    let hashed_password = match hash_password(body.password.to_owned()) {
        Ok(hp) => hp,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorMessages {
                error_code: 500,
                message: format!("Error hashing password: {e}"),
            });
        }
    };
    let email = body.email.clone();
    match sqlx::query_as!(PartialUser, "INSERT INTO User (`email`, `password`, role_id) VALUES ($1,$2,$3) RETURNING id, role_id, `email`, `password`", email, hashed_password, 2)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => {
            let iss = "ElectroShop";
            let sub = "Client";
            let duration_in_minutes: i64 = 5;
            let refresh_token_duration_in_minutes: i64 = 525600;
            if let Some(user_id) = user.id {
                let user_id_usize = user_id as usize;
                let token = generate_token(
                    iss.to_string(),
                    sub.to_string(),
                    duration_in_minutes,
                    "token".to_owned(),
                    user_id_usize, 
                );
                let refresh_jwt = generate_token(
                    iss.to_string(),
                    sub.to_string(),
                    refresh_token_duration_in_minutes,
                    "refresh".to_owned(),
                    user_id_usize,
                );
                let result = validate_token(token.clone());
                match result {
                    Ok(_) => HttpResponse::Ok().json(TokenStruct {
                        token: token.to_string(),
                        refresh: refresh_jwt.to_string(),
                    }),
                    Err(_) => HttpResponse::Unauthorized().json(ErrorMessages {
                        error_code: 401,
                        message: "Invalid token".to_string(),
                    }),
                }
            } else {
                HttpResponse::InternalServerError().json(ErrorMessages {
                    error_code: 500,
                    message: "User ID is null".to_string(),
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
    match sqlx::query_as::<_, PartialUser>("SELECT * FROM User WHERE email = $1")
        .bind(&body.email)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => {
            let iss = "ElectroShop";
            let sub = "Client";
            let duration_in_minutes: i64 = 5;
            let refresh_token_duration_in_minutes: i64 = 525600;
            if let Some(user_id) = user.id {
                if verify_password(body.password.clone(), user.password) {
                let user_id_usize = user_id as usize;
                let token = generate_token(
                    iss.to_string(),
                    sub.to_string(),
                    duration_in_minutes,
                    "token".to_owned(),
                    user_id_usize,
                );
                let refresh_jwt = generate_token(
                    iss.to_string(),
                    sub.to_string(),
                    refresh_token_duration_in_minutes,
                    "refresh".to_owned(),
                    user_id_usize,
                );
                let result = validate_token(token.clone());
                match result {
                    Ok(_) => HttpResponse::Ok().json(TokenStruct {
                        token: token.to_string(),
                        refresh: refresh_jwt.to_string(),
                    }),
                    Err(_) => HttpResponse::Unauthorized().json(ErrorMessages {
                        error_code: 401,
                        message: "Invalid token".to_string(),
                    }),
                }
            } else {
                HttpResponse::Unauthorized().json(ErrorMessages {
                    error_code: 401,
                    message: "Invalid credentials.".to_string(),
                })
            }
            } else {
                HttpResponse::InternalServerError().json(ErrorMessages {
                    error_code: 500,
                    message: "User ID is null".to_string(),
                })
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

#[post("/refresh-token")]
async fn refresh(refresh_jwt: Option<BearerAuth>) -> HttpResponse {
    let Some(refresh_jwt) = refresh_jwt else {
        return HttpResponse::Forbidden().json(ErrorMessages {
            error_code: 403,
            message: "Invalid token".to_string(),
        });
    };
    let claims = validate_token(refresh_jwt.token().to_owned());
    match claims {
        Ok(c) => {
            if c.token_type == "refresh" {
                let iss = "ElectroShop";
                let sub = "Client";
                let duration_in_minutes: i64 = 5;
                
                if let Some(user_id) = c.user_id {
                    let user_id_usize = user_id as usize;
                    let result = generate_token(
                        iss.to_string(),
                        sub.to_string(),
                        duration_in_minutes,
                        "token".to_owned(),
                        user_id_usize,
                    );
                    HttpResponse::Ok().json(RefreshStruct {
                        token: result.to_string(),
                    })
                } else {
                    HttpResponse::InternalServerError().json(ErrorMessages {
                        error_code: 500,
                        message: "User ID is missing.".to_string(),
                    })
                }
            } else {
                HttpResponse::Unauthorized().json(ErrorMessages {
                    error_code: 401,
                    message: "Invalid token.".to_string(),
                })
            }
        }
        Err(_) => HttpResponse::Unauthorized().json(ErrorMessages {
            error_code: 401,
            message: "Invalid token.".to_string(),
        }),
    }
}

