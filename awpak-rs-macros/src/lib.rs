use impls::{awpak_main::awpak_main_impl, from_value::from_value_impl, methods::{connect::connect_impl, delete::delete_impl, get::get_impl, head::head_impl, options::options_impl, patch::patch_impl, post::post_impl, put::put_impl, trace::trace_impl}, middleware::middleware_impl, redirect_to::redirect_to_impl, set_status_code::set_status_code_impl};
use proc_macro::TokenStream;
use quote::quote;

mod impls;
mod util;

/// The `awpak_main` macro defines the entry point for an `awpak-rs` web application.
///
/// This macro should be applied to the `main` function and allows optional configuration
/// of the server's IP address and port.
///
/// # Parameters
///
/// - `ip` *(optional, default: `"127.0.0.1"`)*  
///   Specifies the IP address on which the server will listen.
///
/// - `port` *(optional, default: `"3000"`)*  
///   Specifies the port on which the server will listen.
///
/// # Example
///
/// ```ignore
/// #[awpak_main(ip = "127.0.0.1", port = "3001")]
/// fn main() {}
/// ```
///
/// In this example, the web server will start on `127.0.0.1:3001`.  
/// If no parameters are provided, the server will default to `127.0.0.1:3000`.
#[proc_macro_attribute]
pub fn awpak_main( args: TokenStream, item: TokenStream ) -> TokenStream
{
    awpak_main_impl( args, item )
}

/// The `post` macro defines an HTTP POST endpoint for an `awpak-rs` web application.
///
/// This macro should be applied to a function that handles HTTP requests for the specified URL.
/// 
/// The return type of the function can be any Rust primitive or any type that implements `Serialize` from `serde`.
///
/// # Asynchronous Execution
/// 
/// Functions annotated with `post` are executed asynchronously.  
/// You do not need to explicitly mark them as `async`, as the macro automatically ensures async execution.  
/// This means you can freely use `.await` inside these functions.
/// 
/// # Parameters
///
/// - `url` *(required)*  
///   The URL pattern for this endpoint.
///
/// # Example
///
/// ```ignore
/// #[post(url = "/post_body_echo")]
/// fn post_body_echo(#[request_body] point: Point) -> Point {
///     point
/// }
/// ```
///
/// In this example, a request to `POST /post_body_echo` with a JSON body `{ "x": 3, "y": 2 }`
/// will return the same object.
#[proc_macro_attribute]
pub fn post( args: TokenStream, item: TokenStream ) -> TokenStream
{
    post_impl( args, item )
}

/// The `get` macro defines an HTTP GET endpoint for an `awpak-rs` web application.
///
/// This macro should be applied to a function that handles HTTP requests for the specified URL.
/// 
/// The return type of the function can be any Rust primitive or any type that implements `Serialize` from `serde`.
///
/// # Asynchronous Execution
///
/// Functions annotated with `get` are executed asynchronously.  
/// You do not need to explicitly mark them as `async`, as the macro automatically ensures async execution.  
/// This means you can freely use `.await` inside these functions.
/// 
/// # Parameters
///
/// - `url` *(required)*  
///   The URL pattern for this endpoint.
///
/// # Example
///
/// ```ignore
/// #[get(url = "/get_echo")]
/// fn get_echo() -> String {
///     "Hello, world!".to_string()
/// }
/// ```
///
/// In this example, a request to `GET /get_echo` will return `"Hello, world!"`.
#[proc_macro_attribute]
pub fn get( args: TokenStream, item: TokenStream ) -> TokenStream
{
    get_impl( args, item )
}

