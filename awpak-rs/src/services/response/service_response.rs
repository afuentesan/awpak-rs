
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{body::Bytes, header::{HeaderName, HeaderValue, SET_COOKIE}, Response};

use crate::{io::{io::IO, response::response_data::ResponseData}, ResponseContentTypeStrategy};

const MIME_TYPES_AVAILABLES : &[&str] = &[ "text/plain", "application/json" ];

pub fn get_initial_response() -> ResponseData
{
    ResponseData::default()
}

pub fn generate_response( io : &mut IO ) -> Response<BoxBody<Bytes, hyper::Error>>
{
    let content_type = set_content_type( io );

    let body = get_body_response( &mut io.response, &content_type );

    io.response.headers.replace_header( "content-type".to_string(), content_type.clone() );

    let mut response : hyper::http::response::Builder = Response::builder()
        .status( io.response.status );

    append_headers( &mut response, &mut io.response );

    append_cookies( &mut response, &mut io.response );

    let response = response.body( Full::new( body ).map_err(|never| match never {}).boxed() );

    if response.is_err()
    {
        let response = Response::builder()
            .status( 500 )
            .body( 
                Full::new( b"".to_vec().into() )
                    .map_err(|never| match never {})
                    .boxed() 
            );

        return response.unwrap();
    }

    response.unwrap()
}



fn set_content_type( io : &mut IO ) -> String
{
    if io.response.headers.has( "content-type" ) &&
        MIME_TYPES_AVAILABLES.contains( &io.response.headers.get_value( "content-type" ).as_ref().unwrap().as_str() )
    {
        return io.response.headers.get_value( "content-type" ).unwrap().clone()
    }

    let mime_type = io.request.headers.content_negotiation( MIME_TYPES_AVAILABLES );

    if mime_type.is_none()
    {
        io.response.headers.replace_header( "content-type".to_string(), MIME_TYPES_AVAILABLES[ 0 ].to_string() );
    }
    else
    {
        io.response.headers.replace_header( "content-type".to_string(), mime_type.as_ref().unwrap().clone() );
    }

    io.response.headers.get_value( "content-type" ).unwrap().clone()
}

fn append_cookies( response : &mut hyper::http::response::Builder, response_data : &ResponseData )
{
    response_data.cookies.iter_all().for_each( | cookies | {

        for cookie in cookies
        {
            match HeaderValue::from_str( cookie )
            {
                Ok( v ) => response.headers_mut().unwrap().append(
                    SET_COOKIE, 
                    v
                ),
                _ => continue
            };
        }
    } );
}

fn append_headers( response : &mut hyper::http::response::Builder, response_data : &ResponseData )
{
    response_data.headers.iter_all().for_each( | headers |
        {
            for header_data in headers
            {
                let header_value = HeaderValue::from_bytes( &header_data.value_bytes[..] );

                if header_value.is_ok()
                {
                    let header_name = HeaderName::try_from( &header_data.name );

                    if header_name.is_ok()
                    {
                        response.headers_mut().unwrap().append( 
                            header_name.unwrap(), 
                            header_value.unwrap()
                        );
                    }
                }
            }
        }
    );
}

fn get_body_response( response_data : &ResponseData, content_type : &String ) -> Bytes
{
    let out = ResponseContentTypeStrategy::exec( 
        &content_type, 
        match response_data.body.clone()
        {
            Some( v ) => v,
            _ => serde_json::to_value( "" ).unwrap()
        }
    );

    match out
    {
        Ok( v ) => match v
        {
            Ok( v ) => v,
            _ => "".into()    
        },
        _ => "".into()
    }
}