extern crate diesel;

use crate::error_handlers::ServiceError;
use crate::{db, schema::*};
use crate::{Insertable, Queryable};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{insert_into, AsChangeset, QueryDsl, RunQueryDsl};
use serde_derive::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable, Validate)]
#[diesel(table_name = employees)]
pub struct Employee {
    #[validate(length(min = 1, max = 120))]
    pub first_name: String,
    #[validate(length(min = 1, max = 120))]
    pub last_name: Option<String>,
    #[validate(length(min = 1, max = 120))]
    pub department: String,
    pub salary: i32,
    pub age: i16,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Employees {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub department: String,
    pub salary: i32,
    pub age: i16,
    pub created_at: NaiveDateTime,
}

impl Employees {
    pub fn find_all() -> Result<Vec<Self>, ServiceError> {
        let mut conn = db::connection()?;
        employees::table
            .load::<Employees>(&mut conn)
            .map_err(|e| ServiceError::InternalServerError)
    }
    pub fn create(employee: Employee) -> Result<Self, ServiceError> {
        let mut conn = db::connection()?;
        let data = Employee::from(employee);
        let res = insert_into(employees::table)
            .values(&data)
            .get_result(&mut conn)
            .map_err(|e| ServiceError::InternalServerError);
        res
    }
    pub fn find_by_id(id: i32) -> Result<Self, ServiceError> {
        let mut conn = db::connection()?;
        employees::table
            .filter(employees::id.eq(id))
            .first::<Employees>(&mut conn)
            .map_err(|e| ServiceError::NotFound {
                message: "Employee not found".to_string(),
            })
    }
}

impl Employee {
    pub fn from(employee: Employee) -> Employee {
        Employee {
            first_name: employee.first_name,
            last_name: employee.last_name,
            department: employee.department,
            salary: employee.salary,
            age: employee.age,
        }
    }
}
