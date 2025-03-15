use hyper::body::Bytes;
use strategy_pattern_rs::strategy_pattern_fn;
use crate::{error::error::Error, ContentTypeStrategy, ResponseContentTypeStrategy};

#[strategy_pattern_fn( key = "application/json", strategy = ContentTypeStrategy )]
pub fn json_parser( bytes : Bytes ) -> Result<serde_json::Value, Error>
{
    match serde_json::from_slice::<serde_json::Value>( &bytes ) {
        Ok( v ) => Ok( v ),
        _ => {
            let s = String::from_utf8( bytes.to_vec() ).map_err( | e | Error::ParserError( e.to_string() ) )?;

            Ok( serde_json::Value::String( s ) )
        }
    }
}

#[strategy_pattern_fn( key = "application/json", strategy = ResponseContentTypeStrategy )]
pub fn json_serializer( value : serde_json::Value ) -> Result<Bytes, Error>
{
    match value
    {
        serde_json::Value::String( v ) => Ok( v.into() ),
        _ => match serde_json::to_string( &value ) {
            Ok( v ) => Ok( v.into() ),
            _ => Err( Error::ParserError( "Invalid response json".to_string() ) )
        }
    }
}