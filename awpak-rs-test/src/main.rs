// use awpak_rs_macros::{awpak_main, body_param, get, part_file, part_files, post, query_params, request_body, FromValue};
use awpak_rs::{awpak_main, body_param, get, part_file, part_files, post, query_params, request_body, FromValue};
use serde::{Deserialize, Serialize};

mod middlewares;
mod path_variables;
mod context;
mod headers;
mod cookies;
mod status_code;
mod query_param;
mod redirects;

#[awpak_main( ip = "127.0.0.1", port = "3001" )]
// #[awpak_main]
fn main() {}

#[derive(Serialize, Deserialize, FromValue)]
struct Point3D
{
    x : f32,
    y : f32,
    z : f32
}

#[derive(Serialize, Deserialize, FromValue)]
struct Point
{
    x : Option<f32>,
    y : f32
}

#[get( url = "/" )]
fn get_zero() -> usize
{
    0
}

#[get( url = "/get_echo_point" )]
fn get_echo_point(
    #[query_params]
    point : Point
) -> Point
{
    point
}

#[get( url = "/get_echo_option_point" )]
fn get_echo_option_point(
    #[query_params]
    point : Option<Point>
) -> Option<Point>
{
    point
}

#[post( url = "/post_echo_text" )]
fn post_echo_text(
    #[request_body]
    text : String
) -> String
{
    text
}

#[post( url = "/post_query_string_echo_point" )]
fn post_query_string_echo_point(
    #[query_params]
    point : Point
) -> Point
{
    point
}

#[post( url = "/post_body_echo_point" )]
fn post_body_echo_point(
    #[request_body]
    point : Point
) -> Point
{
    point
}

#[post( url = "/post_body_param_number_echo" )]
fn post_body_param_number_echo(
    #[body_param]
    param : u64
) -> u64
{
    param
}

#[post( url = "/post_body_param_string_echo" )]
fn post_body_param_string_echo(
    #[body_param]
    param : String
) -> String
{
    param
}

#[post( url = "/post_multipart_file_len" )]
fn post_multipart_file_len(
    #[part_file]
    img : awpak_rs::io::request::request_body::FileData
) -> usize
{
    img.bytes.len()
}

#[post( url = "/post_multipart_files_len" )]
fn post_multipart_files_len(
    #[part_files]
    img : Vec<awpak_rs::io::request::request_body::FileData>
) -> usize
{
    img.iter().map( | i | i.bytes.len() ).fold( 0, |a, b| {
        a + b
    } )
}

#[post( url = "/post_multipart_data" )]
fn post_multipart_data(
    #[part_file]
    img : awpak_rs::io::request::request_body::FileData,
    #[body_param]
    param_1 : String,
    #[body_param]
    param_2 : String
) -> String
{
    format!( "{}, {}, {}", img.bytes.len(), param_1, param_2 )
}

#[post( url = "/post_multipart_data_optional_file" )]
fn post_multipart_data_optional_file(
    #[part_file]
    img : Option<awpak_rs::io::request::request_body::FileData>,
    #[body_param]
    param_1 : String,
    #[body_param]
    param_2 : String
) -> String
{
    format!( "{}, {}, {}", match img {
        Some( i ) => i.bytes.len(),
        _ => 0
    }, param_1, param_2 )
}

#[post( url = "/post_multipart_data_two_optional_files" )]
fn post_multipart_data_two_optional_files(
    #[part_file]
    img_1 : Option<awpak_rs::io::request::request_body::FileData>,
    #[part_file]
    img_2 : Option<awpak_rs::io::request::request_body::FileData>,
    #[body_param]
    param_1 : String,
    #[body_param]
    param_2 : String
) -> String
{
    format!( "{}, {}, {}, {}", match img_1 {
        Some( i ) => i.bytes.len(),
        _ => 0
    }, match img_2 {
        Some( i ) => i.bytes.len(),
        _ => 0
    }, param_1, param_2 )
}

#[post( url = "/post_multipart_data_optional_vec_of_files" )]
fn post_multipart_data_optional_vec_of_files(
    #[part_file]
    img : Option<Vec<awpak_rs::io::request::request_body::FileData>>,
    #[body_param]
    param_1 : String,
    #[body_param]
    param_2 : String
) -> String
{
    match img
    {
        Some( v ) => {

            let mut sizes = "".to_string();

            for f in v
            {
                sizes = format!( "{} {},", sizes, f.bytes.len() );
            }

            format!( "{} {}, {}", sizes, param_1, param_2 ).trim().to_string()
        },
        _ => format!( "{}, {}", param_1, param_2 ).trim().to_string()
    }
}

#[post( url = "/post_body_param_vec_i16_echo" )]
fn post_body_param_vec_i16_echo(
    #[body_param]
    param : Vec<i16>
) -> Vec<i16>
{
    param
}

#[post( url = "/post_body_param_vec_string_echo" )]
fn post_body_param_vec_string_echo(
    #[body_param]
    param : Vec<String>
) -> Vec<String>
{
    param
}

#[post( url = "/post_body_param_option_string_echo" )]
fn post_body_param_option_string_echo(
    #[body_param]
    param : Option<String>
) -> Option<String>
{
    param
}

#[post( url = "/post_request_body_vec_u32_echo" )]
fn post_request_body_vec_u32_echo(
    #[request_body]
    param : Vec<u32>
) -> Vec<u32>
{
    param
}

