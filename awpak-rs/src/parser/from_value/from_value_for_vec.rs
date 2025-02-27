use serde_json::Value;

use super::from_value::FromValue;

pub fn from_value_for_vec<T>( value : &Value ) -> Option<Vec<T>>
where T: FromValue
{
    match value
    {
        Value::Array( v ) => from_value_for_array( v ),
        Value::String( v ) => from_value_for_string( v ),
        _ => None
    }
}

fn from_value_for_array<T>( value : &Vec<Value> ) -> Option<Vec<T>>
where T: FromValue
{
    let mut ret : Vec<T> = vec![];

    for item in value
    {
        match T::from_value( item )
        {
            Some( v ) => ret.push( v ),
            _ => return None    
        }
    }

    Some( ret )
}

fn from_value_for_string<T>( value : &String ) -> Option<Vec<T>>
where T: FromValue
{
    let value = value.trim();

    let value = if ! value.starts_with( "[" ) || ! value.ends_with( "]" )
    {
        if value.starts_with( "[" ) || value.ends_with( "]" )
        {
            return None
        }
        else
        {
            Some( format!( "[{}]", value ) )    
        }
    }
    else
    {
        Some( value.to_string() )
    }.unwrap();

    match serde_json::from_str::<Value>( &value )
    {
        Ok( v ) => match v
        {
            Value::Array( v ) => from_value_for_array( &v ),
            _ => None    
        },
        _ => None    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_value_for_string()
    {
        let result : Option<Vec<String>> = from_value_for_string( &r#"["hello", "goodbye"]"#.to_string() );

        assert!( result.is_some() );

        assert_eq!( result.unwrap(), vec![ "hello", "goodbye" ] );

        let result : Option<Vec<String>> = from_value_for_string( &r#" [ "hello" , "goodbye" ]  "#.to_string() );

        assert!( result.is_some() );

        assert_eq!( result.unwrap(), vec![ "hello", "goodbye" ] );

        let result : Option<Vec<String>> = from_value_for_string( &r#"["hello", "goodbye""#.to_string() );

        assert!( result.is_none() );

        let result : Option<Vec<String>> = from_value_for_string( &r#""hello", "goodbye"]"#.to_string() );

        assert!( result.is_none() );

        let result : Option<Vec<String>> = from_value_for_string( &r#""hello", "goodbye""#.to_string() );

        assert!( result.is_some() );

        assert_eq!( result.unwrap(), vec![ "hello", "goodbye" ] );

        let result : Option<Vec<i16>> = from_value_for_string( &r#"1, 7"#.to_string() );

        assert!( result.is_some() );

        assert_eq!( result.unwrap(), vec![ 1, 7 ] );

        let result : Option<Vec<i16>> = from_value_for_string( &r#""1", "7""#.to_string() );

        assert!( result.is_some() );

        assert_eq!( result.unwrap(), vec![ 1, 7 ] );

        let result : Option<Vec<f32>> = from_value_for_string( &r#""1.3", "7.44""#.to_string() );

        assert!( result.is_some() );

        assert_eq!( result.unwrap(), vec![ 1.3, 7.44 ] );

        let result : Option<Vec<f32>> = from_value_for_string( &r#"1.3, 7.44"#.to_string() );

        assert!( result.is_some() );

        assert_eq!( result.unwrap(), vec![ 1.3, 7.44 ] );

        let result : Option<Vec<f32>> = from_value_for_string( &r#""1.3", 7.44"#.to_string() );

        assert!( result.is_some() );

        assert_eq!( result.unwrap(), vec![ 1.3, 7.44 ] );

        let result : Option<Vec<f32>> = from_value_for_string( &r#"{"x":33,"y":21}"#.to_string() );

        assert!( result.is_none() );

    }
}