use std::{str::FromStr, sync::{Arc, Mutex}};

use serde::{de::{DeserializeSeed, MapAccess}, Deserialize};

use crate::{error::error::Error, io::io::IO};

use super::io_deserialize_seed::IODeserializeSeed;

pub trait DeserializeWithIO
where Self: for<'a> serde::Deserialize<'a>
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        let mut deserializer = get_deserializer( &bytes );

        match IODeserializeSeed::<Self>::new( io ).deserialize( &mut deserializer )
        {
            Ok( r ) => Ok( r ),
            Err( e ) => Err( Error::ParserError( e.to_string() ) )
        }
    }

    fn deserialize_param_with_io( 
        bytes : Arc<Box<[u8]>>, 
        io : Arc<Mutex<Option<IO>>>,
        param : &str
    ) -> Result<Self, Error>
    {
        if bytes.len() == 0
        {
            return Self::deserialize_with_io( bytes, io );
        }

        let map = 
            serde_json::from_slice::<serde_json::Value>( &bytes )
            .map_err( | e | Error::ParserError( e.to_string() ) )?;
        
        let value = match map
        {
            serde_json::Value::Object( o ) =>
            {
                match o.get( param )
                {
                    Some( v ) => match v
                    {
                        serde_json::Value::String( s ) => s.clone(),
                        _ => v.to_string()
                    },
                    _ => "".to_string()
                }
            },
            _ => return Err( Error::ParserError( "Not an object".to_string() ) )
        };

        let bytes : Arc<Box<[u8]>> = Arc::new( value.as_bytes().into() );

        Self::deserialize_with_io( bytes, io )
    }

    fn deserialize_seed_body<'de, A>( _io : Arc<Mutex<Option<IO>>>, _map : &mut A ) -> Result<Self, A::Error>
    where 
        A: MapAccess<'de>
    {
        println!( "Unreachable deserialize_seed_body" );

        unreachable!()
    }

    fn get_fields() -> &'static [&'static str]
    {
        &[]
    }

    fn get_name() -> &'static str
    {
        ""
    }
    
}

fn get_deserializer<'a>( bytes : &'a Arc<Box<[u8]>> ) -> serde_json::Deserializer<serde_json::de::SliceRead<'a>>
{
    serde_json::Deserializer::new( serde_json::de::SliceRead::new( bytes ) )
}

fn deserialize_to_string( bytes : Arc<Box<[u8]>> ) -> Result<String, Error>
{
    let s : &[u8] = &bytes;

    match String::from_utf8( s.into() )
    {
        Ok( v ) => Ok( v ),
        Err( e ) => Err( Error::ParserError( e.to_string() ) )
    }
}

fn deserialize_from_str_remove_quotation_marks<T: FromStr>( bytes : Arc<Box<[u8]>> ) -> Result<T, Error>
{
    let bytes = remove_quotation_marks( bytes );

    match deserialize_to_string( bytes )
    {
        Ok( v ) => match v.parse::<T>()
        {
            Ok( r ) => Ok( r ),
            _ => Err( Error::ParserError( format!( "Fail convert {} to a number", v ) ) )
        },
        _ => Err( Error::ParserError( "Fail convert to a number".to_string() ) )
    }
}

fn remove_quotation_marks( bytes : Arc<Box<[u8]>> ) -> Arc<Box<[u8]>>
{
    if bytes.len() < 3
    {
        return bytes;
    }

    let mut ini = 0;

    while 
        ( bytes[ ini ] == 0 || bytes[ ini ] == 34 || bytes[ ini ] == 32 || bytes[ ini ] == 39 ) 
        &&
        ini < bytes.len()
    {
        ini += 1;
    }

    if ( bytes.len() - ini ) < 2
    {
        return bytes;
    }

    let mut end = bytes.len() - 1;

    while 
        ( bytes[ end ] == 0 || bytes[ end ] == 34 || bytes[ end ] == 32 || bytes[ end ] == 39 ) 
        &&
        end > ini
    {
        end -= 1;
    }

    if ini == 0 && end == ( bytes.len() - 1 )
    {
        return bytes;
    }

    let bytes = bytes[ ini..=end ].iter().clone().map( | v | v.clone() ).collect::<Vec<u8>>();

    Arc::new( bytes.into() )
}

