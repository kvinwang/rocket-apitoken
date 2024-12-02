# rocket-apitoken

[![Crate](https://img.shields.io/crates/v/rocket-apitoken.svg)](https://crates.io/crates/rocket-apitoken)
[![API](https://docs.rs/rocket-apitoken/badge.svg)](https://docs.rs/rocket-apitoken)


<!-- cargo-rdme start -->

A very simple API Authorization module for Rocket web applications

## Overview
This module provides a simple token-based authorization system for Rocket web applications.
It supports both enabled and disabled states, and validates Bearer tokens against a predefined set.

## Usage Example
```rust
use rocket;
use rocket_apitoken::{ApiToken, Authorized};

#[post("/<method>?<json>", data = "<data>")]
async fn protected_endpoint(_auth: Authorized, /* other params */) {
    // If this executes, the request was authorized
    // ...
}

#[launch]
fn rocket() -> _ {
    let tokens = vec!["secret-token".to_string()];
    rocket::build()
        .manage(ApiToken::new(tokens, true))
        .mount("/api", routes![protected_endpoint])
}
```

## Configuration
- Create an `ApiToken` instance with a list of valid tokens and enabled state
- Add it to Rocket's state using `.manage()`
- Use the `Authorized` guard in your route handlers

When enabled, requests must include a valid token in the Authorization header.
When disabled, all requests are authorized automatically.

<!-- cargo-rdme end -->

# License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
