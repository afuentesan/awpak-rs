use std::str::FromStr;

use serde_json::Value;

use crate::parser::text::text_parser::get_text_from_value;

pub fn from_value_for_from_str<T>( value : &Value ) -> Option<T>
where T: FromStr
{
    match get_text_from_value( value ) {
        Ok( v ) => match v.parse() {
            Ok( v ) => Some( v ),
            _ => None
        },
        _ => None
    }
}