use awpak_rs::tokio;
use cucumber::{given, then, when, World};
use reqwest::{redirect::Policy, Client};

mod util;

#[derive(Debug, Default, World)]
struct RedirectsWorld
{
    location : String,
    status : u16,
    
    response_location : Option<String>,
    response_status : Option<u16>
}

#[given( expr = "location={string} status={string}" )]
fn define_request( world : &mut RedirectsWorld, location : String, status : String )
{
    world.location = location;
    world.status = if status.trim() == ""
    {
        0
    }
    else
    {
        status.parse().unwrap()
    };
    
    world.response_location = None;
    world.response_status = None;
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut RedirectsWorld, url : String )
{
    let url = format!( "http://127.0.0.1:3001{}?url={}&status={}", url, world.location, world.status );

    let client = Client::builder().redirect(Policy::none()).build().unwrap();

    let response = client.get( &url )
    .send()
    .await.unwrap();

    world.response_status = Some( response.status().as_u16() );
    
    let response_location = response.headers().get( "location" );

    assert!( response_location.is_some() );

    let response_location = response_location.unwrap();

    let response_location = response_location.to_str();

    assert!( response_location.is_ok() );

    world.response_location = Some( response_location.unwrap().into() );
}

#[then( expr = "location={string} status={string}" )]
fn check_result( world : &mut RedirectsWorld, location : String, status : String )
{
    assert!( world.response_location.is_some(), "No response location received" );
    assert!( world.response_status.is_some(), "No response status received" );

    assert_eq!( world.response_location.as_ref().unwrap(), &location );
    assert_eq!( world.response_status.as_ref().unwrap(), &status.parse::<u16>().unwrap() );
}

#[tokio::main]
async fn main()
{
    RedirectsWorld::run( "tests/features/test_redirects.feature" ).await;
}