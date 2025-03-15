use std::future::IntoFuture as _;

use crate::{endpoint::{endpoint::Endpoint, types::EndpointReturnType}, io::io::IO, error::error::Error, util::url_utils::{get_regex, normalize_url}};

pub async fn endpoint_exec( io : IO ) -> Result<IO, Error>
{
    match get_endpoint( &io.request.uri.path, &io.request.method )
    {
        Ok( e ) => e( io ).into_future().await,
        Err( e ) => Err( e )
    }
}

fn get_endpoint<'a>( url : &'a str, method : &'a str ) -> Result<fn( IO ) -> EndpointReturnType, Error>
{
    let url = &normalize_url( url );

    for endpoint in inventory::iter::<Endpoint>
    {
        match endpoint_url_method_match( endpoint, url, method )
        {
            Ok( r ) => if r { return Ok( endpoint.fnc ) },
            Err( e ) => return Err( e )
        };
    }

    Err( Error::EndpointNotFound( format!( "Endpoint not found: {}", url ) ) )
}

fn endpoint_url_method_match( endpoint : &Endpoint, url : &str, method : &str ) -> Result<bool, Error>
{
    if endpoint.method.to_uppercase().trim() != method.to_uppercase().trim()
    {
        return Ok( false );
    }

    let endpoint_url = normalize_url( &endpoint.url );

    match get_regex( &endpoint_url )
    {
        Ok( r ) => match r {
            Some( r ) => if r.is_match( &url ) { Ok( true ) } else { Ok( false ) },
            _ => if endpoint_url == url { Ok( true ) } else { Ok( false ) }
        },
        Err( e ) => Err( e ) 
    }
}
