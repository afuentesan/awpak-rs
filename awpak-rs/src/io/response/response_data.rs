
use crate::io::{cookies::cookies::Cookies, headers::headers::Headers};



/// Represents an HTTP response.
///
/// This struct contains the status code, headers, cookies, and body of the HTTP response.
/// It is used to define the response that will be sent back to the client after processing
/// an HTTP request.
///
/// The response body is stored as a `serde_json::Value`, meaning any type that implements
/// `Serialize` from `serde` can be used as a response in an endpoint.
///
/// Middlewares that run after an endpoint can modify this structure, including changing
/// the status code, adding headers and cookies, or modifying the response body.
pub struct ResponseData
{
    /// The HTTP status code of the response (e.g., `200` for OK, `404` for Not Found).
    ///
    /// This determines the outcome of the request as interpreted by the client.
    ///
    /// # Example: Setting a custom status code
    /// ```rust
    /// use awpak_rs::io::response::response_data::ResponseData;
    /// 
    /// let mut response_data = ResponseData::default();
    /// 
    /// response_data.status = 404;
    /// ```
    pub status : u16,

    /// The headers included in the response.
    ///
    /// Headers provide metadata about the response, such as content type, caching instructions,
    /// or authentication challenges.
    ///
    /// # Example: Setting a content type
    /// ```rust
    /// use awpak_rs::io::response::response_data::ResponseData;
    /// 
    /// let mut response_data = ResponseData::default();
    /// 
    /// response_data.headers.replace_header("Content-Type".to_string(), "application/json".to_string());
    /// ```
    pub headers : Headers,

    /// The cookies included in the response.
    ///
    /// Cookies can be used to store session data, user preferences, or authentication tokens.
    ///
    /// # Example: Setting a cookie
    /// ```rust
    /// use awpak_rs::io::response::response_data::ResponseData;
    /// 
    /// let mut response_data = ResponseData::default();
    /// 
    /// response_data.cookies.replace_cookie("session_id=abc123; Path=/; HttpOnly").unwrap();
    /// ```
    pub cookies : Cookies,

    /// The body of the response.
    ///
    /// The return value of an endpoint function is stored in this field, unless the function returns `None`.  
    ///
    /// An endpoint must return either:
    /// - A Rust primitive (e.g., `String`, `i32`, `bool`).
    /// - A struct that implements `Serialize` from `serde`.
    /// - An `Option<T>` where `T` is a Rust primitive or a struct that implements `Serialize`.
    ///
    /// If the endpoint returns `None`, the body remains `None`.  
    /// Otherwise, the return value is automatically converted to a `serde_json::Value`.
    ///
    /// Middlewares that execute after an endpoint can modify this value using the API provided by `serde_json`.
    ///
    /// # Example: Modifying the response body in a middleware
    /// ```rust
    /// use awpak_rs::io::response::response_data::ResponseData;
    /// 
    /// let mut response_data = ResponseData::default();
    /// 
    /// if let Some(json_body) = &mut response_data.body {
    ///     json_body["message"] = serde_json::Value::String("Modified by middleware".to_string());
    /// }
    /// ```
    pub body : Option<serde_json::Value>
}

impl ResponseData
{
    pub fn new( status : u16, headers : Headers, cookies : Cookies, body : Option<serde_json::Value> ) -> Self
    {
        Self
        {
            status,
            headers,
            cookies,
            body
        }
    }

    pub fn default() -> Self
    {
        Self
        {
            status : 200,
            headers : Headers::new(),
            body : None,
            cookies : Cookies::new()
        }
    }

    pub fn get_headers( &self ) -> Headers
    {
        self.headers.clone()
    }

    pub fn get_cookies( &self ) -> Cookies
    {
        self.cookies.clone()
    }
}
