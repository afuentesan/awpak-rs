use awpak_rs::tokio;
use cucumber::{given, then, when, World};

mod util;

#[derive(Debug, Default, World)]
struct PostRequestBodyWorld
{
    request_body : String,
    content_type : String,
    response : Option<String>
}

#[given( expr = "request_body={string}, content_type={string}" )]
fn define_request( world : &mut PostRequestBodyWorld, request_body : String, content_type : String )
{
    world.request_body = request_body;
    world.content_type = content_type;
    
    world.response = None;
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut PostRequestBodyWorld, url : String )
{
    let url = format!( "http://127.0.0.1:3001{}", url );

    let client = reqwest::Client::new();

    let response = client.post( &url )
    .body( world.request_body.clone() )
    .header( "Content-Type", &world.content_type )
    .send()
    .await.unwrap();

    let body = response.text().await;

    if body.is_ok()
    {
        world.response = Some( body.unwrap() );
    }
}

#[then( expr = "response={string}" )]
fn check_result( world : &mut PostRequestBodyWorld, response : String )
{
    assert!( world.response.is_some(), "No response received" );

    assert_eq!( world.response.as_ref().unwrap(), &response );
}

#[tokio::main]
async fn main()
{
    PostRequestBodyWorld::run( "tests/features/test_post_2.feature" ).await;
}