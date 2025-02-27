use awpak_rs::tokio;
use cucumber::{given, then, when, World};
use util::{OptionParam, ParamWorld};

mod util;

#[given( expr = "param={optionparam}" )]
fn define_param( world : &mut ParamWorld, param : OptionParam )
{
    world.param = param.param;

    world.received_param = None;
}

#[when( regex = r"^I +call +param +(.+)" )]
async fn call_url_param( world : &mut ParamWorld, url : String )
{
    let url = format!( "http://127.0.0.1:3001{}", url );

    let request_body = match &world.param
    {
        Some( v ) => format!( r#"{{"param":{}}}"#, v ),
        _ =>  format!( r#"{{"param":null}}"# )  
    };

    let client = reqwest::Client::new();

    let body = client.post( &url )
    .body( request_body )
    .header( "Content-Type", "application/json" )
    .send()
    .await.unwrap()
    .text().await.unwrap();

    world.received_param = Some( body );
}

#[then( expr = "param={optionparam}" )]
fn check_result_param( world : &mut ParamWorld, param : OptionParam )
{
    assert!( world.received_param.is_some(), "No param received" );

    assert_eq!( world.received_param.as_ref().unwrap(), &param.param.unwrap() );
}

#[tokio::main]
async fn main()
{
    ParamWorld::run( "tests/features/test_post_params.feature" ).await;
}