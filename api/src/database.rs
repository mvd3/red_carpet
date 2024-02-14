use std::collections::HashMap;
use crate::models::{Request, User};
use uuid::Uuid;

pub struct Database {
    requests: HashMap<Uuid, Request>,
    users: HashMap<Uuid, User>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            requests: HashMap::new(),
            users: HashMap::new(),
        }
    }

    pub fn add_request(&mut self, request: Request) {
        self.requests.insert(request.id, request);
    }

    pub fn get_request(&self, id: &Uuid) -> Option<&Request> {
        self.requests.get(id)
    }

    pub fn get_requests(&self, skip: usize, size: usize) -> Vec<Request> {
        self.requests
            .values()
            .skip(skip)
            .take(size)
            .cloned()
            .collect()
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    pub fn username_exists<F>(&self, predicate: F) -> bool
    where
        F: Fn(&User) -> bool,
    {
        self.users.values().any(|user| predicate(user))
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        self.users.values().find(|user| user.username == username)
    }
}
