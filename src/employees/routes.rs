use crate::employees::{Employee, Employees};
use actix_web::{get, post, web, HttpResponse, Result};
use actix_web_validator::Json;
use validator::Validate;

use crate::error_handlers::ServiceError;

#[get("/employees")]
async fn find_all() -> Result<HttpResponse, ServiceError> {
    let employees = web::block(|| Employees::find_all()).await.unwrap()?;
    Ok(HttpResponse::Ok().json(employees))
}

#[post("/employees")]
async fn create(item: Json<Employee>) -> Result<HttpResponse, ServiceError> {
    match item.validate() {
        Ok(_) => {
            let employee = web::block(|| Employees::create(item.into_inner()))
                .await
                .unwrap()?;
            Ok(HttpResponse::Created().json(employee))
        }
        Err(err) => Err(ServiceError::BadRequest {
            message: err.to_string(),
        }),
    }
}

#[get("/employees/{id}")]
async fn find_one(id: web::Path<i32>) -> Result<HttpResponse, ServiceError> {
    let employee_id = id.into_inner();
    let employee = web::block(move || Employees::find_by_id(employee_id))
        .await
        .unwrap()?;
    Ok(HttpResponse::Ok().json(employee))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(create);
    config.service(find_one);
}
