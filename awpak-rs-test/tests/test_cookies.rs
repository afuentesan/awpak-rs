use awpak_rs::tokio;
use cucumber::{given, then, when, World};

mod util;

#[derive(Debug, Default, World)]
struct CookiesWorld
{
    cookies : Vec<(String, String)>,
    names : String,

    response : Option<String>
}

#[given( expr = "names={string}, values={string}" )]
fn define_request( world : &mut CookiesWorld, names : String, values : String )
{
    world.cookies = vec![];

    let arr_names = names.split( "," );
    let values = values.split( "," );

    let mut str_names = "".to_string();

    arr_names.zip( values ).filter( | v | v.0 != "" ).for_each( | v | {
        world.cookies.push( ( v.0.to_string(), v.1.to_string() ) );
        str_names = format!( r#"{},"{}""#, str_names.trim(), v.0 );
    } );
    
    let str_names = str_names[1..].to_string();

    world.names = format!( "[{}]", str_names );

    world.response = None;
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut CookiesWorld, url : String )
{
    let url = format!( "http://127.0.0.1:3001{}", url );

    let client = reqwest::Client::new();

    let mut response = client.post( &url )
    .body( world.names.clone() )
    .header( "Content-Type", "application/json" );

    for header in &world.cookies
    {
        let val = format!( "{}={}", header.0, header.1 );

        response = response.header( "Cookie", val );
    }
    
    let response = response.send()
    .await.unwrap();

    let headers = response.headers();

    let mut headers_str = String::new();

    for header in headers
    {
        if header.0.to_string().trim().to_lowercase() == "set-cookie"
        {
            match header.1.to_str() {
                Ok( v ) => {
                    headers_str = format!( "{} ({})", headers_str, v );
                },
                _ => {}
            }
        }
    }

    world.response = Some( headers_str.trim().to_string() );
}

#[then( expr = "response={string}" )]
fn check_result( world : &mut CookiesWorld, response : String )
{
    assert!( world.response.is_some(), "No response received" );

    assert_eq!( world.response.as_ref().unwrap(), &response );
}

#[tokio::main]
async fn main()
{
    CookiesWorld::run( "tests/features/test_cookies.feature" ).await;
}