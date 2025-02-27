use serde_json::Value;

use super::from_value::FromValue;

pub fn from_value_for_option<T>( value : &Value ) -> Option<Option<T>>
where T: FromValue
{
    match value
    {
        Value::Null => Some( None ),
        _ => Some( T::from_value( value ) )  
    }
}