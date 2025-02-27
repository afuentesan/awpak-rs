use std::{fs, usize};

use awpak_rs::tokio;
use cucumber::{given, then, when, World};
use util::FileWorld;

mod util;

#[given( regex = r"path=([^,]*), filename=([^,]*), name=([^,]*), content_type=(.*)" )]
fn define_data( world : &mut FileWorld, path : String, filename : String, name : String, content_type : String )
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
            world.path.push( p.to_string() );
        }
    } );

    names.for_each( |p| {
        if p.trim() != ""
        {
            world.name.push( p.to_string() );
        }
    } );

    filenames.for_each( |p| {
        if p.trim() != ""
        {
            world.filename.push( p.to_string() );
        }
    } );

    content_types.for_each( |p| {
        if p.trim() != ""
        {
            world.content_type.push( p.to_string() );
        }
    } );

    world.point = None;
    world.len = None;
}

#[when( regex = r"^I +call +(.+)" )]
async fn call_url( world : &mut FileWorld, url : String )
{
    let url = format!( "http://127.0.0.1:3001{}", url );

    let mut form = reqwest::multipart::Form::new();

    for i in 0..world.path.len()
    {
        let path = world.path[ i ].clone();
        let name = world.name[ i ].clone();
        let filename = world.filename[ i ].clone();
        let content_type = world.content_type[ i ].clone();

        let file = fs::read( path ).unwrap();
    
        let file_part = reqwest::multipart::Part::bytes(file)
            .file_name( filename )
            .mime_str( &content_type )
            .unwrap();
        
        form = form.part( name, file_part );
    }

    let client = reqwest::Client::new();

    let body = client.post( &url )
    .multipart( form )
    .send()
    .await.unwrap()
    .text().await.unwrap();

    let len = body.parse::<usize>();

    if len.is_ok()
    {
        world.len = Some( len.unwrap() )
    }
}

#[then( expr = "len={int}" )]
fn check_result( world : &mut FileWorld, len : usize )
{
    assert_eq!( world.len.unwrap(), len );
}

#[tokio::main]
async fn main()
{
    FileWorld::run( "tests/features/test_multipart_files.feature" ).await;
}