use crate::{
    models::{CreateRole, DeleteRoleParams, Role},
    responses::message::Messages,
    AppState,
};
use actix_web::{
    delete, post,
    web::{self, Data, Json},
    HttpResponse,
};
use sqlx;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_role).service(delete_rol);
}

#[post("/roles")]
async fn create_role(state: Data<AppState>, body: Json<CreateRole>) -> HttpResponse {
    let message = Messages {
        message: "Error al crear Rol".to_string(),
    };
    match sqlx::query_as::<_, Role>(
        "INSERT INTO Role (name_role) VALUES ($1) RETURNING id, name_role",
    )
    .bind(&body.name_role)
    .fetch_one(&state.db)
    .await
    {
        Ok(role) => HttpResponse::Created().json(role),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            HttpResponse::InternalServerError().json(message)
        }
    }
}

#[delete("/roles/{id}")]
async fn delete_rol(state: Data<AppState>, params: web::Path<DeleteRoleParams>) -> HttpResponse {
    let err_message = Messages {
        message: "Error al eliminar Rol".to_string(),
    };
    let not_found_message = Messages {
        message: "Rol no encontrado".to_string(),
    };
    let success_message = Messages {
        message: "Rol eliminado correctamente".to_string(),
    };

    match sqlx::query("DELETE FROM Role WHERE id = $1")
        .bind(params.id)
        .execute(&state.db)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().json(success_message)
            } else {
                HttpResponse::NotFound().json(not_found_message)
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            HttpResponse::InternalServerError().json(err_message)
        }
    }
}

