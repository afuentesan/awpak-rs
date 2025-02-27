use serde_json::Value;

use crate::parser::text::text_parser::get_text_from_value;

pub fn from_value_for_string( value : &Value ) -> Option<String>
{
    match get_text_from_value( value ) {
        Ok( v ) => Some( v ),
        _ => None
    }
}

