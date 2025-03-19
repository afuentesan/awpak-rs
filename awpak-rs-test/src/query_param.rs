use awpak_rs::{get, io::io::IO};

use crate::Point;

#[get( url = "/get_echo_params_a_b" )]
fn get_echo_params_a_b(
    #[query_param]
    a : u16,
    #[query_param]
    b : u16
) -> String
{
    format!( "a={}&b={}", a, b )
}

#[get( url = "/get_echo_param_point" )]
fn get_echo_param_point(
    #[query_param]
    point : Point
) -> String
{
    format!( "x={}, y={}", match point.x
    {
        Some( v ) => v,
        None => 0.0
    }, point.y )
}

#[get( url = "/get_echo_param_string" )]
fn get_echo_param_string(
    #[query_param]
    a : String
) -> String
{
    format!( "a={a}" )
}

#[get( url = "/get_echo_param_string_change_name" )]
fn get_echo_param_string_change_name(
    #[query_param(name="a_renamed")]
    a : String
) -> String
{
    format!( "a={a}" )
}

#[get( url = "/get_echo_param_option_string" )]
fn get_echo_param_option_string(
    #[query_param]
    a : Option<String>
) -> String
{
    format!( "a={}", match a
        {
            Some( v ) => v,
            _ => "".to_string()
        } 
    )
}

#[get( url = "/get_echo_param_option_number" )]
fn get_echo_param_option_number(
    #[query_param]
    a : Option<i64>
) -> String
{
    format!( "a={}", match a
        {
            Some( v ) => v.to_string(),
            _ => "".to_string()
        } 
    )
}

#[get( url = "/get_echo_param_point_custom_deserializer" )]
fn get_echo_param_point_custom_deserializer(
    #[query_param(deserialize_with = custom_deserialize_point )]
    point : Point
) -> String
{
    format!( "x={}, y={}", match point.x
    {
        Some( v ) => v,
        None => 0.0
    }, point.y )
}

#[get( url = "/get_echo_param_point_custom_deserializer_change_name" )]
fn get_echo_param_point_custom_deserializer_change_name(
    #[query_param(deserialize_with = custom_deserialize_point, name = "point_renamed" )]
    point : Point
) -> String
{
    format!( "x={}, y={}", match point.x
    {
        Some( v ) => v,
        None => 0.0
    }, point.y )
}

pub async fn custom_deserialize_point( io : &IO, s : &str ) -> Result<Point, String>
{
    let coords = s.split( "," )
                                .into_iter()
                                .map( | s | s.parse::<f32>() )
                                .collect::<Result<Vec<_>, _>>()
                                .map_err( | e | e.to_string() )?;

    match coords.len()
    {
        1 => Ok( Point { x : None, y : coords[ 0 ] + io.response.status as f32 } ),
        2 => Ok( Point { x : Some( coords[ 0 ] + io.response.status as f32 ), y : coords[ 1 ] } ),
        _ => Err( "Only one or two coords allowed".to_string() )
    }
}