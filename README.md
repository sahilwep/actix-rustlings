# Actix Rustlings

Welcome to the Actix-rustlings repository! 🎉

This repository is dedicated to documenting my learning journey with Actix, a powerful asynchronous web framework for Rust.

## About Actix

[Actix](https://actix.rs/) is a powerful, pragmatic, and extremely fast web framework for Rust. It is built on top of Tokio, providing robust asynchronous capabilities for building high-performance web applications.

## Contents

- [Actix Rustlings](#actix-rustlings)
  - [About Actix](#about-actix)
  - [Contents](#contents)
  - [Getting Started](#getting-started)
  - [Features](#features)
  - [Walkthrough](#walkthrough)
    - [Creating a simple *hello world!*](#creating-a-simple-hello-world)
    - [Writing an Application:](#writing-an-application)




## Getting Started

To get started with Actix, make sure you have Rust installed on your system. You can install Rust by following the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can create a new Actix project by using Cargo, Rust's package manager:

```bash
$ cargo new my_actix_project
$ cd my_actix_project
```

Then, add Actix as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
actix-web = "4.5.1"
```

Now, you're ready to start building your Actix application!

## Features

- Asynchronous request handling
- Middleware support
- WebSocket support
- HTTP/2 and SSL/TLS support
- WebSockets
- Testing utilities
- And much more!

## Walkthrough

### Creating a simple *hello world!*

```rust
use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/value")]
async fn value(val: String) -> impl Responder {
    HttpResponse::Ok().body(val)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello) 
            .service(echo)
            .service(value)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```
* Some of these handlers have routing information attached directly using build in macros. These allows you to specify the method and path that the handler should respond to. 

* Next, we create an `App` instance and register the request handlers. Use `App::service` for the handlers using routing macros and `App::route` for manually routed handlers, declaring the path and method. 

* App is started inside an `HttpServer` which will serve incoming requests using your `App` as an "application factory".
* The `#[actix_web::main]` macro executes the async main function within the actix runtime. 

### Writing an Application: 
