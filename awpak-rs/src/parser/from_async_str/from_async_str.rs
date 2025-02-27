use std::str::FromStr;

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
/// impl FromAsyncStr<User> for User {
///     async fn from_async_str(io: &IO, s: &str) -> Result<User, ()> {
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
pub trait FromAsyncStr<T>
{
    fn from_async_str( io : &IO, s : &str ) -> impl std::future::Future<Output = Result<T, ()>> + Send;
}

impl<T: FromStr> FromAsyncStr<T> for T
{
    async fn from_async_str( _io : &IO, s : &str ) -> Result<T, ()>
    {
        match T::from_str( s )
        {
            Ok( v ) => Ok( v ),
            _ => Err( () )
        }
    }
}