use std::sync::{Arc, Mutex};

use serde_json::Value;

use crate::{error::error::Error, io::{deserializer::{deserialize_with_io::DeserializeWithIO, from_path_variable::FromPathVariable}, io::IO}};

pub fn serialize_value<T>( value : T ) -> Option<Value>
where T: serde::Serialize
{
    match serde_json::to_value( value ) {
        Ok( v ) => Some( v ),
        _ => None
    }
}

pub fn parse_query_param_with_io<T>( io : IO, param : &str ) -> ( Option<T>, IO )
where T: for<'a> serde::Deserialize<'a> + DeserializeWithIO
{
    let bytes = Arc::clone( &io.request.uri.query );

    parse_param_with_io( io, param, bytes )
}

pub fn parse_body_param_with_io<T>( io : IO, param : &str ) -> ( Option<T>, IO )
where T: for<'a> serde::Deserialize<'a> + DeserializeWithIO
{
    let bytes = Arc::clone( &io.request.body.data );

    parse_param_with_io( io, param, bytes )
}

fn parse_param_with_io<T>( io : IO, param : &str, bytes : Arc<Box<[u8]>> ) -> ( Option<T>, IO )
where T: for<'a> serde::Deserialize<'a> + DeserializeWithIO
{
    let io = Arc::new( Mutex::new( Some( io ) ) );

    let ret = match T::deserialize_param_with_io( bytes, Arc::clone( &io ), param )
    {
        Ok( v ) => Some( v ),
        _ => None
    };

    let io = io.lock().unwrap().take().unwrap();

    ( ret, io )
}

pub fn parse_query_with_io<T>( io : IO ) -> ( Option<T>, IO )
where T: for<'a> serde::Deserialize<'a> + DeserializeWithIO
{
    let bytes = Arc::clone( &io.request.uri.query );

    parse_data_with_io( io, bytes )
}

pub fn parse_body_with_io<T>( io : IO ) -> ( Option<T>, IO )
where T: for<'a> serde::Deserialize<'a> + DeserializeWithIO
{
    let bytes = Arc::clone( &io.request.body.data );

    parse_data_with_io( io, bytes )
}

fn parse_data_with_io<T>( io : IO, bytes : Arc<Box<[u8]>> ) -> ( Option<T>, IO )
where T: for<'a> serde::Deserialize<'a> + DeserializeWithIO
{
    let io = Arc::new( Mutex::new( Some( io ) ) );

    let ret = parse_bytes_with_io( io.clone(), bytes );

    let io = io.lock().unwrap().take().unwrap();

    ( ret, io )
}

pub fn parse_bytes_with_io<T>( io : Arc<Mutex<Option<IO>>>, bytes : Arc<Box<[u8]>> ) -> Option<T>
where T: for<'a> serde::Deserialize<'a> + DeserializeWithIO
{
    match T::deserialize_with_io( bytes, Arc::clone( &io ) )
    {
        Ok( v ) => Some( v ),
        _ => None
    }
}

pub async fn parse_path_variable<T>( io : &IO, ind : usize ) -> Option<T>
where T: FromPathVariable
{
    match get_str_path_variable( io, ind )
    {
        Some( v ) => match T::from_path_variable( io, &v ).await
        {
            Ok( v ) => Some( v ),
            _ => None
        },
        _ => None   
    }

    // match io.request.uri.path.split( "/" ).enumerate().find(  | v | v.0 == ind ).map( | v | v.1 )
    // {
    //     Some( v ) => match T::from_path_variable( io, v ).await
    //     {
    //         Ok( v ) => Some( v ),
    //         _ => None
    //     },
    //     _ => None
    // }
}

pub fn get_str_path_variable( io : &IO, ind : usize ) -> Option<String>
{
    match io.request.uri.path.split( "/" ).enumerate().find(  | v | v.0 == ind ).map( | v | v.1 )
    {
        Some( v ) => Some( v.to_string() ),
        _ => None
    }
}

pub fn get_param_str_from_bytes( bytes : Arc<Box<[u8]>>, param : &str ) -> Result<String, Error>
{
    if bytes.len() == 0
    {
        return Ok( "".to_string() );
    }

    let map = 
        serde_json::from_slice::<serde_json::Value>( &bytes )
        .map_err( | e | Error::ParserError( e.to_string() ) )?;

    match map
    {
        serde_json::Value::Object( o ) =>
        {
            match o.get( param )
            {
                Some( v ) => match v
                {
                    serde_json::Value::String( s ) => Ok( s.clone() ),
                    _ => Ok( v.to_string() )
                },
                _ => Ok( "".to_string() )
            }
        },
        _ => Err( Error::ParserError( "Not an object".to_string() ) )
    }
}