/// Defines an HTTP `CONNECT` route.
///
/// This macro registers a function as a handler for `CONNECT` requests.
/// It takes a required `url` parameter to specify the endpoint path.
///
/// # Asynchronous Execution
/// 
/// Functions annotated with `connect` are executed asynchronously.  
/// You do not need to explicitly mark them as `async`, as the macro automatically ensures async execution.  
/// This means you can freely use `.await` inside these functions.
/// 
/// # Parameters
/// - `url`: *(required)* The endpoint URL pattern.
///
/// # Return Type
/// The function's return type can be any Rust primitive or a type implementing `Serialize` from `serde`.
///
/// # Example
/// ```ignore
/// #[connect(url = "/connect_example")]
/// fn handle_connect() -> String {
///     "Connected".to_string()
/// }
/// ```
#[proc_macro_attribute]
pub fn connect( args: TokenStream, item: TokenStream ) -> TokenStream
{
    connect_impl( args, item )
}

/// Defines an HTTP `DELETE` route.
///
/// This macro registers a function as a handler for `DELETE` requests.
/// It takes a required `url` parameter to specify the endpoint path.
///
/// # Asynchronous Execution
/// 
/// Functions annotated with `delete` are executed asynchronously.  
/// You do not need to explicitly mark them as `async`, as the macro automatically ensures async execution.  
/// This means you can freely use `.await` inside these functions.
/// 
/// # Parameters
/// - `url`: *(required)* The endpoint URL pattern.
///
/// # Return Type
/// The function's return type can be any Rust primitive or a type implementing `Serialize` from `serde`.
///
/// # Example
/// ```ignore
/// #[delete(url = "/delete_example")]
/// fn handle_delete() -> String {
///     "Deleted".to_string()
/// }
/// ```
#[proc_macro_attribute]
pub fn delete( args: TokenStream, item: TokenStream ) -> TokenStream
{
    delete_impl( args, item )
}

/// Defines an HTTP `HEAD` route.
///
/// This macro registers a function as a handler for `HEAD` requests.
/// It takes a required `url` parameter to specify the endpoint path.
///
/// # Asynchronous Execution
/// 
/// Functions annotated with `head` are executed asynchronously.  
/// You do not need to explicitly mark them as `async`, as the macro automatically ensures async execution.  
/// This means you can freely use `.await` inside these functions.
/// 
/// # Parameters
/// - `url`: *(required)* The endpoint URL pattern.
///
/// # Return Type
/// The function's return type can be any Rust primitive or a type implementing `Serialize` from `serde`.
///
/// # Example
/// ```ignore
/// #[head(url = "/head_example")]
/// fn handle_head() {}
/// ```
#[proc_macro_attribute]
pub fn head( args: TokenStream, item: TokenStream ) -> TokenStream
{
    head_impl( args, item )
}

/// Defines an HTTP `OPTIONS` route.
///
/// This macro registers a function as a handler for `OPTIONS` requests.
/// It takes a required `url` parameter to specify the endpoint path.
///
/// # Asynchronous Execution
/// 
/// Functions annotated with `options` are executed asynchronously.  
/// You do not need to explicitly mark them as `async`, as the macro automatically ensures async execution.  
/// This means you can freely use `.await` inside these functions.
/// 
/// # Parameters
/// - `url`: *(required)* The endpoint URL pattern.
///
/// # Return Type
/// The function's return type can be any Rust primitive or a type implementing `Serialize` from `serde`.
///
/// # Example
/// ```ignore
/// #[options(url = "/options_example")]
/// fn handle_options() -> String {
///     "Allowed methods: GET, POST".to_string()
/// }
/// ```
#[proc_macro_attribute]
pub fn options( args: TokenStream, item: TokenStream ) -> TokenStream
{
    options_impl( args, item )
}

/// Defines an HTTP `PATCH` route.
///
/// This macro registers a function as a handler for `PATCH` requests.
/// It takes a required `url` parameter to specify the endpoint path.
///
/// # Asynchronous Execution
/// 
/// Functions annotated with `patch` are executed asynchronously.  
/// You do not need to explicitly mark them as `async`, as the macro automatically ensures async execution.  
/// This means you can freely use `.await` inside these functions.
/// 
/// # Parameters
/// - `url`: *(required)* The endpoint URL pattern.
///
/// # Return Type
/// The function's return type can be any Rust primitive or a type implementing `Serialize` from `serde`.
///
/// # Example
/// ```ignore
/// #[patch(url = "/patch_example")]
/// fn handle_patch() -> String {
///     "Patched".to_string()
/// }
/// ```
#[proc_macro_attribute]
pub fn patch( args: TokenStream, item: TokenStream ) -> TokenStream
{
    patch_impl( args, item )
}

