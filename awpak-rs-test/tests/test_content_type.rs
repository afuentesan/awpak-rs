use awpak_rs::tokio;
use cucumber::{given, then, when, World};

mod util;

#[allow(dead_code)]
#[derive(Debug, Default, World)]
struct ContentTypeWorld
{
    accept : String,
    content_type : String,
    data : String,
    
    content_type_response : Option<String>,
    response : Option<String>
}

#[given( expr = "accept={string}, content_type={string}, data={string}" )]
fn define_x_y( world : &mut ContentTypeWorld, accept : String, content_type : String, data : String )
{
    world.accept = accept;
    world.content_type = content_type;
    world.data = data;

    world.content_type_response = None;
    world.response = None;
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut ContentTypeWorld, url : String )
{
    let url = format!( 
        "http://127.0.0.1:3001{}", 
        url
    );

    let client = reqwest::Client::new();

    let response = client.post( &url )
    .body( world.data.clone() )
    .header( "Accept", world.accept.clone() )
    .header( "Content-Type", world.content_type.clone() )
    .send()
    .await.unwrap();

    let content_type = response.headers().get( "content-type" );

    if content_type.is_some()
    {
        let content_type = content_type.unwrap().to_str();

        if content_type.is_ok()
        {
            world.content_type_response = Some( content_type.unwrap().to_string() );
        }
    }

    let body = response.text().await;

    if body.is_ok()
    {
        world.response = Some( body.unwrap() );
    }
}

#[then( expr = "content_type={string}, data={string}" )]
fn check_result( world : &mut ContentTypeWorld, content_type : String, data : String )
{
    assert!( world.content_type_response.is_some(), "No content-type received" );
    assert!( world.response.is_some(), "No response received" );

    assert_eq!( world.content_type_response.as_ref().unwrap(), &content_type );
    assert_eq!( world.response.as_ref().unwrap(), &data );
}

#[tokio::main]
async fn main()
{
    ContentTypeWorld::run( "tests/features/test_content_type.feature" ).await;
}