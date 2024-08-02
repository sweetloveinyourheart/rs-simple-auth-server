# rs-simple-auth-server
Simple authentication server using Rust

# Description
rs-simple-auth-server is a simple authentication server built using Rust, Tokio, and Axum. It provides a basic structure for building an authentication server with modular components for handling routes, configurations, database interactions, error handling, and more.

# Project Structure
```
my-axum-app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── health_check.rs
│   │   ├── user.rs
│   ├── routes.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   ├── db.rs
│   ├── errors.rs
│   ├── utils.rs
```