/// Defines an HTTP `PUT` route.
///
/// This macro registers a function as a handler for `PUT` requests.
/// It takes a required `url` parameter to specify the endpoint path.
/// 
/// # Asynchronous Execution
/// 
/// Functions annotated with `put` are executed asynchronously.  
/// You do not need to explicitly mark them as `async`, as the macro automatically ensures async execution.  
/// This means you can freely use `.await` inside these functions.
///
/// # Parameters
/// - `url`: *(required)* The endpoint URL pattern.
///
/// # Return Type
/// The function's return type can be any Rust primitive or a type implementing `Serialize` from `serde`.
///
/// # Example
/// ```ignore
/// #[put(url = "/put_example")]
/// fn handle_put() -> String {
///     "Updated".to_string()
/// }
/// ```
#[proc_macro_attribute]
pub fn put( args: TokenStream, item: TokenStream ) -> TokenStream
{
    put_impl( args, item )
}

/// Defines an HTTP `TRACE` route.
///
/// This macro registers a function as a handler for `TRACE` requests.
/// It takes a required `url` parameter to specify the endpoint path.
///
/// # Asynchronous Execution
/// 
/// Functions annotated with `trace` are executed asynchronously.  
/// You do not need to explicitly mark them as `async`, as the macro automatically ensures async execution.  
/// This means you can freely use `.await` inside these functions.
/// 
/// # Parameters
/// - `url`: *(required)* The endpoint URL pattern.
///
/// # Return Type
/// The function's return type can be any Rust primitive or a type implementing `Serialize` from `serde`.
///
/// # Example
/// ```ignore
/// #[trace(url = "/trace_example")]
/// fn handle_trace() -> String {
///     "Trace response".to_string()
/// }
/// ```
#[proc_macro_attribute]
pub fn trace( args: TokenStream, item: TokenStream ) -> TokenStream
{
    trace_impl( args, item )
}

/// The `request_body` macro deserializes the entire request body into a Rust type.
///
/// This macro should be applied to function parameters that represent the body of the request.
///
/// The type must implement `Deserialize` and `FromValue`.
///
/// # Example
///
/// ```ignore
/// #[post(url = "/post_point")]
/// fn post_point(#[request_body] point: Point) -> Point {
///     point
/// }
/// ```
///
/// A request like `POST /post_point` with a JSON body `{ "x": 3, "y": 2 }`
/// will return the same object.
#[proc_macro]
pub fn request_body( _args : TokenStream ) -> TokenStream
{
    quote! {}.into()
}

/// The `query_params` macro deserializes multiple query parameters into a struct.
///
/// This macro should be applied to a function parameter that implements `Deserialize` and `FromValue`.
///
/// # Example
///
/// ```ignore
/// #[derive(Serialize, Deserialize, FromValue)]
/// struct Point {
///     x: Option<f32>,
///     y: f32,
/// }
///
/// #[get(url = "/point")]
/// fn get_point(#[query_params] point: Point) -> Point {
///     point
/// }
/// ```
///
/// A request like `GET /point?x=1.5&y=2.0` will return a JSON object `{"x":1.5,"y":2.0}`.
#[proc_macro]
pub fn query_params( _args : TokenStream ) -> TokenStream
{
    quote! {}.into()
}

