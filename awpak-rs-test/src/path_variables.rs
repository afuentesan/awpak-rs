
use awpak_rs::from_async_str::FromAsyncStr;
use awpak_rs::io::io::IO;
use awpak_rs::get;
use awpak_rs::path_variable;
use serde::Serialize;

#[derive(Serialize)]
struct ObjectPathVariable
{
    x : usize
}

impl FromAsyncStr<ObjectPathVariable> for ObjectPathVariable
{
    async fn from_async_str( _io : &IO, s : &str ) -> Result<ObjectPathVariable, ()>
    {
        match s.parse::<usize>()
        {
            Ok( x ) => Ok( ObjectPathVariable { x } ),
            _ => Err( () )
        }
    }
}

#[get( url = "/get_echo/path_variable/string/{variable}" )]
fn get_echo_path_variable_string(
    #[path_variable]
    variable : String
) -> String
{
    variable
}

#[get( url = "/get_echo/{variable_2}/string_usize/{variable_1}" )]
fn get_echo_path_variable_string_usize(
    #[path_variable]
    variable_1 : String,
    #[path_variable]
    variable_2 : usize
) -> String
{
    format!( "{} {}", variable_1, variable_2 )
}

#[get( url = "/get_echo/path_variable/object/{variable}" )]
fn get_echo_path_variable_object(
    #[path_variable]
    variable : ObjectPathVariable
) -> ObjectPathVariable
{
    variable
}