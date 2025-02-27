use std::collections::HashMap;

use serde_json::Value;

use crate::{from_async_str::FromAsyncStr, from_value::FromValue, io::io::IO};

pub fn serialize_value<T>( value : T ) -> Option<Value>
where T: serde::Serialize
{
    match serde_json::to_value( value ) {
        Ok( v ) => Some( v ),
        _ => None
    }
}

pub fn parse_value<T>( io : &IO, from : &str ) -> Option<T>
where T: for<'a> serde::Deserialize<'a> + FromValue
{
    match from
    {
        "request_body" => parse_value_from_request_body( io ),
        "query_params" => parse_value_from_query_params( io ),
        _ => None
    }
}

pub fn parse_body_param_value<T>( io : &IO, name : &str ) -> Option<T>
where T: for<'a> serde::Deserialize<'a> + FromValue
{
    match io.request.body.get_param( name ) {
        Some( v ) => T::from_value( v ),
        _ => None
    }
}

pub fn parse_from_value<T>( value : &Value ) -> Option<T>
where T: for<'a> serde::Deserialize<'a>
{
    match serde_json::from_value( value.clone() )
    {
        Ok( v ) => Some( v ),
        _ => match value {
            serde_json::Value::String( v ) => match serde_json::from_str( v )
            {
                Ok( v ) => Some( v ),
                _ => None
            },
            _ => None
        }
    }
}

fn parse_value_from_request_body<T>( io : &IO ) -> Option<T>
where T: for<'a> serde::Deserialize<'a> + FromValue
{
    match io.request.body.value.as_ref()
    {
        Some( v ) => T::from_value( v ),
        _ => None
    }
}

pub fn parse_query_param_value<T>( io : &IO, name : &str ) -> Option<T>
where T: for<'a> serde::Deserialize<'a> + FromValue
{
    match &io.request.uri.query_map
    {
        Some( v ) => match v.get( name )
        {
            Some( s ) => match &serde_json::from_str( s )
            {
                Ok( v ) => T::from_value( v ),
                _ => match serde_json::to_value( s )
                {
                    Ok( v ) => T::from_value( &v ),
                    _ => None    
                }
            },
            _ => T::from_value( &serde_json::Value::Null )    
        },
        _ => T::from_value( &serde_json::Value::Null )   
    }
}

fn parse_value_from_query_params<T>( io : &IO ) -> Option<T>
where T: for<'a> serde::Deserialize<'a>
{
    let query_params = &io.request.uri.query;

    if query_params.is_none()
    {
        let salida : Result<T, _> = serde_qs::from_str( "" );

        if salida.is_err()
        {
            return None;
        }

        return Some( salida.unwrap() );
    }

    let salida : Result<T, _> = serde_qs::from_str( query_params.as_ref().unwrap() );

    if salida.is_err()
    {
        //TODO: Hacer que el query_map sea un HashMap<String, Value>. Habrá que revisar la función parse_query_param_value
        if io.request.uri.query_map.is_none()
        {
            return None;
        }

        let mut map : HashMap<String, serde_json::Value> = HashMap::new();

        for item in io.request.uri.query_map.as_ref().unwrap()
        {
            let val = match serde_json::from_str::<serde_json::Value>( item.1 )
            {
                Ok( v ) => Ok( v ),
                _ => match serde_json::to_value( item.1 ) {
                    Ok( v ) => Ok( v ),
                    _ => Err( () )
                }
            };

            if val.is_ok()
            {
                map.insert( item.0.clone(), val.unwrap() );
            }
        }

        let value = serde_json::to_value( map );

        if value.is_err()
        {
            return None;
        }

        let salida : Result<T, _> = serde_json::from_value( value.unwrap() );

        if salida.is_err()
        {
            return None;
        }

        return Some( salida.unwrap() )
    }

    Some( salida.unwrap() )
}

pub async fn parse_path_variable<T>( io : &IO, ind : usize ) -> Option<T>
where T: FromAsyncStr<T>
{
    match io.request.uri.path.split( "/" ).enumerate().find(  | v | v.0 == ind ).map( | v | v.1 )
    {
        Some( v ) => match T::from_async_str( io, v ).await
        {
            Ok( v ) => Some( v ),
            _ => None
        },
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use crate::io::response::response_data::ResponseData;

    use super::*;

    #[test]
    fn test_parse_query_param_value()
    {
        let io = IO::with_response( ResponseData::default() );

        let val  = parse_query_param_value::<Option<String>>( &io, "a" );

        assert!( val.is_some() );

        let val = val.unwrap();

        assert!( val.is_none() );
    }
}