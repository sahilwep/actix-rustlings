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
    - [Simple web-server, that Serves HTML contents](#simple-web-server-that-serves-html-contents)
    - [Writing an Application:](#writing-an-application)
      - [State:](#state)
      - [Shared Mutable State:](#shared-mutable-state)
      - [Application guards and virtual hosting](#application-guards-and-virtual-hosting)
      - [Configure](#configure)
    - [The HTTP Server](#the-http-server)
      - [Multi-Threading](#multi-threading)




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

### Simple web-server, that Serves HTML contents 

* This code is a simple web server written in Rust using the Actix-web framework.

* This code creates a simple web server that serves the content of ***index.html*** file when accessed via the URL `http://localhost:8080/app/index.html`.

```rust
use actix_web::{web, App, HttpResponse, HttpServer};
use std::fs;


async fn index() -> HttpResponse {
    let html_content = fs::read_to_string("index.html")
        .expect("Unable to read file");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/app")
                // ...so this handles requests for `GET /app/index.html`
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

* The code begin with importing necessary items from the `actix_web` and `std::fs` modules.

```rust
use actix_web::{web, App, HttpResponse, HttpServer};
use std::fs;
```

* Next, there is an asynchronous function named `index()` that serves the content of an HTML file named `index.html` when accessed via a web browser.

```rust
async fn index() -> HttpResponse {
    let html_content = fs::read_to_string("index.html")
        .expect("Unable to read file");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}
```
* Inside the `main()` function, as `HttpServer` is created with a closure that configures the server. It bind the server to the address `127.0.0.1` (localhost) on port `8080`.

```rust
HttpServer::new(|| {
    App::new().service(
        // prefixes all resources and routes attached to it...
        web::scope("/app")
            // ...so this handles requests for `GET /app/index.html`
            .route("/index.html", web::get().to(index)),
    )
})
.bind(("127.0.0.1", 8080))?
```
* The closure passed to `HttpServer::new()` creates an `App` instance which defines the application's routes. In this case, it sets up the route for `GET /app/index.html` to serve the `index()` function.

```rust
App::new().service(
    // prefixes all resources and routes attached to it...
    web::scope("/app")
        // ...so this handles requests for `GET /app/index.html`
        .route("/index.html", web::get().to(index)),
)
```

* Finally, the server is started by calling `.run().await()` method, which starts the server and awaits its completion.

```rust
.run()
.await()
```

* On browser, when we go to path `http://localhost:8080/app/index.html` we can access the contents of `index.html`


### Writing an Application: 

#### State:

* Application state is shared with all routes and responses with the same scope. State can be accessed with the `web::Data<T>` extractor where `T` is the type of the state. State is also assessable for middleware.

* Example below, application stores the application name in the state:
```rust
use actix_web::{get, web, App, HttpServer};

// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```
* Defining a struct `AppState` representing the state of the application, which is this case contains a single field `app_name` of type `String`.

```rust
// This struct represents state
struct AppState {
    app_name: String,
}
```

* Defines an asynchronous function `index` which serves as a request handler for the root path `("/")`. It takes a parameter `data` of type `web::Data<AppState>` which represents the application state. It extracts the `app_name` field from the application state and returns a string response containing the app name.

```rust
#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}
```
* Then we have the `main()` function logic.


#### Shared Mutable State:

* The shared resources can be mutable in nature, example of the codebase:

```rust
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```
* We have imported the `use std::sync::Mutex;`, mutexes in this module implement a strategy called `poisoning` where a mutex is considered poisoned wherever a thread panics while holding the mutex. Once mutex is poisoned, all other threads are unable to access the data by default as it is likely trained(some invariant is not being upheld).
* In simple word, `Mutex` is used for synchronization.

```rust
struct AppStateWithCounter {
    counter: Mutex<i32>,    // mutes is necessary to mutate safely across thread
}
```
* Defines a struct `AppStateWithCounter` that holds a counter wrapped in a `Mutex`. This structure will be used to store application state.

```rust
async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}
```

* Defines an async function `index` that takes `web::Data<AppStateWithCounter>` as input. This function implements the counter inside the `AppStateWithCounter` and returns a string indicating the request number.

* Lastly, we have main function is marked with `#[actix_web::main]`, which is procedural macro provided by actix web for bootstrapping the async runtime. It creates an `HttpServer` instance and binds it to the address `127.0.0.1:8080`.


#### Application guards and virtual hosting

* guard as a simple function that accepts a *request* object reference and returns *true* or *false*. Formally, a guard is any object that implements a `Guard` trait. 

* One of the provided guards is `Host` . It can be used as a filter based on request header information.

```rust
use actix_web::{web::{self}, App, guard, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            )
            .service(
                web::scope("/")
                    .guard(guard::Host("users.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("user") })),
            )
            .route("/", web::to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```


#### Configure

* For Simplicity and reusability both `App` and `web::Scope` provide the `configure` method. This function is useful for moving parts of the configuration to a different module or even library. For example, some of the resource's configuration could be moved to a different module.

```rust
use actix_web::{web::{self, route}, App, HttpResponse, HttpServer};


fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
                    .route(web::get().to( || async { HttpResponse::Ok().body("test") }))
                    .route(web::head().to( HttpResponse::MethodNotAllowed)),
    );
}


fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api")
                    .route(web::get().to( || async {HttpResponse::Ok().body("app") }))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route("/", 
            web::get().to( || async { HttpResponse::Ok().body("/") }),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

* The result of the above example would be: 

```plain
/          ->    "/"
/app       ->    "app"
/api/test  ->    "test"
```

### The HTTP Server

* The `HttpServer` type is responsible for serving HTTP requests.

* `HttpServer` accepts an application factory as a parameter, and the application factory must have Send + Sync boundaries. More about that in the multi-threading section.

* To start the web server it must first be bound to a network socket. Use `HttpServer::bind()` with a socket address tuple or string such as `("127.0.0.1", 8080)` or `"0.0.0.0:8080"`. This will fail if the socket is being used by another application.

* After the `bind` is successful, use `HttpServer::run()` to return a Server instance. The Server must be `await`ed or `spawn`ed to start processing requests and will run until it receives a shutdown signal (such as, by default, a `ctrl-c`; read more here).

```rust
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```


#### Multi-Threading

* `HttpServer` automatically start a number of HTTP *worker*, by default this number is equal to the number of physical CPU in the system. This number can be overridden with the `HttpServer::workers()` method.

```rust
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok))).workers(4)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```
* Once the workers are created, they each receive a separate *application* instance to handle requests. Application state is not shared between the thread, and handlers are free to manipulate their copy of the state with no concurrency concerns.
* Application state does not need to be `Send` or `Sync`, but application factories must be `Send` + `Sync`.
* To share state between worker thread, use an `Arc`/`Data`. Special care should be take once sharing and synchronization are introduced. In many cases, performance costs are inadvertenly introduced as a result of locking occurs at all.

* Since each worker thread processes its requests sequentially, handlers which block the current thread will cause the current worker to stop processing new requests:

```rust

fn my_handler() -> impl Responder {
    std::thread::sleep(Duration::from_secs(5));     //  <-- Bad practice! will cause the current worker thread to hang!
    "response"
}
```
* The same limitation applies to extractors as well. When a handler function receives an argument which implements `FromRequest`, and that implementation block the current thread, the worker thread will block when running the handler. Special attention must be given when implementing extractors for this very reason, and they should also be implemented asynchronously where needed.

