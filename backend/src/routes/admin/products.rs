use crate::{
	models::product::{Product, CreateProduct},
	params::product::PartialProductParams,
	responses::message::Messages,
	AppState,
};
use actix_web::{
	delete, post, put,
	web::{self, Data, Json},
	HttpResponse,
};
use actix_web_grants::protect;

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(create).service(delete).service(edit);
}

#[post("/product")]
#[protect("Administrador")]
async fn create(state: Data<AppState>, body: Json<CreateProduct>) -> HttpResponse {
	let category_id = &body.category_id;
	let name = &body.name;
	let image = &body.image;
	let description = &body.description;
	let price = &body.price;
	let units = &body.units;
	let deleted = &body.deleted;

	match sqlx::query_as!(
		CreateProduct,
		"INSERT INTO Product (category_id, name, image, description, price, units, deleted) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING category_id, name, image, description, price, units, deleted",
		category_id,
		name,
		image,
		description,
		price,
		units,
		deleted
	)
	.fetch_one(&state.db)
	.await
	{
		Ok(product) => HttpResponse::Ok().json(product),
		Err(_) => HttpResponse::Unauthorized().json(Messages{
			message: "Invalid token.".to_string()
		})
	}
}


#[delete("/products/{id}")]
#[protect("Administrador")]
async fn delete(state: Data<AppState>, params: web::Path<PartialProductParams>) -> HttpResponse {
    let id = &params.id;
    match sqlx::query_as!(
		Product,
		"DELETE FROM Product WHERE id = $1 RETURNING *",
		id
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::Unauthorized().json(Messages {
            message: "Invalid token.".to_string(),
        }),
    }
}

#[put("/products/{id}")]
#[protect("Administrador")]
async fn edit(
    state: Data<AppState>,
    params: web::Path<PartialProductParams>,
    body: Json<CreateProduct>,
) -> HttpResponse {
    let category_id = &body.category_id;
    let name = &body.name;
    let image = &body.image;
    let price = &body.price;
    let units = &body.units;
    let deleted = &body.deleted;
    let description = &body.description;
    let id = params.id;

    match sqlx::query_as!(
		Product,
		"UPDATE Product SET category_id = $1, name = $2, image = $3, price = $4, units = $5, deleted = $6, description = $7 WHERE id = $8 RETURNING *", 
		category_id, 
		name, 
		image,
		price, 
		units,
		deleted,
		description,
		id
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(Messages {
            message: "Product not found.".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(Messages {
            message: "An error occurred while updating the product.".to_string(),
        }),
    }
}
