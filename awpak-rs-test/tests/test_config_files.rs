use awpak_rs::tokio;
use cucumber::{then, when, World};

mod util;

#[allow(dead_code)]
#[derive(Debug, Default, World)]
struct ConfigFilesWorld
{
    response : Option<String>
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut ConfigFilesWorld, url : String )
{
    let url = format!( 
        "http://127.0.0.1:3001{}", 
        url
    );

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
fn check_result( world : &mut ConfigFilesWorld, response : String )
{
    assert!( world.response.is_some(), "No response received" );

    assert_eq!( world.response.as_ref().unwrap(), &response );
}

#[tokio::main]
async fn main()
{
    ConfigFilesWorld::run( "tests/features/test_config_files.feature" ).await;
}