/// Macro for extracting a single file from a multipart request in `awpak-rs`.
///
/// The `part_file` macro allows an endpoint to retrieve a specific file from a multipart request.
/// The extracted `FileData` struct contains information about the uploaded file, including its name, filename, bytes, and content type.
///
/// # Example
/// ```ignore
/// #[post( url = "/post_multipart_file_len" )]
/// fn post_multipart_file_len(
///     #[part_file] img: FileData
/// ) -> usize {
///     img.bytes.len()
/// }
/// ```
///
/// If the file is optional, use `Option<FileData>`:
/// ```ignore
/// #[part_file] img: Option<FileData>
/// ```
#[proc_macro]
pub fn part_file( _args : TokenStream ) -> TokenStream
{
    quote! {}.into()
}

/// Macro for extracting multiple files from a multipart request in `awpak-rs`.
///
/// The `part_files` macro allows an endpoint to retrieve all files uploaded in a multipart request.
/// The extracted `Vec<FileData>` contains information about each file received.
///
/// # Example
/// ```ignore
/// #[post( url = "/post_multipart_files_len" )]
/// fn post_multipart_files_len(
///     #[part_files] img: Vec<FileData>
/// ) -> usize {
///     img.iter().map(|i| i.bytes.len()).sum()
/// }
/// ```
#[proc_macro]
pub fn part_files( _args : TokenStream ) -> TokenStream
{
    quote! {}.into()
}

/// The `body_param` macro extracts a specific parameter from the request body.
///
/// This macro should be applied to function parameters to deserialize a single value
/// from the body of a JSON request.
///
/// The extracted type must be a Rust primitive or implement `Deserialize` and `FromValue`.
///
/// # Example
///
/// ```ignore
/// #[post(url = "/post_x")]
/// fn post_x(#[body_param] x: f32) -> f32 {
///     x
/// }
/// ```
///
/// If a request sends `{ "x": 3.5, "y": 9.7 }` in the body, the function will receive `3.5` as `x`.
#[proc_macro]
pub fn body_param( _args : TokenStream ) -> TokenStream
{
    quote! {}.into()
}

/// The `path_variable` macro extracts a path parameter from the URL.
///
/// This macro should be applied to function parameters to deserialize path variables.
/// The extracted value can be converted into any Rust primitive type or any type
/// that implements the `FromAsyncStr` trait.
///
/// # Example
///
/// ```ignore
/// #[get(url = "/user/{name}")]
/// fn get_user(#[path_variable] name: String) -> String {
///     name
/// }
/// ```
///
/// A request like `GET /user/john` will return `"john"`.
///
/// You can also deserialize the path variable into a more complex type that
/// implements `FromAsyncStr`:
///
/// ```ignore
/// #[get(url = "/user/{id}")]
/// async fn get_user(#[path_variable] user: User) -> User {
///     user
/// }
/// ```
///
/// If `User` implements `FromAsyncStr`, the framework will automatically fetch
/// the user from a database or other source based on the `id` provided in the URL.
#[proc_macro]
pub fn path_variable( _args : TokenStream ) -> TokenStream
{
    quote! {}.into()
}

/// Macro for accessing and modifying the request context in `awpak-rs`.
///
/// The `context` macro allows an endpoint to retrieve or modify the shared context set by a middleware.
/// This context can store application-specific data that needs to persist through the request lifecycle.
///
/// # Example
/// ```ignore
/// struct ContextTest {
///     x: usize,
/// }
///
/// #[middleware]
/// fn middleware_set_context(mut io: IO) -> MiddlewareResponse {
///     io.set_context(ContextTest { x: 77 });
///     MiddlewareResponse::Next(io)
/// }
///
/// #[post( url = "/post_echo_context" )]
/// fn post_echo_context(
///     #[context] ctx: Option<&ContextTest>
/// ) -> String {
///     match ctx {
///         Some(v) => format!("x:{}", v.x),
///         None => "None".to_string(),
///     }
/// }
/// ```
///
/// To modify the context, use `Option<&mut T>` instead:
/// ```ignore
/// #[post( url = "/post_modify_context" )]
/// fn post_modify_context(
///     #[context] ctx: Option<&mut ContextTest>
/// ) -> String {
///     if let Some(c) = ctx {
///         c.x += 1;
///         format!("Updated x: {}", c.x)
///     } else {
///         "No context available".to_string()
///     }
/// }
/// ```
#[proc_macro]
pub fn context( _args : TokenStream ) -> TokenStream
{
    quote! {}.into()
}

