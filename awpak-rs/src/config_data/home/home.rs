use std::env;
use std::path::Path;

use crate::error::error::Error;

pub const HOME_VAR : &'static str = "AWPAK_RS_HOME";

pub fn get_home_path() -> Result<String, Error> 
{
    match get_home_path_from_var()
    {
        Ok( d ) => Ok( normalize_path( d ) ),
        Err( e_var ) =>
        {
            eprintln!( "{:?}", e_var );

            match get_default_home_path()
            {
                Ok( d ) => Ok( normalize_path( d ) ),
                Err( e_default ) =>
                {
                    eprintln!( "{:?}", e_default );

                    Err( e_default )
                }
            }
        }
    }
    
}

fn normalize_path( mut path : String ) -> String
{
    while path.ends_with( std::path::MAIN_SEPARATOR_STR )
    {
        path = path[ 0..path.len() - 1 ].to_string();
    }
    
    format!( "{}{}", path, std::path::MAIN_SEPARATOR_STR )
}

fn get_home_path_from_var() -> Result<String, Error>
{
    match env::var( HOME_VAR )
    {
        Ok( d ) if d.trim() != "" && dir_exists( d.as_ref() ) => Ok( d ),
        _ => Err( Error::HomePathNotFound( format!( "{} not set.", HOME_VAR ) ) )    
    }
}

fn get_default_home_path() -> Result<String, Error> 
{
    match env::current_dir()
    {
        Ok( d ) => {

            let d = d.to_string_lossy().to_string();

            if d.trim() == "" || ! dir_exists( d.as_ref() )
            {
                Err( Error::HomePathNotFound( "Current dir not found.".to_string() ) )
            }
            else
            {
                Ok( d )
            }
        },
        Err( _ ) => Err( Error::HomePathNotFound( "Current dir not found.".to_string() ) )
    }
}

fn dir_exists( path : &str ) -> bool
{
    Path::new( path ).exists() && Path::new( path ).is_dir()
}

#[cfg(test)]
mod tests
{
    use std::fs::File;

    use super::*;

    #[test]
    fn test_dir_exists()
    {
        assert!( dir_exists( "/" ) );
        assert!( ! dir_exists( "/asdf" ) );

        let _ = File::create( "/tmp/foo.txt" );

        assert!( ! dir_exists( "/tmp/foo.txt" ) );
    }

    #[test]
    fn test_get_home_path_from_var()
    {
        env::set_var( HOME_VAR, "" );

        assert!( get_home_path_from_var().is_err() );

        env::set_var( HOME_VAR, "/tmp" );

        assert!( get_home_path_from_var().is_ok() );

        assert_eq!( get_home_path_from_var().unwrap(), "/tmp" );

        env::set_var( HOME_VAR, "/tmp_not_found" );

        assert!( get_home_path_from_var().is_err() );
    }

    #[test]
    fn test_get_default_home_path()
    {
        assert!( get_default_home_path().is_ok() );
    }

    #[test]
    fn test_get_home_path()
    {
        env::set_var( HOME_VAR, "" );
        
        assert!( get_home_path().is_ok() );

        assert!( get_home_path().unwrap().ends_with( "/" ) );

        assert_ne!( get_home_path().unwrap(), "/tmp/" );

        env::set_var( HOME_VAR, "/tmp" );

        assert!( get_home_path().is_ok() );

        assert!( get_home_path().unwrap().ends_with( "/" ) );

        assert_eq!( get_home_path().unwrap(), "/tmp/" );

        env::set_var( HOME_VAR, "/" );

        assert!( get_home_path().is_ok() );

        assert!( get_home_path().unwrap().ends_with( "/" ) );

        assert_eq!( get_home_path().unwrap(), "/" );

        env::set_var( HOME_VAR, "/tmp/////////" );

        assert!( get_home_path().is_ok() );

        assert!( get_home_path().unwrap().ends_with( "/" ) );

        assert_eq!( get_home_path().unwrap(), "/tmp/" );
    }
}