use std::{fs::read_to_string, path::Path};

use crate::error::error::Error;


pub fn get_string_from_file( path : &str ) -> Result<String, Error>
{
    if ! Path::new( path ).is_file()
    {
        return Err( Error::FileNotFound( format!( "{} is not a file.", path ) ) )
    }

    match read_to_string( path )
    {
        Ok( s ) => Ok( s ),
        _ => Err( Error::FileNotFound( format!( "{} file not found", path ) ) )
    }
}