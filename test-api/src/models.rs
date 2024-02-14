use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Diploma {
    pub title: String,
    pub schools: String,
    pub from: NaiveDate,
    pub to: Option<NaiveDate>,
    pub ongoing: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRequest {
    pub name: String,
    pub birthday: NaiveDate,
    pub diplomas: Vec<Diploma>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub id: Uuid,
    pub name: String,
    pub birthday: NaiveDate,
    pub diplomas: Vec<Diploma>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDetailsResponse {
    pub name: String,
    pub birthday: String,
    pub diplomas: Vec<Diploma>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDetailsRequest {
    pub username: String,
    pub password: String,
}