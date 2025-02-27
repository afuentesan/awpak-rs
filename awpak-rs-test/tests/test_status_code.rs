use awpak_rs::tokio;
use cucumber::{given, then, when, World};

mod util;

#[derive(Debug, Default, World)]
struct StatusCodeWorld
{
    status : u16,

    response : Option<u16>
}

#[given( expr = "status={string}" )]
fn define_request( world : &mut StatusCodeWorld, status : String )
{
    world.status = status.parse::<u16>().unwrap();

    world.response = None;
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut StatusCodeWorld, url : String )
{
    let url = format!( "http://127.0.0.1:3001{}", url );

    let client = reqwest::Client::new();

    let body = format!( r#"{{"status":{}}}"#, world.status );

    let response = client.post( &url )
    .body( body )
    .header( "Content-Type", "application/json" )
    .send()
    .await.unwrap();

    let status = response.status().as_u16();

    world.response = Some( status );
}

#[then( expr = "response={string}" )]
fn check_result( world : &mut StatusCodeWorld, response : String )
{
    assert!( world.response.is_some(), "No response received" );

    let response = response.parse::<u16>();

    assert!( response.is_ok(), "Response not parse as u16" );

    assert_eq!( world.response.as_ref().unwrap(), response.as_ref().unwrap() );
}

#[tokio::main]
async fn main()
{
    StatusCodeWorld::run( "tests/features/test_status_code.feature" ).await;
}