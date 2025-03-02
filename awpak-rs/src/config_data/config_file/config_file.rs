use serde::Deserialize;

use crate::{error::error::Error, util::file_utils::get_string_from_file};


pub fn parse_config_file<T: for<'a> Deserialize<'a>>( path : &str ) -> Result<T, Error>
{
    match get_string_from_file( path )
    {
        Ok( s ) => parse_config_string( path, s ),
        Err( e ) => Err( e )
    }
}

fn parse_config_string<T: for<'a> Deserialize<'a>>( path : &str, contents : String ) -> Result<T, Error>
{
    // TODO: Aceptar otros formatos como yaml o .properties.

    match serde_json::from_str::<T>( &contents )
    {
        Ok( v ) => Ok( v ),
        _ => Err( Error::ParserError( format!( "Fail serialize config file to json. File: {}.", path ) ) )    
    }
}