/// Extracts the request headers in an endpoint function.
///
/// The `request_headers` macro allows an endpoint function to access the HTTP request headers.
/// It must be used as an attribute on a function parameter of type `Headers`.
///
/// # Usage
/// - If headers need to be modified, the parameter should be mutable (`mut`).
/// - The headers can be queried using methods from the `Headers` struct.
///
/// # Example
/// ```ignore
/// #[post( url = "/post_request_header_example" )]
/// fn post_request_header_example(
///     #[request_headers]
///     headers : Headers
/// ) -> String
/// {
///     match headers.get_value("content-type") {
///         Some(c) => c,
///         _ => "Content-Type not found".to_string()
///     }
/// }
/// ```
#[proc_macro]
pub fn request_headers( _args : TokenStream ) -> TokenStream
{
    quote! {}.into()
}

/// Provides access to response headers in an endpoint function.
///
/// The `response_headers` macro allows modifying HTTP response headers.
/// It must be used as an attribute on a function parameter of type `Headers`.
///
/// # Usage
/// - If headers need to be modified, the parameter should be mutable (`mut`).
/// - Headers can be set or replaced using methods from the `Headers` struct.
///
/// # Example
/// ```ignore
/// #[post( url = "/post_response_header_example" )]
/// fn post_response_header_example(
///     #[response_headers]
///     mut headers : Headers
/// ) -> String
/// {
///     headers.replace_header("content-type".to_string(), "application/json".to_string());
///
///     "Content-Type changed".to_string()
/// }
/// ```
#[proc_macro]
pub fn response_headers( _args : TokenStream ) -> TokenStream
{
    quote! {}.into()
}

/// Macro for accessing cookies in `awpak-rs` request handlers.
///
/// The `request_cookies` macro allows an endpoint to access the cookies sent by the client in the request.
/// The extracted `Cookies` struct provides methods for retrieving specific cookies by name.
///
/// # Example
/// ```ignore
/// #[post( url = "/post_request_cookies_example" )]
/// fn post_request_cookies_example(
///     #[request_cookies] cookies: Cookies
/// ) -> String {
///     match cookies.find_first_by_name("session_id") {
///         Some(cookie) => format!("Session ID: {}", cookie.value()),
///         None => "No session cookie found".to_string(),
///     }
/// }
/// ```
#[proc_macro]
pub fn request_cookies( _args : TokenStream ) -> TokenStream
{
    quote! {}.into()
}

/// Macro for modifying response cookies in `awpak-rs` request handlers.
///
/// The `response_cookies` macro allows an endpoint to modify the cookies that will be sent in the response.
/// The extracted `Cookies` struct can be used to add or replace cookies.
///
/// # Example
/// ```ignore
/// #[post( url = "/post_response_cookies_example" )]
/// fn post_response_cookies_example(
///     #[response_cookies] mut res_cookies: Cookies
/// ) -> String {
///     res_cookies.add_cookie("user_id=12345; Path=/;").unwrap();
///     "Cookie set successfully".to_string()
/// }
/// ```
///
/// If cookies do not need to be modified, the `mut` keyword can be omitted.
#[proc_macro]
pub fn response_cookies( _args : TokenStream ) -> TokenStream
{
    quote! {}.into()
}

/// The `query_param` macro extracts a single query parameter from the URL.
///
/// This macro should be applied to function parameters to deserialize query parameters.
/// 
/// The extracted type must be a Rust primitive or implement `Deserialize` and `FromValue`.
///
/// # Example
///
/// ```ignore
/// #[get(url = "/sum")]
/// fn sum(
///     #[query_param] a: u16,
///     #[query_param] b: u16
/// ) -> u16 {
///     a + b
/// }
/// ```
///
/// A request like `GET /sum?a=5&b=10` will return `15`.
#[proc_macro]
pub fn query_param( _args : TokenStream ) -> TokenStream
{
    quote! {}.into()
}

