use awpak_rs::tokio;
use cucumber::{given, then, when, World};

mod util;

#[allow(dead_code)]
#[derive(Debug, Default, World)]
struct ContentNegotiationWorld
{
    accept : String,
    response : Option<String>
}

#[given( expr = "accept={string}" )]
fn define_x_y( world : &mut ContentNegotiationWorld, accept : String )
{
    world.accept = accept;

    world.response = None;
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut ContentNegotiationWorld, url : String )
{
    let url = format!( 
        "http://127.0.0.1:3001{}", 
        url
    );

    let client = reqwest::Client::new();

    let response = client.get( &url )
    .header( "Accept", world.accept.clone() )
    .send()
    .await.unwrap();

    let content_type = response.headers().get( "content-type" );

    if content_type.is_none()
    {
        return;
    }

    let content_type = content_type.unwrap().to_str();

    if content_type.is_err()
    {
        return;
    }

    world.response = Some( content_type.unwrap().to_string() );
}

#[then( expr = "content_type={string}" )]
fn check_result( world : &mut ContentNegotiationWorld, content_type : String )
{
    assert!( world.response.is_some(), "No content type received" );

    assert_eq!( world.response.as_ref().unwrap(), &content_type );
}

#[tokio::main]
async fn main()
{
    ContentNegotiationWorld::run( "tests/features/test_content_negotiation.feature" ).await;
}