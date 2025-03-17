use awpak_rs::{get, io::io::IO, post};
use serde::Serialize;

#[derive(Serialize)]
struct Response
{
    status : u16
}

async fn request_body_from_async_str( io : &IO, s : &str ) -> Result<Response, String>
{
    let s : Vec<u8> = s.as_bytes().iter().filter( | b | **b >= 48 && **b <= 57 ).map( | b | *b ).collect();

    let s = std::str::from_utf8( &s[..] ).map_err( | e | e.to_string() )?;

    match s.trim().parse::<u16>()
    {
        Ok( x ) => Ok( Response { status : x + io.response.status } ),
        _ => Err( "Err parse to u16".to_string() )
    }
}

#[post( url = "/post_request_body_from_async_str" )]
fn post_request_body_from_async_str(
    #[request_body(deserialize_with = request_body_from_async_str)]
    data : Response
) -> Response
{
    data
}

#[get( url = "/get_query_params_from_async_str" )]
fn get_query_params_from_async_str(
    #[query_params(deserialize_with = request_body_from_async_str)]
    data : Response
) -> Response
{
    data
}