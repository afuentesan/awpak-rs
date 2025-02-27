use awpak_rs::{io::{headers::headers::Headers, io::IO}, MiddlewareResponse};
use awpak_rs::{middleware, post};

use awpak_rs::{request_headers, request_body, response_headers};

#[post( url = "/post_echo_request_header" )]
fn post_echo_request_header(
    #[request_headers]
    headers : Headers,
    #[request_body]
    names : Vec<String>
) -> String
{
    if names.len() <= 0
    {
        return "None".to_string()
    }

    let mut ret = String::new();

    for name in names
    {
        match headers.get( &name )
        {
            Some( v ) => {
                ret = format!( "{} ({}, {})", ret, v.name, match &v.value {
                    Some( v ) => v,
                    _ => "None"
                } );
            },
            _ => continue
        }
    }

    println!( "{}", prueba_async(  "request header".to_string(), &headers ).await );
    
    ret.trim().to_string()
}

#[post( url = "/post_echo_request_header_add_one" )]
fn post_echo_request_header_add_one(
    #[request_headers]
    mut headers : Headers,
    #[request_body]
    names : Vec<String>
) -> String
{
    if names.len() <= 0
    {
        return "None".to_string()
    }

    for name in &names
    {
        match headers.get_value( name ) {
            Some( val ) => {
                match val.parse::<usize>()
                {
                    Ok( n ) => { headers.replace_header( name.clone(), ( n + 1 ).to_string() ); },
                    _ => {}    
                };
            },
            _ => {}
        };
    }

    let mut ret = String::new();

    for name in names
    {
        match headers.get( &name )
        {
            Some( v ) => {
                ret = format!( "{} ({}, {})", ret, v.name, match &v.value {
                    Some( v ) => v,
                    _ => "None"
                } );
            },
            _ => continue
        }
    }

    println!( "{}", prueba_async_mut(  "add one".to_string(), &mut headers ).await );
    
    ret.trim().to_string()
}

async fn prueba_async( text : String, _headers : &Headers ) -> String
{
    format!( "Prueba async: {}", text )
}

async fn prueba_async_mut( text : String, headers : &mut Headers ) -> String
{
    headers.replace_header( "prueba-async".to_string(), "Prueba async".to_string() );
    
    format!( "Prueba async: {}", text )
}

#[middleware(
    urls=["/post_echo_request_header_add_one", "/post_echo_request_header"],
    execute_after=true
)]
fn middleware_set_request_headers_to_response( mut io : IO ) -> MiddlewareResponse
{
    io.request.headers.iter().for_each( | h | {

        if h.name.trim().to_lowercase() == "content-type" || 
            h.name.trim().to_lowercase() == "content-length" ||
            h.value.is_none()
        {
            return;
        }

        io.response.headers.replace_header( h.name.clone(), h.value.as_ref().unwrap().clone() );

    } );

    MiddlewareResponse::Next( io )
}

#[post( url = "/post_echo_request_header_add_two" )]
fn post_echo_request_header_add_two(
    #[request_headers]
    headers : Headers,
    #[response_headers]
    mut res_headers : Headers,
    #[request_body]
    names : Vec<String>
) -> String
{
    if names.len() <= 0
    {
        return "None".to_string()
    }

    for name in names
    {
        match headers.get_value( &name )
        {
            Some( v ) => match v.parse::<usize>()
            {
                Ok( n ) => {
                    res_headers.replace_header( name, ( n + 2 ).to_string() );
                },
                _ => {}
            },
            _ => {}
        }
    }

    "Some".to_string()
}