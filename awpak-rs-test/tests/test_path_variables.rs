use awpak_rs::tokio;
use cucumber::{then, when, World};

mod util;

#[derive(Debug, Default, World)]
struct PathVariableWord
{
    response : Option<String>
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut PathVariableWord, url : String )
{
    world.response = None;

    let url = format!( "http://127.0.0.1:3001{}", url );

    let client = reqwest::Client::new();

    let response = client.get( &url )
    .send()
    .await.unwrap();

    let body = response.text().await;

    world.response = match body
    {
        Ok( v ) => Some( v ),
        _ => None
    };
}

#[then( expr = "response={string}" )]
fn check_result( world : &mut PathVariableWord, response : String )
{
    assert!( world.response.is_some(), "No response received" );

    assert_eq!( world.response.as_ref().unwrap(), &response );
}

#[tokio::main]
async fn main()
{
    PathVariableWord::run( "tests/features/test_path_variables.feature" ).await;
}