use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::response::Response;

pub type Users = Response<Person>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub age: i32,
    pub height: i32,
}

impl Person {
    pub fn new(name: String, age: i32, height: i32) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            name: name,
            age: age,
            height: height,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonRequest {
    pub name: Option<String>,
    pub age: Option<i32>,
    pub height: Option<i32>,
}

impl PersonRequest {
    pub fn create_person(&self) -> Option<Person> {
        let name: String;
        let age: i32;
        let height: i32;

        match &self.name {
            Some(new_name) => name = new_name.to_owned(),
            None => name = "".to_owned()
        }

        match &self.age {
            Some(new_age) => age = *new_age,
            None => age = 0
        }

        match &self.height {
            Some(new_height) => height = *new_height,
            None => height = 0
        }

        Some(Person::new(name, age, height))
    }
}

#[get("/users")]
pub async fn get_users() -> HttpResponse {
    let users = Users { results: vec![] };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(users)
}

#[post("/users")]
pub async fn create_user(user_req: Json<PersonRequest>) -> HttpResponse {
    HttpResponse::Created()
        .content_type("application/json")
        .json(user_req.create_person())
}