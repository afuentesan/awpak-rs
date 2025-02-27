use awpak_rs::tokio;
use cucumber::{given, then, when, World};

mod util;

#[derive(Debug, Default, World)]
struct QueryParamWorld
{
    query : String,
    
    response : Option<String>
}

#[given( expr = "query={string}" )]
fn define_request( world : &mut QueryParamWorld, query : String )
{
    world.query = query;
    
    world.response = None;
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut QueryParamWorld, url : String )
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
fn check_result( world : &mut QueryParamWorld, response : String )
{
    assert!( world.response.is_some(), "No response received" );

    assert_eq!( world.response.as_ref().unwrap(), &response );
}

#[tokio::main]
async fn main()
{
    QueryParamWorld::run( "tests/features/test_query_param.feature" ).await;
}