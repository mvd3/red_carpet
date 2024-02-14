use crate::models::{Diploma, NewRequest, Request, UserDetailsRequest, UserDetailsResponse};
use reqwest::Response;
use reqwest::{Client, Error};
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

pub async fn check_api(base_url: &str) {
    let url = format!("{}check", base_url);
    match reqwest::get(&url).await {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.text().await {
                    Ok(text) => println!("Response from /check: {}", text),
                    Err(e) => println!("Error reading response text: {}", e),
                }
            } else {
                println!("Non-success response: {}", resp.status());
            }
        }
        Err(e) => println!("Failed to make request: {}", e),
    }
}

pub async fn new_request(base_url: &str, new_request: NewRequest) -> Result<(), Error> {
    let client = Client::new();
    let url = format!("{}createRequest", base_url);

    let res = client.post(&url)
        .json(&new_request)
        .send()
        .await?;

    if res.status().is_success() {
        println!("Successfully created a new request.");
    } else {
        println!("Failed to create a new request. Status: {}", res.status());
    }

    Ok(())
}

pub async fn get_onboarding_list(base_url: &str, skip: usize, size: usize) -> Result<(Vec<Request>), Error> {
    let client = Client::new();
    let url = format!("{}onboardingList?skip={}&size={}", base_url, skip, size);

    let res: Response = client.get(&url)
        .send()
        .await?;

    let mut list_content: Vec<Request> = Vec::new();

    if res.status().is_success() {
        list_content = res.json().await?;
        println!("List content: {:?}", list_content);
    } else {
        println!("Failed to get the onboarding list. Status: {}", res.status());
    }

    Ok(list_content)
}

pub async fn add_user(base_url: &str, user_id: Uuid) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let url = format!("{}addUser/{}", base_url, user_id);

    let response = client.patch(&url)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Successfully added user with UUID: {}", user_id);
    } else {
        println!("Failed to add user. Status: {}", response.status());
    }

    Ok(())
}

pub async fn get_user_details(base_url: &str, username: &str, password: &str) -> Result<UserDetailsResponse, Error> {
    let client = Client::new();
    let url = format!("{}userDetails", base_url); // Adjust the endpoint as necessary

    let user_credentials: UserDetailsRequest = UserDetailsRequest {
        username: username.to_string(),
        password: password.to_string()
    };

    let response = client.get(&url)
        .json(&user_credentials)
        .send()
        .await?;

    let mut user_details: UserDetailsResponse = UserDetailsResponse {
        name: "".to_string(),
        birthday: "".to_string(),
        diplomas: Vec::new()
    };

    if response.status().is_success() {
        user_details = response.json().await?;
        println!("User Details: {:?}", user_details);
        
    } else {
        println!("Error while retrieving user details: {}", response.status().to_string());
    }

    Ok(user_details)
}