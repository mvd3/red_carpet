use axum::{
    extract::{Json, Extension, Query, Path},
    http::StatusCode,
    response::IntoResponse,
};
use crate::models::{Request, Diploma, User};
use crate::database::Database;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use rand::{distributions::Alphanumeric, Rng};
use rand::distributions::Uniform;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub async fn check_handler() -> &'static str {
    println!("Received a request to /check");
    "PFK"
}

pub async fn create_request_handler(
    Extension(database): Extension<Arc<Mutex<Database>>>,
    Json(payload): Json<NewRequest>,
) -> impl IntoResponse {
    println!("Received a request to /createRequest");

    let new_request = Request {
        id: Uuid::new_v4(),
        name: payload.name,
        birthday: payload.birthday,
        diplomas: payload.diplomas,
    };

    let mut db = database.lock().unwrap();
    db.add_request(new_request);

    (StatusCode::CREATED, "Request created")
}

pub async fn onboarding_list_handler(
    Query(pagination): Query<Pagination>,
    Extension(database): Extension<Arc<Mutex<Database>>>,
) -> impl IntoResponse {
    println!("Received a request to /onboardingList");

    let db = database.lock().unwrap();
    let requests = db.get_requests(pagination.skip.unwrap_or(0), pagination.size.unwrap_or(10)); // Default to 0 skip and 10 size

    Json(requests)
}

pub async fn add_user_handler(
    Path(id): Path<Uuid>,
    Extension(database): Extension<Arc<Mutex<Database>>>,
) -> impl IntoResponse {
    println!("Received a request to /addUser with id: {}", id);

    let mut db = database.lock().unwrap();

    let hashed_password = match hash_password(generate_password().as_bytes()) {
        Ok(hash) => hash,
        Err(e) => {
            panic!("Password hashing failed: {}", e);
        }
    };

    if let Some(req) = db.get_request(&id) {
        let user = User {
            id: Uuid::new_v4(),
            name: req.name.clone(),
            username: generate_username(&req.name, &db),
            password: hashed_password,
            birthday: req.birthday,
            diplomas: req.diplomas.clone(),
        };

        println!("{}", user.username);

        db.add_user(user);

        (StatusCode::CREATED, "User added")
    } else {
        (StatusCode::NOT_FOUND, "Request not found")
    }
}

pub async fn user_details_handler(
    //Query(user_details): Query<UserDetailsRequest>,
    Extension(database): Extension<Arc<Mutex<Database>>>,
    Json(user_details): Json<UserDetailsRequest>,
) -> Json<Option<UserDetailsResponse>> {
    println!("Received a request to /userDetails");

    let db = database.lock().unwrap();

    println!("{}, {}", user_details.username, user_details.password);

    if let Some(user) = db.get_user_by_username(&user_details.username) {
        if verify_password(&user.password, user_details.password.as_bytes()).is_ok() {
            return Json(Some(UserDetailsResponse {
                name: user.name.clone(),
                birthday: user.birthday.to_string(),
                diplomas: user.diplomas.clone(),
            }));
        }
    }

    Json(None)
}

// Private methods

fn generate_password() -> String {
    let mut rng = rand::thread_rng();
    
    let mut password: Vec<char> = (0..6)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    
    let uppercase_pos = rng.gen_range(1..password.len());
    password.insert(uppercase_pos, rng.sample(Uniform::new_inclusive('A', 'Z')));
    
    let digit_pos = rng.gen_range(1..password.len() + 1);
    password.insert(digit_pos, rng.sample(Uniform::new_inclusive('0', '9')));
    
    let special_characters = "!@#$%^&*";
    let special_pos = rng.gen_range(1..password.len());
    password.insert(special_pos, special_characters.chars().nth(rng.gen_range(0..special_characters.len())).unwrap());

    //password.into_iter().collect()
    "!Avaya123".to_string() // For testing purposes only, else remove this line and uncomment the previous
}

fn generate_username(name: &str, db: &Database) -> String {
    let base_username = name.to_lowercase().replace(" ", ".");
    let mut username = base_username.clone();
    let mut suffix = 1;
    while db.username_exists(|user| user.username == username) {
        username = format!("{}{}", base_username, suffix);
        suffix += 1;
    }
    username
}

fn hash_password(password: &[u8]) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt)?.to_string();

    Ok(password_hash)
}

fn verify_password(stored_hash: &str, attempted_password: &[u8]) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(stored_hash)?;
    let argon2 = Argon2::default();

    match argon2.verify_password(attempted_password, &parsed_hash) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

// Utility structures

#[derive(Deserialize)]
pub struct NewRequest {
    name: String,
    birthday: NaiveDate,
    diplomas: Vec<Diploma>,
}

#[derive(Deserialize)]
pub struct Pagination {
    skip: Option<usize>,
    size: Option<usize>,
}

#[derive(Deserialize)]
pub struct UserDetailsRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserDetailsResponse {
    name: String,
    birthday: String,
    diplomas: Vec<Diploma>,
}