
use std::{ops::Add, str::FromStr};

use awpak_rs::{get, post, DeserializeWithIO};
use serde::{Deserialize, Serialize};

use crate::Point;

#[derive(Serialize, Deserialize, DeserializeWithIO)]
pub struct PointWithContextWrapper
{
    #[io_deserializer( deserialize_with = deserialize_y_with_context )]
    x : f32,
    y : f32,
    #[serde(default)]
    point : Option<PointWithContext>
}

#[derive(Serialize, Deserialize, DeserializeWithIO)]
pub struct PointWithContext
{
    x : f32,
    #[io_deserializer( deserialize_with = deserialize_y_with_context )]
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
    #[query_params]
    point : PointWithContext
) -> PointWithContext
{
    point
}

#[get( url = "/get_point_with_context_wrapper" )]
pub fn get_point_with_context_wrapper(
    #[query_params]
    point : PointWithContextWrapper
) -> PointWithContextWrapper
{
    point
}

#[post( url = "/post_point_with_context" )]
pub fn post_point_with_context(
    #[request_body]
    point : PointWithContext
) -> PointWithContext
{
    point
}

#[post( url = "/post_point_with_context_wrapper" )]
pub fn post_point_with_context_wrapper(
    #[request_body]
    point : PointWithContextWrapper
) -> PointWithContextWrapper
{
    point
}