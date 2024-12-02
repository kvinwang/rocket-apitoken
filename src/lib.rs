//! A very simple API Authorization module for Rocket web applications
//!
//! # Overview
//! This module provides a simple token-based authorization system for Rocket web applications.
//! It supports both enabled and disabled states, and validates Bearer tokens against a predefined set.
//!
//! # Usage Example
//! ```no_run
//! use rocket;
//! use rocket_apitoken::{ApiToken, Authorized};
//!
//! #[post("/<method>?<json>", data = "<data>")]
//! async fn protected_endpoint(_auth: Authorized, /* other params */) {
//!     // If this executes, the request was authorized
//!     // ...
//! }
//!
//! #[launch]
//! fn rocket() -> _ {
//!     let tokens = vec!["secret-token".to_string()];
//!     rocket::build()
//!         .manage(ApiToken::new(tokens, true))
//!         .mount("/api", routes![protected_endpoint])
//! }
//! ```
//!
//! # Configuration
//! - Create an `ApiToken` instance with a list of valid tokens and enabled state
//! - Add it to Rocket's state using `.manage()`
//! - Use the `Authorized` guard in your route handlers
//!
//! When enabled, requests must include a valid token in the Authorization header.
//! When disabled, all requests are authorized automatically.

#![warn(missing_docs)]

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use std::collections::HashSet;

/// Configuration for API token authorization
pub struct ApiToken {
    tokens: HashSet<String>,
    enabled: bool,
}

impl ApiToken {
    /// Create a new `ApiToken` instance
    pub fn new(tokens: Vec<String>, enabled: bool) -> Self {
        Self {
            tokens: tokens
                .into_iter()
                .map(|token| format!("Bearer {}", token))
                .collect(),
            enabled,
        }
    }

    /// Add bearer tokens to the list of valid tokens
    pub fn add_bearer(&mut self, token: &str) {
        self.tokens.insert(format!("Bearer {}", token));
    }
}

/// Request guard that ensures requests are authorized
///
/// This guard will succeed if either:
/// - Authorization is disabled (`enabled = false` in ApiToken)
/// - A valid bearer token is provided in the Authorization header
///
/// # Errors
/// Returns 401 Unauthorized if:
/// - Authorization is enabled and no Authorization header is present
/// - The provided token is invalid
#[derive(Debug)]
pub struct Authorized;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authorized {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request
            .rocket()
            .state::<ApiToken>()
            .expect("Token state not available.");
        if !token.enabled {
            return Outcome::Success(Authorized);
        }
        match request.headers().get_one("Authorization") {
            Some(value) => {
                // Check the Bearer token
                if token.tokens.contains(value) {
                    Outcome::Success(Authorized)
                } else {
                    Outcome::Error((Status::Unauthorized, "invalid token"))
                }
            }
            _ => Outcome::Error((Status::Unauthorized, "Authorization header not found")),
        }
    }
}