/// Derive macro for implementing `FromValue`.
///
/// This macro automatically generates an implementation of the `FromValue` trait,
/// allowing a struct to be deserialized from query parameters, request bodies, or
/// other extracted values.
///
/// # Usage
///
/// This derive macro is used for structs that need to be deserialized using
/// `query_param`, `query_params`, `request_body`, or `body_param`. It works alongside
/// `serde`'s `Deserialize` trait.
///
/// # Example
///
/// ```ignore
/// use awpak_rs::FromValue;
/// use serde::Deserialize;
///
/// #[derive(Deserialize, FromValue)]
/// struct User {
///     id: u32,
///     name: String,
/// }
/// ```
///
/// Now, `User` can be used with `#[query_param]`, `#[query_params]`, `#[request_body]`, or `#[body_param]`
/// to automatically deserialize values from the request.
#[proc_macro_derive(FromValue)]
pub fn derive_from_value( item : TokenStream ) -> TokenStream
{
    from_value_impl( item )
}

/// Defines a middleware function in `awpak-rs`.
///
/// Middleware functions allow modifying incoming requests, responses, or setting a shared context
/// before or after executing an endpoint. Middleware can be applied globally or conditionally
/// based on URL patterns, HTTP methods, or execution order.
///
/// # Asynchronous Execution
/// 
/// Functions annotated with `middleware` are executed asynchronously.  
/// You do not need to explicitly mark them as `async`, as the macro automatically ensures async execution.  
/// This means you can freely use `.await` inside these functions.
/// 
/// # Parameters
///
/// - `urls`: *(optional, default: all URLs)*  
///   A list of regular expressions defining which URLs this middleware should apply to.
///   Example: `urls = ["/api/.*"]` applies the middleware to all endpoints under `/api/`.
///
/// - `order`: *(optional, default: 10000)*  
///   Determines the execution order of middlewares. Lower values run first. If two middlewares
///   have the same order, their execution order is undefined.
///
/// - `execute_after`: *(optional, default: `false`)*  
///   If `true`, the middleware will execute after the endpoint instead of before it.
///
/// - `method`: *(optional, default: all methods)*  
///   Restricts the middleware to a specific HTTP method (e.g., `method = "get"`).
///
/// # Middleware Response
///
/// Middleware functions must return a `MiddlewareResponse`, which can be:
/// - `MiddlewareResponse::Next(io)`: Continue to the next middleware or endpoint.
/// - `MiddlewareResponse::Cancel(io)`: Stop execution and return a response immediately.
///
/// # Example: Logging Middleware
///
/// ```ignore
/// use awpak_rs::*;
///
/// #[middleware(urls=["/secure/.*"], order=1)]
/// fn logging_middleware(mut io: IO) -> MiddlewareResponse {
///     println!("Request to: {}", io.request.uri.path);
///     MiddlewareResponse::Next(io)
/// }
/// ```
///
/// # Example: Authentication Middleware
///
/// ```ignore
/// #[middleware(order=5, execute_after=false)]
/// fn auth_middleware(mut io: IO) -> MiddlewareResponse {
///     if let Some(token) = io.request.headers.get("Authorization") {
///         if validate_token(token) {
///             return MiddlewareResponse::Next(io);
///         }
///     }
///     io.response.status = 401;
///     MiddlewareResponse::Cancel(io)
/// }
/// ```
#[proc_macro_attribute]
pub fn middleware( args: TokenStream, item: TokenStream ) -> TokenStream
{
    middleware_impl( args, item )
}

#[proc_macro]
pub fn set_status_code( item : TokenStream ) -> TokenStream
{
    set_status_code_impl( item )
}

#[proc_macro]
pub fn redirect_to( item : TokenStream ) -> TokenStream
{
    redirect_to_impl( item )
}