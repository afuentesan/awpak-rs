use std::collections::HashMap;

use cookie::Cookie;

/// Represents a collection of HTTP cookies.
///
/// The `Cookies` struct stores cookies as a mapping of cookie names to their values.
/// Since multiple cookies can have the same name, each key in the map corresponds to a `Vec<String>`
/// containing all values associated with that cookie name.
///
/// This struct is used for both HTTP requests and responses.
///
/// # Example
///
/// Suppose we receive the following cookie header:
///
/// ```http
/// Set-Cookie: id=a3fWa; Expires=Wed, 21 Oct 2015 07:28:00 GMT
/// ```
///
/// The `Cookies` struct would store this as:
///
/// ```text
/// let mut cookies = Cookies {
///     cookies: HashMap::new()
/// };
/// cookies.cookies.insert(
///     "id".to_string(),
///     vec!["a3fWa; Expires=Wed, 21 Oct 2015 07:28:00 GMT".to_string()]
/// );
/// ```
///
/// This structure allows handling multiple values for the same cookie name.
#[derive(Clone)]
pub struct Cookies
{
    cookies : HashMap<String, Vec<String>>
}

impl Cookies
{
    /// Creates a new, empty `Cookies` instance.
    ///
    /// # Example
    /// ```
    /// use awpak_rs::io::cookies::cookies::Cookies;
    /// 
    /// let cookies = Cookies::new();
    /// ```
    pub fn new() -> Self
    {
        Self
        {
            cookies : HashMap::new()
        }
    }

    /// Adds a cookie to the collection by parsing a cookie string.
    ///
    /// If the cookie is valid, it is added to the internal storage.
    ///
    /// # Parameters
    /// - `str_cookie`: The cookie string to parse and store.
    ///
    /// # Returns
    /// - `Ok(())` if the cookie was successfully added.
    /// - `Err(())` if the cookie string could not be parsed.
    ///
    /// # Example
    /// ```
    /// use awpak_rs::io::cookies::cookies::Cookies;
    /// 
    /// let mut cookies = Cookies::new();
    /// cookies.add_cookie("session_id=abc123; Path=/; HttpOnly").unwrap();
    /// ```
    pub fn add_cookie( &mut self, str_cookie : &str ) -> Result<(), ()>
    {
        match Cookie::parse( str_cookie )
        {
            Ok( v ) => {
                
                self.add_cookie_obj( v );

                Ok( () )
            },
            _ => Err( () )
        }
    }

    /// Adds a parsed `Cookie` object to the collection.
    ///
    /// This method is used internally by `add_cookie()`.
    fn add_cookie_obj( &mut self, cookie : Cookie )
    {
        let str_cookie = cookie.to_string();

        let name = cookie.name();

        if ! self.cookies.contains_key( name )
        {
            self.cookies.insert( cookie.name().to_string(), vec![ str_cookie ] );

            return
        }

        self.cookies.get_mut( cookie.name() ).unwrap().push( str_cookie );
    }

    /// Replaces an existing cookie or adds it if it does not exist.
    ///
    /// This method ensures that if a cookie with the same name exists, it will be replaced.
    ///
    /// # Parameters
    /// - `str_cookie`: The cookie string to parse and store.
    ///
    /// # Returns
    /// - `Ok(())` if the cookie was successfully replaced.
    /// - `Err(())` if the cookie string could not be parsed.
    ///
    /// # Example
    /// ```
    /// use awpak_rs::io::cookies::cookies::Cookies;
    /// 
    /// let mut cookies = Cookies::new();
    /// cookies.add_cookie("user=JohnDoe").unwrap();
    /// cookies.replace_cookie("user=JaneDoe").unwrap();
    /// ```
    pub fn replace_cookie( &mut self, str_cookie : &str ) -> Result<(), ()>
    {
        match Cookie::parse( str_cookie )
        {
            Ok( v ) => {
                
                self.replace_cookie_obj( v );

                Ok( () )
            },
            _ => Err( () )
        }
    }

    /// Replaces a cookie in the collection with a new `Cookie` object.
    ///
    /// This method is used internally by `replace_cookie()`.
    fn replace_cookie_obj( &mut self, cookie : Cookie )
    {
        let name = cookie.name();
        let str_cookie = cookie.to_string();

        self.cookies.insert( name.to_string(), vec![ str_cookie ] );
    }

    /// Finds the first cookie with the specified name.
    ///
    /// # Parameters
    /// - `name`: The name of the cookie to search for.
    ///
    /// # Returns
    /// - `Some(Cookie)` if a matching cookie is found.
    /// - `None` if no cookie with the given name exists.
    ///
    /// # Example
    /// ```
    /// use awpak_rs::io::cookies::cookies::Cookies;
    /// 
    /// let mut cookies = Cookies::new();
    /// cookies.add_cookie("token=xyz123").unwrap();
    /// if let Some(cookie) = cookies.find_first_by_name("token") {
    ///     println!("Found cookie: {}", cookie);
    /// }
    /// ```
    pub fn find_first_by_name( &self, name : &str ) -> Option<Cookie>
    {
        if self.cookies.contains_key( name )
        {
            return match Cookie::parse( &self.cookies.get( name ).as_ref().unwrap()[ 0 ] )
            {
                Ok( v ) => Some( v ),
                _ => None
            }
        }

        None
    }

    /// Finds all cookies with the specified name.
    ///
    /// # Parameters
    /// - `name`: The name of the cookies to search for.
    ///
    /// # Returns
    /// - A `Vec<Cookie>` containing all matching cookies.
    ///
    /// # Example
    /// ```
    /// use awpak_rs::io::cookies::cookies::Cookies;
    /// 
    /// let mut cookies = Cookies::new();
    /// cookies.add_cookie("id=123").unwrap();
    /// cookies.add_cookie("id=456").unwrap();
    ///
    /// let found_cookies = cookies.find_all_by_name("id");
    /// println!("Found {} cookies.", found_cookies.len());
    /// ```
    pub fn find_all_by_name( &self, name : &str ) -> Vec<Cookie>
    {
        let mut ret : Vec<Cookie> = vec![];

        if self.cookies.contains_key( name )
        {
            for str_cookie in self.cookies.get( name ).unwrap()
            {
                match Cookie::parse( str_cookie )
                {
                    Ok( v ) => ret.push( v ),
                    _ => continue    
                }
            }
        }

        ret
    }

    pub fn iter(&self) -> CookiesIterator
    {
        CookiesIterator
        {
            iter : self.cookies.iter()
        }
    }

    pub fn iter_all(&self) -> CookiesIteratorAll
    {
        CookiesIteratorAll
        {
            iter : self.cookies.iter()
        }
    }

}

pub struct CookiesIterator<'a>
{
    iter : std::collections::hash_map::Iter<'a, String, Vec<String>>
}

impl<'a> Iterator for CookiesIterator<'a>
{
    type Item = &'a String;

    fn next( &mut self ) -> Option<Self::Item>
    {
        let next = self.iter.next();

        if next.is_some()
        {
            let next = next.as_ref().unwrap();

            return Some( &next.1[ 0 ] )
        }

        None
    }
}

pub struct CookiesIteratorAll<'a>
{
    iter : std::collections::hash_map::Iter<'a, String, Vec<String>>
}

impl<'a> Iterator for CookiesIteratorAll<'a>
{
    type Item = &'a Vec<String>;

    fn next( &mut self ) -> Option<Self::Item>
    {
        let next = self.iter.next();

        if next.is_some()
        {
            let next = next.as_ref().unwrap();

            return Some( &next.1 )
        }

        None
    }
}