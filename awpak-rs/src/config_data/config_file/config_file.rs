use std::env;

use serde::Deserialize;
use unicode_segmentation::UnicodeSegmentation as _;

use crate::{config_data::home::home::get_home_path, error::error::Error, util::file_utils::get_string_from_file};

pub fn parse_config_file<T: for<'a> Deserialize<'a>>( path : &str ) -> Result<T, Error>
{
    let path = &format!( "{}{}", get_home_path(), path );

    match get_string_from_file( path )
    {
        Ok( s ) => parse_config_string( path, s ),
        Err( e ) => Err( e )
    }
}

fn parse_config_string<T: for<'a> Deserialize<'a>>( path : &str, contents : String ) -> Result<T, Error>
{
    // TODO: Aceptar otros formatos como yaml o .properties.

    let contents = replace_vars( contents );

    match serde_json::from_str::<T>( &contents )
    {
        Ok( v ) => Ok( v ),
        _ => Err( Error::ParserError( format!( "Fail serialize config file to json. File: {}.", path ) ) )    
    }
}

fn replace_vars( txt : String ) -> String
{
    let chars = txt.graphemes( true ).collect::<Vec<&str>>();
    let mut buffer : String = String::new();

    enum State
    {
        FirstKey,
        InVar,
        Other
    }

    let mut state = State::Other;

    let mut ret : String = String::new();

    let re_first = regex::Regex::new( r#"^[_a-zA-Z]$"# ).unwrap();
    let re_others = regex::Regex::new( r#"^[_a-zA-Z0-9]$"# ).unwrap();

    for c in chars
    {
        match state
        {
            State::Other =>
            {
                if c == "{"
                {
                    if buffer.len() > 0
                    {
                        ret.push_str( "{" );
                        ret.push_str( &buffer );

                        buffer.clear();
                    }
                    
                    state = State::FirstKey;

                    continue;
                }

                ret.push_str( c );
            },
            State::FirstKey =>
            {
                if c == "{"
                {
                    state = State::Other;

                    ret.push_str( c );

                    continue;
                }

                if ! re_first.is_match( c )
                {
                    ret.push_str( "{" );
                    ret.push_str( c );

                    state = State::Other;

                    continue;
                }

                buffer = c.to_string();

                state = State::InVar;
            },
            State::InVar =>
            {
                if c == "}"
                {
                    ret.push_str( &get_env_var( &buffer ) );

                    buffer.clear();

                    state = State::Other;

                    continue;
                }

                if ! re_others.is_match( c )
                {
                    if c != "{"
                    {
                        buffer.push_str( c );

                        state = State::Other;
                    }
                    else
                    {
                        state = State::FirstKey;    
                    }
                    
                    ret.push_str( "{" );

                    ret.push_str( &buffer );

                    buffer.clear();

                    continue;
                }

                buffer.push_str( c );
            } 
        }
    }

    if buffer.len() > 0
    {
        ret.push_str( "{" );
        ret.push_str( &buffer );
    }
    
    ret
}

fn get_env_var( var : &str ) -> String
{
    match env::var( var )
    {
        Ok( s ) => s,
        _ => String::new()
    }
}

pub struct InitConfigFile
{
    pub fnc : fn()
}

inventory::collect!( InitConfigFile );

pub fn load_config_files()
{
    for c in inventory::iter::<InitConfigFile>
    {
        (c.fnc)();
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_replace_vars()
    {
        assert_eq!( replace_vars( "asdf".to_string() ), "asdf".to_string() );
        assert_eq!( replace_vars( "{asdf}".to_string() ), "".to_string() );

        env::set_var( "asdf", "33" );

        assert_eq!( replace_vars( "{asdf}".to_string() ), "33".to_string() );
        assert_eq!( replace_vars( "{{asdf}".to_string() ), "{asdf}".to_string() );
        assert_eq!( replace_vars( "{{{asdf}".to_string() ), "{33".to_string() );
        assert_eq!( replace_vars( "{{{{asdf}".to_string() ), "{{asdf}".to_string() );

        assert_eq!( replace_vars( "{asdf".to_string() ), "{asdf".to_string() );
        assert_eq!( replace_vars( "{asdf}}".to_string() ), "33}".to_string() );
        assert_eq!( replace_vars( "{abcd{asdf}}}".to_string() ), "{abcd33}}".to_string() );
    }
}