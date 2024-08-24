# Cash Book Application

Have you seen a **CashBook** before? If not take a look [here](https://www.amazon.com/cash-log-book/s?k=cash+log+bookhttps:/) and you will undestand what's this.

There's no purposes as product. Its just for Rust practicing and fun! Fell free to clone and modify. Feel free to ask for features and do your contributions;

# Running Project Locally

## Requirements

- Rust development setup
- Podman
- Just
- sqlx-cli

## Let's Run

First Execute database server and pod setup running the following command:

```
just run-dev
```

Then create database and run database migrations:

```
sqlx database create && slqx migrate run
```

If ok! Now run locally with `cargo` to build and run the application:

```
cargo run
```

# Project setup

- [X] Application basic infrastucture and oganization;
- [ ] Dockerize application

# Features

### Movements Registration

- [ ] Create movements
  - [X] API
  - [ ] UI

## Development Lifecycle Automation

The Ops things from "Ops" of Devops term if you undestand me..

- [ ] Deploy automation (deploy where and how? anywhere that support container deployment)
- [ ] Run tests on pull request (github actions setup)
- [ ] Setup automation for release creation

# Author

[Marcos R. Araujo](https://m4rc0s.github.io/https:/)
