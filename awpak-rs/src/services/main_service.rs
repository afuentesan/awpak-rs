use std::future::Future;
use std::pin::Pin;

use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::{Request, Response};

use crate::io::cookies::cookies::Cookies;
use crate::io::headers::headers::Headers;
use crate::io::io::IO;
use crate::io::response::response_data::ResponseData;
use crate::server::server::ServerParams;
use crate::services::response::service_response::get_initial_response;

use super::endpoint::endpoint_executor::endpoint_exec;
use super::middleware::middleware::{post_middlewares_exec, pre_middlewares_exec, MiddlewareResponse};
use super::request::service_request::get_request_data;
use super::response::service_response::generate_response;

pub fn main_service_fn( server_params : ServerParams<'static> ) -> impl Fn(hyper::Request<hyper::body::Incoming>) 
-> Pin<
        Box<
            dyn Future<
                Output = Result<Response<BoxBody<hyper::body::Bytes, hyper::Error>>, hyper::Error>
            >
            + std::marker::Send
        >
    >
{
    move | r | {
        
        Box::pin(
            async move {
                
                main_service( r, server_params ).await
            }
        )
    }
}

async fn main_service( req: Request<hyper::body::Incoming>, server_params : ServerParams<'_> ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>
{
    let io = get_initial_io( req, server_params ).await?;

    let io = match pre_middlewares_exec( io ).await
    {
        MiddlewareResponse::Next( v ) => v,
        MiddlewareResponse::Cancel( mut v ) => return Ok( generate_response( &mut v ) )
    };

    let endpoint_response = endpoint_exec( io ).await;

    let io = match endpoint_response
    {
        Ok( s ) => s,
        Err( e ) => {

            eprintln!( "{:?}", e );
            
            IO::with_response( ResponseData::new( e.get_status_code(), Headers::new(), Cookies::new(), Some( serde_json::to_value( "Server Error" ).unwrap() ) ) )
        }
    };

    let mut io = match post_middlewares_exec( io ).await
    {
        MiddlewareResponse::Next( v ) => v,
        MiddlewareResponse::Cancel( mut v ) => return Ok( generate_response( &mut v ) )
    };

    Ok( generate_response( &mut io ) )
}

async fn get_initial_io( req: Request<hyper::body::Incoming>, _server_params : ServerParams<'_> ) -> Result<IO, hyper::Error>
{
    let request_data = get_request_data( req ).await?;

    let response_data = get_initial_response();

    Ok( IO::new( request_data, response_data, None ) )
}


