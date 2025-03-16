
use std::{ops::Add, str::FromStr, sync::{Arc, Mutex}};

use awpak_rs::{get, io::{deserializer::deserialize_with_io::DeserializeWithIO, io::IO, request::request_data::RequestData, response::response_data::ResponseData}, DeserializeWithIO};
use serde::{Deserialize, Serialize};

use crate::Point;


#[derive(Serialize, Deserialize, DeserializeWithIO)]
pub struct PointWithContext
{
    x : f32,
    #[io_deserializer(deserialize_y_with_context)]
    y : f32,
    #[serde(default)]
    point : Option<Point>
}

async fn deserialize_y_with_context<T>( input : String, io : &awpak_rs::io::io::IO ) -> Result<T, String>
where T: FromStr + Add<Output = T>
{
    let status = match io.response.status.to_string().trim().parse::<T>()
    {
        Ok( s ) => s,
        _ => return Err( "".to_string() )
    };

    match input.trim().parse::<T>()
    {
        Ok( v ) => Ok( v + status ),
        Err( _ ) => Err( "".to_string() )
    }
}

#[get( url = "/get_point_with_context" )]
pub fn get_point_with_context(
    #[query_param]
    _input : String
) -> PointWithContext
{
    PointWithContext { x : 0.0, y : 0.0, point : None }
}

#[derive(Deserialize, DeserializeWithIO)]
struct Prueba
{
    #[io_deserializer(deserialize_y_with_context)]
    x : f32,
    y : f32
}

#[get( url = "/get_prueba_deserialize_with_io")]
fn get_prueba_deserialize_with_io() -> String
{
    let bytes = r#"{"x":2,"y":3}"#.as_bytes().to_vec();

    let mut io = IO::new( RequestData::default(), ResponseData::default(), None );

    io.request.body.data = Arc::new( bytes.into() );

    let result = Prueba::deserialize_with_io( Arc::clone( &io.request.body.data ), Arc::new( Mutex::new( Some( io ) ) ) );

    match result
    {
        Ok( p ) => format!( "x:{}, y:{}", p.x, p.y ),
        _ => "Err".to_string()
    }
}

#[get( url = "/get_prueba_deserialize_context")]
fn get_prueba_deserialize_context(
    #[request_body]
    prueba : Prueba
) -> String
{
    format!( "x:{}, y:{}", prueba.x, prueba.y )
}

#[get( url = "/get_prueba_deserialize_context_vec")]
fn get_prueba_deserialize_context_vec(
    #[request_body]
    prueba : Vec<Prueba>
) -> String
{
    let mut ret = String::new();

    for p in &prueba
    {
        ret = format!( "{}, x:{}::y:{}", ret, p.x, p.y );
    } 

    format!( "xises: {}", ret )
}

#[get( url = "/get_option_prueba_deserialize_context")]
fn get_option_prueba_deserialize_context(
    #[request_body]
    prueba : Option<Prueba>
) -> String
{
    match prueba
    {
        Some( p ) => format!( "x:{}, y:{}", p.x, p.y ),
        _ => "None".to_string()    
    }
}

#[derive(Deserialize, DeserializeWithIO, Serialize)]
struct PruebaWrapper
{
    #[io_deserializer(deserialize_y_with_context)]
    x : f32,
    y : f32,
    inner : PruebaInner
}

#[derive(Deserialize, DeserializeWithIO, Serialize)]
struct PruebaInner
{
    x : u32,
    #[io_deserializer(deserialize_y_with_context)]
    y : u32
}

#[get( url = "/get_pruebas_async_deserializer")]
fn get_pruebas_async_deserializer(
    #[request_body]
    prueba : PruebaWrapper
) -> PruebaWrapper
{
    prueba
}

#[derive(Deserialize, DeserializeWithIO, Serialize)]
struct PruebaString
{
    #[io_deserializer(prueba_string)]
    name : String
}

async fn prueba_string( input : String, io : &awpak_rs::io::io::IO ) -> Result<String, String>
{
    Ok( format!( "{}: {}", input, io.response.status ) )
}

#[get( url = "/get_prueba_string_async_deserializer")]
fn get_prueba_string_async_deserializer(
    #[request_body]
    prueba : PruebaString
) -> PruebaString
{
    prueba
}