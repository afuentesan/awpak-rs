use awpak_rs::tokio;
use cucumber::{given, then, when, World};

mod util;

#[derive(Debug, Default, World)]
struct HeadersWorld
{
    headers : Vec<(String, String)>,
    names : String,

    response : Option<String>,
    response_headers : Option<String>
}

#[given( expr = "names={string}, values={string}" )]
fn define_request( world : &mut HeadersWorld, names : String, values : String )
{
    world.headers = vec![];

    let arr_names = names.split( "," );
    let values = values.split( "," );

    let mut str_names = "".to_string();

    arr_names.zip( values ).filter( | v | v.0 != "" ).for_each( | v | {
        world.headers.push( ( v.0.to_string(), v.1.to_string() ) );
        str_names = format!( r#"{},"{}""#, str_names.trim(), v.0 );
    } );
    
    let str_names = str_names[1..].to_string();

    world.names = format!( "[{}]", str_names );

    world.response = None;
    world.response_headers = None;
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut HeadersWorld, url : String )
{
    let url = format!( "http://127.0.0.1:3001{}", url );

    let client = reqwest::Client::new();

    let mut response = client.post( &url )
    .body( world.names.clone() )
    .header( "Content-Type", "application/json" );

    for header in &world.headers
    {
        response = response.header( header.0.clone(), header.1.clone() );
    }
    
    let response = response.send()
    .await.unwrap();

    let headers = response.headers();

    let mut headers_str = String::new();

    for header in &world.headers
    {
        let h = headers.get( &header.0 );

        if h.is_some()
        {
            if h.as_ref().unwrap().to_str().is_ok()
            {
                headers_str = format!( "{} ({}, {})", headers_str, header.0, h.as_ref().unwrap().to_str().unwrap() );
            }
            
        }
    }

    world.response_headers = Some( headers_str.trim().to_string() );

    let body = response.text().await;

    if body.is_ok()
    {
        world.response = Some( body.unwrap() );
    }
}

#[then( expr = "response={string}, response_headers={string}" )]
fn check_result( world : &mut HeadersWorld, response : String, response_headers : String )
{
    assert!( world.response.is_some(), "No response received" );

    assert_eq!( world.response.as_ref().unwrap(), &response );

    assert!( world.response_headers.is_some(), "No response headers received" );

    assert_eq!( world.response_headers.as_ref().unwrap(), &response_headers );
}

#[tokio::main]
async fn main()
{
    HeadersWorld::run( "tests/features/test_headers.feature" ).await;
}