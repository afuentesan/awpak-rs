use std::{fs, usize};

use awpak_rs::tokio;
use cucumber::{given, then, when, World};

mod util;

#[derive(Debug, Default, World)]
pub struct MultipartDataWorld
{
    pub path : Vec<String>,
    pub name : Vec<String>,
    pub filename : Vec<String>,
    pub content_type : Vec<String>,

    pub param_1 : String,
    pub param_2 : String,

    pub response : Option<String>
}

#[given( expr = "path={string}, filename={string}, name={string}, content_type={string}, param_1={string}, param_2={string}" )]
fn define_data( world : &mut MultipartDataWorld, path : String, filename : String, name : String, content_type : String, param_1 : String, param_2 : String )
{
    world.path = vec![];
    world.name = vec![];
    world.filename = vec![];
    world.content_type = vec![];

    let paths = path.trim().split( ";" );
    let names = name.trim().split( ";" );
    let filenames = filename.trim().split( ";" );
    let content_types = content_type.trim().split( ";" );

    paths.for_each( |p| {
        if p.trim() != ""
        {
            world.path.push( p.trim().to_string() );
        }
    } );

    names.for_each( |p| {
        if p.trim() != ""
        {
            world.name.push( p.trim().to_string() );
        }
    } );

    filenames.for_each( |p| {
        if p.trim() != ""
        {
            world.filename.push( p.trim().to_string() );
        }
    } );

    content_types.for_each( |p| {
        if p.trim() != ""
        {
            world.content_type.push( p.trim().to_string() );
        }
    } );

    assert!( 
        world.path.len() == world.name.len() &&
        world.path.len() == world.filename.len() &&
        world.path.len() == world.content_type.len()
    );

    world.param_1 = param_1;
    world.param_2 = param_2;

    world.response = None;
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut MultipartDataWorld, url : String )
{
    let url = format!( "http://127.0.0.1:3001{}", url );

    let mut form = reqwest::multipart::Form::new();

    for i in 0..world.path.len()
    {
        let path = world.path[ i ].clone();

        if path != ""
        {
            let filename = world.filename[ i ].clone();
            let content_type = world.content_type[ i ].clone();
            let name = world.name[ i ].clone();

            match fs::read( path ) {
                Ok( file ) => {
                    let file_part = reqwest::multipart::Part::bytes(file)
                        .file_name( filename )
                        .mime_str( &content_type )
                        .unwrap();
                    
                    form = form.part( name, file_part );
                },
                _ => {}
            };
        }
    }

    if world.param_1 != ""
    {
        form = form.part( "param_1", reqwest::multipart::Part::text( world.param_1.clone() ) );
    }

    if world.param_2 != ""
    {
        form = form.part( "param_2", reqwest::multipart::Part::text( world.param_2.clone() ) );
    }

    let client = reqwest::Client::new();

    let body = client.post( &url )
    .multipart( form )
    .send()
    .await.unwrap()
    .text().await.unwrap();

    world.response = Some( body );
}

#[then( expr = "response={string}" )]
fn check_result( world : &mut MultipartDataWorld, response : String )
{
    assert!( world.response.is_some(), "No response data received" );
    assert_eq!( world.response.as_ref().unwrap(), &response );
}

#[tokio::main]
async fn main()
{
    MultipartDataWorld::run( "tests/features/test_multipart_data.feature" ).await;
}