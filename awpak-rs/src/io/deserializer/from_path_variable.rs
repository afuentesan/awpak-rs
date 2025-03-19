use crate::io::io::IO;



/// A trait for asynchronously converting a string into a type.
///
/// Implement this trait for types that should be deserialized from URL path variables.
/// This is useful when fetching database records or performing async lookups based
/// on URL parameters.
///
/// # Example
///
/// ```ignore
/// impl FromPathVariable<User> for User {
///     async fn from_path_variable(io: &IO, s: &str) -> Result<User, String> {
///         let user = get_user_from_db(s).await;
///         Ok(user)
///     }
/// }
/// ```
///
/// This allows extracting a `User` object from a path variable, like in:
///
/// ```ignore
/// #[get(url = "/user/{id}")]
/// async fn get_user(#[path_variable] user: User) -> User {
///     user
/// }
/// ```
///
/// A request like `GET /user/42` will trigger an asynchronous database lookup,
/// fetching the corresponding `User` object.
pub trait FromPathVariable: Sized + Send
{
    fn from_path_variable( io : &IO, s : &str ) -> impl std::future::Future<Output = Result<Self, String>> + Send;
}

impl FromPathVariable for String
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<String, String>
    {
        Ok( s.to_string() )
    }
}

impl FromPathVariable for bool
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<bool, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}

impl FromPathVariable for f32
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<f32, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}

impl FromPathVariable for f64
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<f64, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}

impl FromPathVariable for i8
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<i8, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}

impl FromPathVariable for i16
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<i16, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}

impl FromPathVariable for i32
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<i32, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}

impl FromPathVariable for i64
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<i64, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}

impl FromPathVariable for u8
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<u8, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}

impl FromPathVariable for u16
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<u16, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}

impl FromPathVariable for u32
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<u32, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}

impl FromPathVariable for u64
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<u64, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}

impl FromPathVariable for u128
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<u128, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}

impl FromPathVariable for usize
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<usize, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}

impl FromPathVariable for char
{
    async fn from_path_variable( _io : &IO, s : &str ) -> Result<char, String>
    {
        s.parse().map_err( | _ | "".to_string() )
    }
}


impl<T: FromPathVariable> FromPathVariable for Vec<T>
{
    async fn from_path_variable( io : &IO, s : &str ) -> Result<Vec<T>, String>
    {
        let mut ret : Vec<T> = vec![];

        for s in s.split( "," ).into_iter()
        {
            ret.push( T::from_path_variable( io, s ).await? );
        }

        Ok( ret )
    }
}