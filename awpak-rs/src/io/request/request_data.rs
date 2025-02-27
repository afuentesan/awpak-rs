use std::collections::HashMap;

use crate::{body::body::BodyData, io::{cookies::cookies::Cookies, headers::headers::Headers}};

/// Represents an incoming HTTP request.
///
/// This struct contains all the relevant information about an HTTP request, including the request
/// method, URI, headers, body, and cookies. It can be accessed in an endpoint or middleware
/// function to inspect or modify the request before generating a response.
pub struct RequestData
{
    /// The URI of the request, including host, path, query parameters, and scheme.
    ///
    /// This contains detailed information about the requested resource.
    ///
    /// # Example
    /// A request to `http://example.org:3000/hello/world?name=John` will have:
    /// - `uri.host = Some("example.org".to_string())`
    /// - `uri.path = "/hello/world".to_string()`
    /// - `uri.query = Some("name=John".to_string())`
    /// - `uri.query_map = Some(HashMap::from([("name".to_string(), "John".to_string())]))`
    /// - `uri.port = Some(3000)`
    /// - `uri.scheme = Some("http".to_string())`
    pub uri : Uri,

    /// The HTTP method of the request (e.g., `"get"`, `"post"`, `"put"`, etc.).
    ///
    /// This indicates the action the client wants to perform on the requested resource.
    ///
    /// # Example
    /// ```rust
    /// use awpak_rs::io::request::request_data::RequestData;
    /// 
    /// let request_data = RequestData::default();
    /// 
    /// if request_data.method == "post" {
    ///     println!("Handling a POST request");
    /// }
    /// ```
    pub method : String,
    
    /// The headers included in the request.
    ///
    /// Headers provide metadata about the request, such as content type, authentication tokens,
    /// or caching information.
    ///
    /// # Example: Checking for an Authorization header
    /// ```rust
    /// use awpak_rs::io::request::request_data::RequestData;
    /// 
    /// let request_data = RequestData::default();
    /// 
    /// if let Some(auth_header) = request_data.headers.get_value("Authorization") {
    ///     println!("Authorization: {}", auth_header);
    /// }
    /// ```
    pub headers : Headers,

    /// The body of the request.
    ///
    /// The request body will be stored in `body.value`.
    /// If the request is a `multipart/form-data` upload, the files will be
    /// available in `body.files`.
    ///
    /// # Example: Accessing JSON data
    /// ```rust
    /// use awpak_rs::io::request::request_data::RequestData;
    /// 
    /// let request_data = RequestData::default();
    ///
    /// if let Some(json_body) = &request_data.body.value {
    ///     println!("Received JSON: {}", json_body);
    /// }
    /// ```
    ///
    /// # Example: Handling file uploads
    /// ```ignore
    /// for file in &request_data.body.files {
    ///     println!("Received file: {}", file.filename);
    /// }
    /// ```
    pub body : BodyData,

    /// The cookies included in the request.
    ///
    /// Cookies are stored as key-value pairs and can be used for session management,
    /// authentication, or storing user preferences.
    ///
    /// # Example: Retrieving a cookie by name
    /// ```rust
    /// use awpak_rs::io::request::request_data::RequestData;
    /// 
    /// let request_data = RequestData::default();
    /// 
    /// if let Some(cookie) = request_data.cookies.find_first_by_name("session_id") {
    ///     println!("Session ID: {}", cookie.value());
    /// }
    /// ```
    pub cookies : Cookies
}

/// Represents a parsed URI from an incoming HTTP request.
///
/// This struct provides access to different parts of the URI, such as the host, path, query parameters, and scheme.
///
/// # Example
///
/// Given the URL:  
/// `http://example.org:3000/hello/world?name=John`
///
/// The `Uri` struct will contain:
/// ```text
/// host = Some("example.org".to_string())
/// path = "/hello/world".to_string()
/// query = Some("name=John".to_string())
/// query_map = Some({ "name" => "John" })
/// port = Some(3000)
/// scheme = Some("http".to_string())
/// ```
///
/// # Fields
///
/// - `host` *(optional)* – The hostname of the request (e.g., `"example.org"`).
/// - `path` – The request path (e.g., `"/hello/world"`).
/// - `query` *(optional)* – The raw query string (e.g., `"name=John"`).
/// - `query_map` *(optional)* – A parsed key-value map of query parameters.
/// - `port` *(optional)* – The port number if specified (e.g., `3000`).
/// - `scheme` *(optional)* – The URI scheme (e.g., `"http"` or `"https"`).
pub struct Uri
{
    pub host : Option<String>,
    pub path : String,
    pub query : Option<String>,
    pub query_map : Option<HashMap<String, String>>,
    pub port : Option<u16>,
    pub scheme : Option<String>
}

impl Uri
{
    pub fn new( 
        host : Option<String>,
        path : String,
        query : Option<String>,
        port : Option<u16>,
        scheme : Option<String>
    ) -> Self
    {
        let query = match query {
            Some( v ) => if v.trim() == ""
            {
                None
            }
            else
            {
                Some( v )
            },
            _ => None
        };
        
        let query_map = Self::get_query_map( &query );

        Self
        {
            host,
            path,
            query,
            query_map,
            port,
            scheme
        }
    }

    fn get_query_map( query : &Option<String> ) -> Option<HashMap<String, String>>
    {
        match query {
            Some( s ) => match serde_qs::from_str::<HashMap<String, String>>( s.as_str() )
            {
                Ok( v ) => if v.len() > 0 {
                    Some( v )
                }
                else
                {
                    None
                },
                _ => None
            },
            _ => None
        }
    }
}

impl Default for Uri
{
    fn default() -> Self
    {
        Self
        {
            host : None,
            path : "/".to_string(),
            query : None,
            query_map : None,
            port : None,
            scheme : None
        }
    }
}

impl RequestData
{
    pub fn new( uri : Uri, method : String, headers : Headers, cookies : Cookies, body : BodyData ) -> Self
    {
        Self
        {
            uri,
            method,
            headers,
            body,
            cookies
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

impl Default for RequestData
{
    fn default() -> Self
    {
        Self
        {
            uri : Uri::default(),
            method : "get".to_string(),
            headers : Headers::new(),
            body : BodyData { value : None, files : vec![] },
            cookies : Cookies::new()
        }
    }
}
