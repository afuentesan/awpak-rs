use awpak_rs::tokio;
use cucumber::{given, then, when, World};
use util::{Optionf32, Point, PointWorld};

mod util;

#[given( expr = "x={optionf32}, y={float}" )]
fn define_x_y( world : &mut PointWorld, x : Optionf32, y : f32 )
{
    world.point.x = x.number;
    world.point.y = y;

    world.received_point = None;
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut PointWorld, url : String )
{
    let url = if url.contains( "query_string" )
    {
        format!( 
            "http://127.0.0.1:3001{}?x={}&y={}", 
            url, 
            match world.point.x
            {
                Some( v ) => format!( "{}", v ),
                _ => "".to_string()
            }, 
            world.point.y 
        )
    }
    else
    {
        format!( "http://127.0.0.1:3001{}", url )
    };

    let request_body = if url.contains( "query_string" )
    {
        "".to_string()
    }
    else
    {
        serde_json::to_string( &world.point ).unwrap()
    };

    let client = reqwest::Client::new();

    let response = client.post( &url )
    .body( request_body )
    .header( "Content-Type", "application/json" )
    .header( "Accept", "application/json" )
    .send()
    .await.unwrap();

    assert!( response.headers().get( "content-type" ).unwrap().to_str().unwrap() == "application/json" );

    let body = response.text().await.unwrap();

    let received_point : Point = serde_json::from_str( &body ).unwrap();

    world.received_point = Some( received_point );
}

#[then( expr = "x={optionf32}, y={float}" )]
fn check_result( world : &mut PointWorld, x : Optionf32, y : f32 )
{
    assert!( world.received_point.is_some(), "No point received" );

    assert_eq!( world.received_point.as_ref().unwrap().x, x.number );
    assert_eq!( world.received_point.as_ref().unwrap().y, y );
}

#[tokio::main]
async fn main()
{
    PointWorld::run( "tests/features/test_post.feature" ).await;
}