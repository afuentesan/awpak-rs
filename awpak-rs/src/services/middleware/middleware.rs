use std::sync::OnceLock;

use crate::io::io::IO;

/// Represents the possible outcomes of a middleware execution.
///
/// A middleware can either allow the request to continue processing (`Next`)
/// or stop further processing and return a response immediately (`Cancel`).
pub enum MiddlewareResponse
{
    /// Continues to the next middleware or the endpoint if no more middlewares are left.
    ///
    /// The `IO` instance passed will be forwarded to the next middleware or the endpoint.
    Next( IO ),

    /// Cancels the request processing and immediately returns a response to the client.
    ///
    /// The `IO` instance provided will be used as the final response.
    Cancel( IO )
}

pub type MiddlewareResponseType = std::pin::Pin<std::boxed::Box<
                                    dyn std::future::Future<
                                        Output = MiddlewareResponse
                                    > 
                                    + std::marker::Send
                                >>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MiddlewareExecOrder
{
    PRE,
    POST
}

#[derive(Copy, Clone)]
pub struct Middleware
{
    pub regex : Option<&'static str>,
    pub order : usize,
    pub method : Option<&'static str>,
    pub fnc : fn( IO ) -> MiddlewareResponseType,
    pub exec_order : MiddlewareExecOrder
}

impl Middleware
{
    pub const fn new(
        regex : Option<&'static str>,
        order : usize,
        method : Option<&'static str>,
        fnc : fn( IO ) -> MiddlewareResponseType,
        exec_order : MiddlewareExecOrder
    ) -> Self
    {
        Self
        {
            regex,
            order,
            method,
            fnc,
            exec_order
        }
    }
}

inventory::collect!( Middleware );

fn pre_middlewares() -> &'static Vec<Middleware> {
    static ARRAY_PRE_MIDDLEWARES: OnceLock<Vec<Middleware>> = OnceLock::new();
    ARRAY_PRE_MIDDLEWARES.get_or_init(|| get_init_middlewares( MiddlewareExecOrder::PRE ) )
}

fn post_middlewares() -> &'static Vec<Middleware> {
    static ARRAY_POST_MIDDLEWARES: OnceLock<Vec<Middleware>> = OnceLock::new();
    ARRAY_POST_MIDDLEWARES.get_or_init(|| get_init_middlewares( MiddlewareExecOrder::POST ) )
}

pub fn initialize_middlewares()
{
    let _ = pre_middlewares();
    let _ = post_middlewares();
}

fn get_init_middlewares( exec_order : MiddlewareExecOrder ) -> Vec<Middleware>
{
    let mut middlewares : Vec<Middleware> = vec![];
    
    for m in inventory::iter::<Middleware>
    {
        if m.exec_order == exec_order
        {
            middlewares.push( *m );
        }
    }

    middlewares.sort_by( | a, b | a.order.cmp( &b.order ) );

    middlewares
}

pub async fn pre_middlewares_exec( io : IO ) -> MiddlewareResponse
{
    exec_middlewares( io, MiddlewareExecOrder::PRE ).await
}

pub async fn post_middlewares_exec( io : IO ) -> MiddlewareResponse
{
    exec_middlewares( io, MiddlewareExecOrder::POST ).await
}

async fn exec_middlewares( mut io : IO, exec_order : MiddlewareExecOrder ) -> MiddlewareResponse
{
    let middlewares = match exec_order
    {
        MiddlewareExecOrder::POST => post_middlewares(),
        MiddlewareExecOrder::PRE => pre_middlewares()
    };

    for middleware in middlewares
    {
        if middleware.method.is_some()
        {
            if &io.request.method.to_lowercase().as_str() != middleware.method.as_ref().unwrap()
            {
                continue;
            }
        }

        if middleware.regex.is_some()
        {
            match regex::Regex::new( middleware.regex.as_ref().unwrap() ) {
                
                Ok( v ) => 
                    if v.is_match( &io.request.uri.path ) {} else { continue; },
                _ => { continue; }
            };
        }

        io =  match ( middleware.fnc )( io ).await {
            MiddlewareResponse::Next( v ) => v,
            MiddlewareResponse::Cancel( v ) => return MiddlewareResponse::Cancel( v )
        };
    }

    MiddlewareResponse::Next( io )
}