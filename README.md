# awpak-rs

**awpak-rs** is a lightweight web framework for Rust, built on top of [`tokio`](https://tokio.rs/) and [`hyper`](https://hyper.rs/). It simplifies request handling, middleware processing, and response generation, using declarative macros for a more ergonomic development experience.

⚠️ **This is an initial version of the project, and the API may change in future releases.**

## Features
- **Declarative Macros**: Define routes using `#[get]`, `#[post]`, `#[put]`, `#[delete]`, and more.
- **Flexible Request Handling**: Extract query parameters, request headers, cookies, request bodies, and file uploads effortlessly.
- **Middleware Support**: Define and chain middleware to modify requests and responses.
- **Context Sharing**: Use middleware to pass shared state across request processing.
- **Automatic JSON Serialization**: Responses and request bodies seamlessly integrate with `serde`.
- **File Upload Support**: Handle `multipart/form-data` file uploads efficiently.

## Installation

To use `awpak-rs`, add it to your `Cargo.toml`:

```toml
[dependencies]
awpak-rs = "0.0.1"
serde = { version = "1.0", features = ["derive"] }
```

## Getting Started

### Basic Example

```rust
use awpak_rs::*;
use serde::{Serialize, Deserialize};

#[awpak_main(ip = "127.0.0.1", port = "3001")]
fn main() {}

#[derive(Serialize, Deserialize, FromValue)]
struct Point {
    x: Option<f32>,
    y: f32,
}

#[get(url = "/get_point")]
fn get_point(#[query_params] point: Point) -> Point {
    point
}
```

Run the server and test with:

```sh
curl "http://127.0.0.1:3001/get_point?x=1&y=2"
```

Response:
```json
{"x":1.0,"y":2.0}
```

### Setting Up the Main Function
The `#[awpak_main]` macro defines the entry point of the application:

```rust
#[awpak_main(ip = "127.0.0.1", port = "3001")]
fn main() {}
```

- **`ip`** (optional): The IP address to bind to (default: `127.0.0.1`).
- **`port`** (optional): The port the server will listen on (default: `3000`).

### Defining Routes

Endpoints are defined using route macros such as `#[get]` and `#[post]`:

```rust
#[get(url = "/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}
```

#### Extracting Query Parameters
```rust
#[get(url = "/greet")]
fn greet(
    #[query_param] name: String
) -> String {
    format!("Hello, {}!", name)
}
```

### Path Variables
```rust
#[get(url = "/user/{id}")]
fn get_user(
    #[path_variable] id: usize
) -> String {
    format!("User ID: {}", id)
}
```

### Extracting a User from a Path Variable

```rust
#[get(url = "/user/{id}")]
async fn get_user(#[path_variable] user: User) -> User {
    user
}

impl FromAsyncStr<User> for User {
    async fn from_async_str(io: &IO, s: &str) -> Result<User, ()> {
        let user = get_user_from_db(s).await;
        Ok(user)
    }
}
```

### Handling Request and Response Headers
Extract request headers:
```rust
#[post(url = "/header-example")]
fn header_example(
    #[request_headers] headers: Headers
) -> String {
    match headers.get_value("content-type") {
        Some(content_type) => content_type,
        None => "Content-Type not found".to_string(),
    }
}
```

Modify response headers:
```rust
#[post(url = "/set-header")]
fn set_header(
    #[response_headers] mut headers: Headers
) -> String {
    headers.replace_header("content-type", "application/json");
    "Header set".to_string()
}
```

### Working with Cookies
Retrieve request cookies:
```rust
#[get(url = "/get-cookie")]
fn get_cookie(
    #[request_cookies] cookies: Cookies
) -> String {
    match cookies.find_first_by_name("session_id") {
        Some(cookie) => format!("Session ID: {}", cookie.value()),
        None => "No session cookie found".to_string(),
    }
}
```

Modify response cookies:
```rust
#[post(url = "/set-cookie")]
fn set_cookie(
    #[response_cookies] mut res_cookies: Cookies
) -> String {
    res_cookies.add_cookie("user_id=12345; Path=/;").unwrap();
    "Cookie set successfully".to_string()
}
```

### Middleware Support
Middlewares allow modifying incoming requests, responses, or setting shared context before or after executing an endpoint. They can:
- Modify request headers, cookies, or body.
- Modify response headers, cookies, or status codes.
- Add authentication or logging logic.
- Execute conditionally based on URL patterns, HTTP methods, or execution order.

```rust
#[middleware]
fn log_requests(mut io: IO) -> MiddlewareResponse {
    println!("Received request: {} {}", io.request.method, io.request.uri.path);
    MiddlewareResponse::Next(io)
}
```
Middlewares return:
- `MiddlewareResponse::Next(io)`: Continue to the next middleware or endpoint.
- `MiddlewareResponse::Cancel(io)`: Stop execution and return a response immediately.

#### Using Context in Middleware
Middleware can set shared context using `IO::set_context`:
```rust
struct ContextExample {
    value: usize,
}

#[middleware]
fn set_context(mut io: IO) -> MiddlewareResponse {
    io.set_context(ContextExample { value: 42 });
    MiddlewareResponse::Next(io)
}
```

Endpoints can then retrieve this context:
```rust
#[get(url = "/context")]
fn get_context(
    #[context] ctx: Option<&ContextExample>
) -> String {
    match ctx {
        Some(c) => format!("Context value: {}", c.value),
        None => "No context available".to_string(),
    }
}
```

### Handling File Uploads
Extract a single uploaded file:
```rust
#[post(url = "/upload")]
fn upload_file(
    #[part_file] file: FileData
) -> usize {
    file.bytes.len()
}
```

Extract multiple uploaded files:
```rust
#[post(url = "/upload-multiple")]
fn upload_multiple_files(
    #[part_files] files: Vec<FileData>
) -> usize {
    files.iter().map(|f| f.bytes.len()).sum()
}
```

## Supported HTTP Methods
Awpak-rs supports the following HTTP methods:
- `#[get]`
- `#[post]`
- `#[put]`
- `#[delete]`
- `#[patch]`
- `#[head]`
- `#[options]`
- `#[trace]`
- `#[connect]`

## Asynchronous Execution

All endpoint functions and middlewares in `awpak-rs` are executed asynchronously. There is no need to manually mark functions with `async`, as the macros automatically handle this.

This means you can use `await` inside any function annotated with an endpoint macro (such as `#[get]`, `#[post]`, etc.) or inside a middleware function.

### Example: Using `await` in an Endpoint

```rust
async fn fetch_data() -> String {
    awpak_rs::tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    "Data loaded".to_string()
}

#[get(url = "/async-example")]
fn async_example() -> String {
    fetch_data().await
}
```

## License
Awpak-rs is released under the MIT License.

