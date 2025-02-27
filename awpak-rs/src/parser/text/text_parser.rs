use hyper::body::Bytes;
use strategy_pattern_rs::strategy_pattern_fn;
use crate::{Error, ContentTypeStrategy, ResponseContentTypeStrategy};

#[strategy_pattern_fn( key = "text/plain", strategy = ContentTypeStrategy )]
pub fn text_parser( bytes : Bytes ) -> Result<serde_json::Value, Error>
{
    let text = std::str::from_utf8( &bytes );

    if text.is_err()
    {
        return Err( Error::ParserError( "Invalid text/plain".to_string() ) )
    }

    let text = text.unwrap().to_string();

    let val = serde_json::to_value( text );

    if val.is_err()
    {
        return Err( Error::ParserError( "Invalid text/plain".to_string() ) )
    }

    Ok( val.unwrap() )
}

#[strategy_pattern_fn( key = "text/plain", strategy = ResponseContentTypeStrategy )]
pub fn text_serializer( value : serde_json::Value ) -> Result<Bytes, Error>
{
    let salida = get_text_from_value( &value );

    if salida.is_err()
    {
        return Err( salida.err().unwrap() )
    }

    Ok( Bytes::from( salida.unwrap() ) )
}

pub fn get_text_from_value( value : &serde_json::Value ) -> Result<String, Error>
{
    if value == &serde_json::Value::Null
    {
        return Ok( "".to_string() )
    }

    let salida = &value.as_str();

    match salida
    {
        Some( v ) => Ok( v.to_string() ),
        _ => match serde_json::to_string( &value )
        {
            Ok( v ) => Ok( v ),
            _ => Err( Error::ParserError( "Invalid response text/plain".to_string() ) )
        } 
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[test]
    fn test_text_serializer_text_input()
    {
        let value = serde_json::to_value( "text" );

        assert!( value.is_ok(), "value is Err" );

        let value = value.unwrap();

        let text = text_serializer( value );

        assert!( text.is_ok(), "text is Err" );

        let text = std::str::from_utf8( text.as_ref().unwrap() );

        assert!( text.is_ok(), "text 2 is Err" );

        let text = text.unwrap();

        assert_eq!( text, "text" );
    }

    #[test]
    fn test_text_serializer_json_input()
    {
        #[derive(Serialize, Deserialize)]
        struct Point
        {
            x : f32,
            y : f32
        }

        let value = serde_json::from_str::<serde_json::Value>( r#"{"x":3,"y":5}"# );

        assert!( value.is_ok(), "value is Err" );

        let value = value.unwrap();

        let point = serde_json::from_value::<Point>( value.clone() );

        assert!( point.is_ok(), "point is Err" );

        let point = point.unwrap();

        assert_eq!( point.x, 3.0 );
        assert_eq!( point.y, 5.0 );

        let text = text_serializer( value );

        assert!( text.is_ok(), "text is Err" );

        let text = std::str::from_utf8( text.as_ref().unwrap() );

        assert!( text.is_ok(), "text 2 is Err" );

        let text = text.unwrap();

        assert_eq!( text, r#"{"x":3,"y":5}"# );
    }

    #[test]
    fn test_text_parser_json_input()
    {
        let text_value = text_parser( Bytes::from( r#"{"x":3,"y":5}"# ) );

        assert!( text_value.is_ok(), "text_value Err" );

        let text = text_value.as_ref().unwrap().as_str();

        assert!( text.is_some(), "text_value.as_str() is none" );

        assert_eq!( text.unwrap(), r#"{"x":3,"y":5}"# );
    }

    #[test]
    fn test_text_parser_text_input()
    {
        let text_value = text_parser( Bytes::from( "hello" ) );

        assert!( text_value.is_ok(), "text_value Err" );

        let text = text_value.as_ref().unwrap().as_str();

        assert!( text.is_some(), "text_value.as_str() is none" );

        assert_eq!( text.unwrap(), "hello" );
    }
}