# REDDY

Reddit Server Clone using Rust.

## Getting Started

To build Reddy you need:
* `rustc` v1.26.2 or above.
* `cargo` v1.26.0 or above.
* Running Postgres database

To make Reddy working do the following:
```
$ git clone https://github.com/incubus8/reddy
$ cd reddy
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
$ cargo run 
```

## Stacks

1. Rust Async web framework (actix-web)[https://github.com/actix/actix-web]
2. Rust (diesel ORM)[https://github.com/diesel-rs/diesel]
3. postgres as database
4. JWT

## Features

- [x] User Sign up
- [x] User Sign in
- [x] User Info
- [x] User Delete
- [x] User Update
- [x] Theme List
- [x] Create New Theme
- [x] Get Community
- [x] Get All Communities
- [x] Create New Community
- [x] Create New Community Name
- [x] Create New Community Category
- [x] Add Like to Community

