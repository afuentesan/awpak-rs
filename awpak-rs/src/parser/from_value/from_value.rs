use serde_json::Value;

use super::{from_value_for_from_str::from_value_for_from_str, from_value_for_option::from_value_for_option, from_value_for_string::from_value_for_string, from_value_for_vec::from_value_for_vec};

/// Trait for converting `serde_json::Value` into Rust types in `awpak-rs`.
///
/// The `FromValue` trait enables the conversion of `serde_json::Value` into a concrete Rust type.
/// This trait is automatically implemented when using the `#[derive(FromValue)]` macro.
///
/// Any type that implements `serde::Deserialize` can implement `FromValue` to allow seamless extraction
/// of JSON values from request bodies, query parameters, and other sources.
///
/// # Example
/// ```rust
/// use serde::Deserialize;
/// use serde_json::Value;
/// use awpak_rs::FromValue;
/// use awpak_rs::from_value::FromValue;
///
/// #[derive(Deserialize, FromValue)]
/// struct User {
///     name: String,
///     age: u8,
/// }
/// 
/// let json_value = serde_json::json!({ "name": "Alice", "age": 30 });
/// let user: Option<User> = User::from_value(&json_value);
/// assert!(user.is_some());
/// ```
pub trait FromValue
where Self: for<'a> serde::Deserialize<'a>
{
    /// Converts a `serde_json::Value` into the implementing type.
    ///
    /// Returns `Some(Self)` if the conversion is successful, otherwise returns `None`.
    fn from_value( value : &Value ) -> Option<Self> where Self: Sized;
}

impl FromValue for String
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_string( value )
    }
}

impl FromValue for bool
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_from_str( value )
    }
}

impl FromValue for f32
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_from_str( value )
    }
}

impl FromValue for f64
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_from_str( value )
    }
}

impl FromValue for i8
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_from_str( value )
    }
}

impl FromValue for i16
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_from_str( value )
    }
}

impl FromValue for i32
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_from_str( value )
    }
}

impl FromValue for i64
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_from_str( value )
    }
}

impl FromValue for u8
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_from_str( value )
    }
}

impl FromValue for u16
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_from_str( value )
    }
}

impl FromValue for u32
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_from_str( value )
    }
}

impl FromValue for u64
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_from_str( value )
    }
}

impl FromValue for u128
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_from_str( value )
    }
}

impl FromValue for char
{
    fn from_value( value : &Value ) -> Option<Self>
    {
        from_value_for_from_str( value )
    }
}

impl<T> FromValue for Vec<T>
where T: FromValue
{
    fn from_value( value : &Value ) -> Option<Self> where Self: Sized 
    {
        from_value_for_vec( value )
    }
}

impl<T> FromValue for Option<T>
where T: FromValue
{
    fn from_value( value : &Value ) -> Option<Self> where Self: Sized 
    {
        from_value_for_option( value )
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_from_value_for_string()
    {
        let value : Value = serde_json::from_str( r#"{"x":33,"y":22}"# ).unwrap();

        assert_eq!( String::from_value( &value ).unwrap(), r#"{"x":33,"y":22}"# );

        let value : Value = serde_json::to_value( "hello" ).unwrap();

        assert_eq!( String::from_value( &value ).unwrap(), "hello" );
    }

    #[test]
    fn test_from_value_for_char()
    {
        let value : Value = serde_json::to_value( "a" ).unwrap();

        assert_eq!( char::from_value( &value ).unwrap(), 'a' );

        let value : Value = serde_json::to_value( "asdf" ).unwrap();

        assert_eq!( char::from_value( &value ), None );
    }

    #[test]
    fn test_from_value_for_f32()
    {
        let value : Value = serde_json::to_value( "33" ).unwrap();

        assert_eq!( f32::from_value( &value ).unwrap(), 33.0 );

        let value : Value = serde_json::to_value( "asdf" ).unwrap();

        assert_eq!( f32::from_value( &value ), None );
    }

    #[test]
    fn test_from_value_for_f64()
    {
        let value : Value = serde_json::to_value( "33" ).unwrap();

        assert_eq!( f64::from_value( &value ).unwrap(), 33.0 );

        let value : Value = serde_json::to_value( "asdf" ).unwrap();

        assert_eq!( f32::from_value( &value ), None );
    }
}