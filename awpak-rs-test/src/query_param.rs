use awpak_rs::{get, query_param};

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