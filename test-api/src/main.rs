mod calls;
mod models;

use crate::models::{Diploma, NewRequest};
use chrono::NaiveDate;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let base_url = "http://localhost:5000/";
    calls::check_api(base_url).await;

    let new_request_nb = models::NewRequest {
        name: "Nikola Bubić".into(),
        birthday: NaiveDate::from_ymd(1996, 12, 31),
        diplomas: vec![
            Diploma {
                title: "Electrical Engineer".into(),
                schools: "University of Belgrade".into(),
                from: NaiveDate::from_ymd(2015, 9, 1),
                to: Some(NaiveDate::from_ymd(2024, 3, 15)),
                ongoing: false,
            },
        ],
    };

    let new_request_grga = models::NewRequest {
        name: "Grga Taksista".into(),
        birthday: NaiveDate::from_ymd(1996, 12, 31),
        diplomas: vec![
            Diploma {
                title: "Driver".into(),
                schools: "Driving school".into(),
                from: NaiveDate::from_ymd(2010, 9, 1),
                to: Some(NaiveDate::from_ymd(2011, 3, 15)),
                ongoing: false,
            },
        ],
    };

    if let Err(e) = calls::new_request(base_url, new_request_nb).await {
        eprintln!("Error making request: {}", e);
    }

    if let Err(e) = calls::new_request(base_url, new_request_grga).await {
        eprintln!("Error making request: {}", e);
    }

    let userId: Uuid = match calls::get_onboarding_list(base_url, 1, 5).await {
        Ok(list) => {
            list[0].id
        },
        Err(e) => {
            panic!("Error making request: {}", e);
            Uuid::new_v4()
        }
    };

    if let Err(e) = calls::get_onboarding_list(base_url, 0, 5).await {
        eprintln!("Error making request: {}", e);
    }

    if let Err(e) = calls::add_user(base_url, userId).await {
        eprintln!("Error making request: {}", e);
    }

    if let Err(e) = calls::get_user_details(base_url, "grga.taksista", "!Avaya123").await {
        eprintln!("Error making request: {}", e);
    }

}