
use awpak_rs::io::deserializer::from_path_variable::FromPathVariable;
use awpak_rs::io::io::IO;
use awpak_rs::get;
use serde::Serialize;

#[derive(Serialize)]
struct ObjectPathVariable
{
    x : usize
}

impl FromPathVariable for ObjectPathVariable
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<ObjectPathVariable, String>
    {
        match s.parse::<usize>()
        {
            Ok( x ) => Ok( ObjectPathVariable { x } ),
            _ => Err( "".to_string() )
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

#[get( url = "/get_echo/path_variable/object/custom_deserializer/{variable}" )]
fn get_echo_path_variable_object_custom_deserializer(
    #[path_variable(deserialize_with = custom_deserialize_path_variable)]
    variable : ObjectPathVariable
) -> ObjectPathVariable
{
    variable
}

async fn custom_deserialize_path_variable( io : &IO, s : &str ) -> Result<ObjectPathVariable, String>
{
    match s.parse::<usize>()
    {
        Ok( x ) => Ok( ObjectPathVariable { x : x + io.response.status as usize } ),
        _ => Err( "".to_string() )
    }
}

#[get( url = "/get_echo/path_variable/vec/u32/{nums}" )]
fn get_echo_path_variable_vec_u32_deserializer(
    #[path_variable]
    nums : Vec<u32>
) -> Vec<u32>
{
    nums
}

#[get( url = "/get_echo/path_variable_renamed/vec/u32/{nums_renamed}" )]
fn get_echo_path_variable_renamed_vec_u32_deserializer(
    #[path_variable( name="nums_renamed" )]
    nums : Vec<u32>
) -> Vec<u32>
{
    nums
}

#[get( url = "/get_echo/path_variable/vec/objects/{objs}" )]
fn get_echo_path_variable_vec_objects_deserializer(
    #[path_variable]
    objs : Vec<ObjectPathVariable>
) -> Vec<ObjectPathVariable>
{
    objs
}