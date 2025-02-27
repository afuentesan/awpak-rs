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
    let url = format!( "http://127.0.0.1:3001{}", url );

    let mut form = reqwest::multipart::Form::new();

    if world.point.x.is_some()
    {
        let x_part = reqwest::multipart::Part::text( world.point.x.as_ref().unwrap().to_string() );

        form = form.part( "x", x_part );
    }

    let y_part = reqwest::multipart::Part::text( world.point.y.to_string() );

    form = form.part( "y", y_part );

    let client = reqwest::Client::new();

    let body = client.post( &url )
    .multipart( form )
    .send()
    .await.unwrap()
    .text().await.unwrap();

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
    PointWorld::run( "tests/features/test_multipart_json.feature" ).await;
}