impl DeserializeWithIO for String
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_to_string( bytes )
    }
}

impl DeserializeWithIO for bool
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_from_str_remove_quotation_marks( bytes )
    }
}

impl DeserializeWithIO for f32
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_from_str_remove_quotation_marks( bytes )
    }
}

impl DeserializeWithIO for f64
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_from_str_remove_quotation_marks( bytes )
    }
}

impl DeserializeWithIO for i8
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_from_str_remove_quotation_marks( bytes )
    }
}

impl DeserializeWithIO for i16
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_from_str_remove_quotation_marks( bytes )
    }
}

impl DeserializeWithIO for i32
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_from_str_remove_quotation_marks( bytes )
    }
}

impl DeserializeWithIO for i64
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_from_str_remove_quotation_marks( bytes )
    }
}

impl DeserializeWithIO for u8
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_from_str_remove_quotation_marks( bytes )
    }
}

impl DeserializeWithIO for u16
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_from_str_remove_quotation_marks( bytes )
    }
}

impl DeserializeWithIO for u32
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_from_str_remove_quotation_marks( bytes )
    }
}

impl DeserializeWithIO for u64
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_from_str_remove_quotation_marks( bytes )
    }
}

impl DeserializeWithIO for u128
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_from_str_remove_quotation_marks( bytes )
    }
}

impl DeserializeWithIO for char
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        deserialize_from_str_remove_quotation_marks( bytes )
    }
}

impl DeserializeWithIO for serde_json::Value
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        __io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        Ok( 
            serde_json::from_slice::<Self>( &bytes )
                .map_err( | e | Error::ParserError( e.to_string() ) )? 
        )
    }
}

impl<T> DeserializeWithIO for Vec<T>
where for<'a> T: Deserialize<'a> + DeserializeWithIO
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        let mut bytes_str = 
            String::from_utf8( bytes.to_vec() )
            .map_err( | _ | Error::ParserError( "Not a valid utf-8 string".to_string() ) )?;

        bytes_str = bytes_str.trim().to_string();

        if ! bytes_str.starts_with( "[" )
        {
            bytes_str.insert( 0, '[' );
        }

        if ! bytes_str.ends_with( "]" )
        {
            bytes_str.push( ']' );
        }

        let value = 
            serde_json::from_slice::<serde_json::Value>( bytes_str.as_bytes() )
            .map_err( | e | Error::ParserError( e.to_string() ) )?;

        let vec = match value
        {
            serde_json::Value::Array( a ) => {
                a
            },
            _ => return Err( Error::ParserError( "Not an array".to_string() ) )
        };

        let mut ret : Vec<T> = Vec::with_capacity( vec.len() );

        for value in vec
        {
            let s = match value
            {
                serde_json::Value::String( s ) => s,
                _ => value.to_string()
            };

            let b : Arc<Box<[u8]>> = Arc::new( s.as_bytes().into() );

            match T::deserialize_with_io( b, io.clone() )
            {
                Ok( v ) => ret.push( v ),
                _ => return Err( Error::ParserError( "".to_string() ) )
            }
        }

        Ok( ret )
    }
}

impl<T> DeserializeWithIO for Option<T>
where for<'a> T: Deserialize<'a> + DeserializeWithIO
{
    fn deserialize_with_io( 
        bytes : Arc<Box<[u8]>>, 
        io : Arc<Mutex<Option<IO>>> 
    ) -> Result<Self, Error>
    {
        if bytes.len() == 0
        {
            return Ok( None )
        }

        match T::deserialize_with_io( bytes, io )
        {
            Ok( v ) => Ok( Some( v ) ),
            _ => Ok( None )
        }
    }
}