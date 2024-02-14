use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Diploma {
    pub title: String,
    pub schools: String,
    pub from: NaiveDate,
    pub to: Option<NaiveDate>,
    pub ongoing: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub id: Uuid,
    pub name: String,
    pub birthday: NaiveDate,
    pub diplomas: Vec<Diploma>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub password: String,
    pub birthday: NaiveDate,
    pub diplomas: Vec<Diploma>,
}