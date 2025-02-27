use awpak_rs::tokio;
use cucumber::{given, then, when, World};

mod util;

#[derive(Debug, Default, World)]
struct GetWorld
{
    query : String,
    
    response : Option<String>
}

#[given( expr = "query={string}" )]
fn define_request( world : &mut GetWorld, query : String )
{
    world.query = query;
    
    world.response = None;
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut GetWorld, url : String )
{
    let url = format!( "http://127.0.0.1:3001{}?{}", url, world.query );

    let client = reqwest::Client::new();

    let response = client.get( &url )
    .send()
    .await.unwrap();

    let body = response.text().await;

    if body.is_ok()
    {
        world.response = Some( body.unwrap() );
    }
}

#[then( expr = "response={string}" )]
fn check_result( world : &mut GetWorld, response : String )
{
    assert!( world.response.is_some(), "No response received" );

    assert_eq!( world.response.as_ref().unwrap(), &response );
}

#[tokio::main]
async fn main()
{
    GetWorld::run( "tests/features/test_get_2.feature" ).await;
}