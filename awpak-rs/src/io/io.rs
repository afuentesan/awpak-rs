use std::any::Any;

use super::{request::request_data::RequestData, response::response_data::ResponseData};

/// Represents the input and output data for an HTTP request in `awpak-rs`.
///
/// The `IO` struct encapsulates both the request and response data, allowing
/// middlewares and endpoints to read, modify, and process HTTP requests.
/// It also includes an optional context for sharing data between middlewares and endpoints.
///
/// # Fields
///
/// - `request`: Contains details of the incoming HTTP request, including:
///   - The requested URI (`uri`), which includes the host, path, query parameters, and more.
///   - The HTTP method (`method`), such as `"GET"`, `"POST"`, etc.
///   - The request headers (`headers`), accessible via the `Headers` struct.
///   - The request body (`body`), which may contain both structured data (`serde_json::Value`)
///     and uploaded files. In multipart requests, non-binary data is stored as JSON,
///     while files are stored separately in the `files` field.
///   - The cookies (`cookies`), stored in a structured format (`Cookies`).
///
/// - `response`: Holds the data that will be sent back to the client, including:
///   - The HTTP status code (`status`).
///   - The response headers (`headers`).
///   - The response body (`body`), which is always serialized into JSON unless set to `None`.
///   - The response cookies (`cookies`).
///
/// - `context`: An optional field (`Option<Box<dyn Any + Send + Sync>>`) that can store arbitrary data.
///   Middlewares and endpoints can use this to share information, such as authentication details.
///
/// # Example: Logging Middleware
///
/// This middleware logs the request method and path before passing control to the next middleware or endpoint.
///
/// ```rust
/// use awpak_rs::*;
/// use awpak_rs::io::io::IO;
///
/// #[middleware]
/// fn logging_middleware(mut io: IO) -> MiddlewareResponse {
///     println!("Incoming request: {} {}", io.request.method, io.request.uri.path);
///     MiddlewareResponse::Next(io)
/// }
/// ```
///
/// Middlewares can also modify the response before it is sent to the client:
///
/// ```rust
/// use awpak_rs::*;
/// use awpak_rs::io::io::IO;
/// 
/// #[middleware]
/// fn modify_response_middleware(mut io: IO) -> MiddlewareResponse {
///     io.response.status = 200;
///     io.response.body = Some(serde_json::json!({"message": "Middleware modified this response"}));
///     MiddlewareResponse::Next(io)
/// }
/// ```
pub struct IO
{
    /// The request data, containing information about the HTTP request.
    pub request : RequestData,

    /// The response data, which will be sent back to the client.
    pub response : ResponseData,

    /// An optional context that middlewares and endpoints can use to store and retrieve arbitrary data.
    context : Option<Box<dyn Any + Send + Sync>>
}

impl IO
{
    /// Creates a new `IO` instance with the given request, response, and optional context.
    ///
    /// # Arguments
    ///
    /// * `request` - The incoming request data.
    /// * `response` - The initial response data.
    /// * `context` - An optional boxed value for storing arbitrary data.
    ///
    /// # Returns
    ///
    /// A new `IO` instance.
    pub fn new( request : RequestData, response : ResponseData, context : Option<Box<dyn Any + Send + Sync>> ) -> Self
    {
        Self
        {
            request,
            response,
            context
        }
    }

    /// Creates an `IO` instance with only a response, initializing the request with default values.
    ///
    /// # Arguments
    ///
    /// * `response` - The response data to initialize the `IO` instance.
    ///
    /// # Returns
    ///
    /// An `IO` instance with a default request and the given response.
    pub fn with_response( response : ResponseData ) -> Self
    {
        Self
        {
            request : RequestData::default(),
            response,
            context : None
        }
    }

    /// Sets the context of the `IO` instance to a given value.
    ///
    /// This allows middlewares or endpoints to store arbitrary data in the context.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context value to store. Must implement `Any + Send + Sync`.
    pub fn set_context<T>( &mut self, ctx : T )
    where T: Any + Send + Sync
    {
        self.context = Some( Box::new( ctx ) )
    }

    /// Retrieves a reference to the stored context, if it exists and is of the expected type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected type of the stored context.
    ///
    /// # Returns
    ///
    /// An `Option<&T>` containing a reference to the stored context if it exists and matches the type,
    /// or `None` if there is no context or the type does not match.
    pub fn get_context<T: 'static>( &self ) -> Option<&T>
    {
        match &self.context
        {
            Some( v ) => {
                match v.downcast_ref::<T>()
                {
                    Some( v ) => Some( v ),
                    _ => None
                }
            },
            _ => None
        }
    }

    /// Retrieves a mutable reference to the stored context, if it exists and is of the expected type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected type of the stored context.
    ///
    /// # Returns
    ///
    /// An `Option<&mut T>` containing a mutable reference to the stored context if it exists and matches the type,
    /// or `None` if there is no context or the type does not match.
    pub fn get_context_mut<T: 'static>( &mut self ) -> Option<&mut T>
    {
        match &mut self.context
        {
            Some( v ) => {
                match v.downcast_mut::<T>()
                {
                    Some( v ) => Some( v ),
                    _ => None
                }
            },
            _ => None
        }
    }
}