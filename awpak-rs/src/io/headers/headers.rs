use std::collections::{BTreeSet, HashMap};

use regex::Regex;

use super::{header_data::HeaderData, mime::Mime};

/// Represents the HTTP headers of a request or response.
///
/// This struct stores headers as a map, where the key is the header name, and the value is a vector of `HeaderData`.
/// Since some headers can have multiple values, each header name maps to a list of `HeaderData` entries.
///
/// # Example
///
/// Given the headers:
/// ```text
/// Content-Type: application/json
/// Set-Cookie: session_id=abc123
/// Set-Cookie: theme=dark
/// ```
///
/// The `Headers` struct will contain:
/// ```text
/// headers = {
///     "Content-Type" => [HeaderData { name: "Content-Type", value: Some("application/json"), value_bytes: [...] }],
///     "Set-Cookie" => [
///         HeaderData { name: "Set-Cookie", value: Some("session_id=abc123"), value_bytes: [...] },
///         HeaderData { name: "Set-Cookie", value: Some("theme=dark"), value_bytes: [...] }
///     ]
/// }
/// ```
///
/// # Fields
///
/// - `headers` – A map where keys are header names and values are lists of `HeaderData` entries.
///
/// # Notes
///
/// - Some headers (like `Set-Cookie`) can have multiple values, which is why the map stores a vector of `HeaderData`.
#[derive(Clone)]
pub struct Headers
{
    headers : HashMap<String, Vec<HeaderData>>,
    accept : BTreeSet<Mime>
}

impl Headers
{
    pub fn new() -> Self
    {
        Self
        {
            headers : HashMap::new(),
            accept : BTreeSet::new()
        }
    }

    pub fn set_accept( &mut self, mimes : BTreeSet<Mime> )
    {
        self.accept = mimes
    }

    pub fn content_negotiation( &self, availables : &[&str] ) -> Option<String>
    {
        if self.accept.len() == 0 || availables.len() == 0
        {
            return None
        }

        for mime in &self.accept
        {
            let mime_type = mime.get_mime_type();

            if mime.accept_all()
            {
                return Some( availables[ 0 ].to_string() )
            }

            if availables.contains( &mime_type.as_str() )
            {
                return Some( mime_type )
            }

            if mime.accept_all_media_types()
            {
                let re = Regex::new( format!( r"^.+\/{}", mime.subtype ).as_str() ).unwrap();

                for a in availables
                {
                    if re.is_match( a )
                    {
                        return Some( a.to_string() )
                    }
                }
            }

            if mime.accept_all_subtypes()
            {
                let re = Regex::new( format!( r"^{}\/.+", mime.media_type ).as_str() ).unwrap();

                for a in availables
                {
                    if re.is_match( a )
                    {
                        return Some( a.to_string() )
                    }
                }
            }
        }

        None
    }

    /// Retrieves the first `HeaderData` entry for a given header name.
    ///
    /// If the header exists, returns a reference to the first `HeaderData` entry.
    /// If the header does not exist, returns `None`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(header) = headers.get("Content-Type") {
    ///     println!("Content-Type: {:?}", header.value);
    /// }
    /// ```
    pub fn get( &self, key : &str ) -> Option<&HeaderData>
    {
        match self.headers.get( key )  {
            Some( v ) => Some( &v[ 0 ] ),
            _ => None
        }
    }

    /// Retrieves a mutable reference to the first `HeaderData` entry for a given header name.
    ///
    /// If the header exists, returns a mutable reference to the first `HeaderData` entry.
    /// If the header does not exist, returns `None`.
    pub fn get_mut( &mut self, key : &str ) -> Option<&mut HeaderData>
    {
        match self.headers.get_mut( key )  {
            Some( v ) => Some( &mut v[ 0 ] ),
            _ => None
        }
    }

    /// Checks if a given header exists in the collection.
    ///
    /// Returns `true` if the header exists, otherwise returns `false`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// if headers.has("Authorization") {
    ///     println!("Authorization header is present.");
    /// }
    /// ```
    pub fn has( &self, key : &str ) -> bool
    {
        self.get( key ).is_some()
    }

    /// Retrieves the value of a header as a `String`, if available.
    ///
    /// If the header exists and has a text value, returns a reference to the `String` value.
    /// If the header does not exist or has a binary value, returns `None`.
    pub fn get_value( &self, key : &str ) -> Option<&String>
    {
        match self.get( key ) {
            Some( v ) => Some( v.value.as_ref().unwrap() ),
            None => None
        }
    }

    /// Retrieves the raw binary value of a header.
    ///
    /// If the header exists, returns a reference to the binary value as `Vec<u8>`.
    /// If the header does not exist, returns `None`.
    pub fn get_value_bytes( &self, key : &str ) -> Option<&Vec<u8>>
    {
        match self.get( key ) {
            Some( v ) => Some( &v.value_bytes ),
            None => None
        }
    }

    /// Adds a new header entry to the collection.
    ///
    /// This function creates a new `HeaderData` entry and appends it to the existing headers.
    ///
    /// # Example
    ///
    /// ```ignore
    /// headers.add_header("X-Custom-Header".to_string(), "value".to_string());
    pub fn add_header( &mut self, name : String, value : String )
    {
        let header_data = HeaderData::new( name, value.clone().into_bytes(), Some( value ) );

        self.add_header_data( header_data )
    }

    /// Replaces the existing header with a new value.
    ///
    /// If the header exists, all previous values are removed and replaced with the new one.
    /// If the header does not exist, it is added.
    pub fn replace_header( &mut self, name : String, value : String )
    {
        let header_data = HeaderData::new( name, value.clone().into_bytes(), Some( value ) );

        self.replace_header_data( header_data )
    }

    /// Replaces the existing header value only if the header already exists.
    ///
    /// Returns `Ok(())` if the header was replaced, or `Err(())` if the header was not found.
    pub fn replace_header_if_exists( &mut self, name : String, value : String ) -> Result<(), ()>
    {
        let header_data = HeaderData::new( name, value.clone().into_bytes(), Some( value ) );

        self.replace_header_data_if_exists( header_data )
    }

    /// Adds a new `HeaderData` entry to the collection.
    ///
    /// If the header does not exist, it is created.
    /// If it already exists, the new value is appended to the existing list.
    pub fn add_header_data( &mut self, data : HeaderData )
    {
        if ! self.headers.contains_key( &data.name )
        {
            self.headers.insert( data.name.clone(), vec![ data ] );

            return;
        }

        self.headers.get_mut( &data.name ).unwrap().push( data );
    }

    /// Replaces all existing values for a given header with a new `HeaderData` entry.
    ///
    /// If the header does not exist, it is created.
    pub fn replace_header_data( &mut self, data : HeaderData )
    {
        self.headers.insert( data.name.clone(), vec![ data ] );
    }

    /// Replaces all existing values for a given header, but only if the header already exists.
    ///
    /// Returns `Ok(())` if the header was replaced, or `Err(())` if the header was not found.
    pub fn replace_header_data_if_exists( &mut self, data : HeaderData ) -> Result<(), ()>
    {
        if ! self.headers.contains_key( &data.name )
        {
            return Err( () )
        }

        self.headers.insert( data.name.clone(), vec![ data ] );

        Ok( () )
    }

    /// Replaces all headers with a new set, except for `Accept`, which is kept unchanged.
    ///
    /// This method replaces the headers with a new `Headers` instance, except for the `"Accept"` header, which remains unchanged.
    pub fn replace_headers( &mut self, headers : Headers )
    {
        for header in headers.headers
        {
            if header.0.to_lowercase() == "accept"
            {
                continue;
            }
            
            self.headers.insert( header.0, header.1 );
        }
    }

    pub fn iter(&self) -> HeadersIterator
    {
        HeadersIterator
        {
            iter : self.headers.iter()
        }
    }

    pub fn iter_all(&self) -> HeadersIteratorAll
    {
        HeadersIteratorAll
        {
            iter : self.headers.iter()
        }
    }
}

pub struct HeadersIterator<'a>
{
    iter : std::collections::hash_map::Iter<'a, String, Vec<HeaderData>>
}

impl<'a> Iterator for HeadersIterator<'a>
{
    type Item = &'a HeaderData;

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

pub struct HeadersIteratorAll<'a>
{
    iter : std::collections::hash_map::Iter<'a, String, Vec<HeaderData>>
}

impl<'a> Iterator for HeadersIteratorAll<'a>
{
    type Item = &'a Vec<HeaderData>;

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



#[cfg(test)]
mod tests {
    use super::*;

    const AVAILABLES : &[&str] = &["text/html", "text/plain", "application/json"];

    #[test]
    fn test_content_negotiation()
    {
        let mut headers = Headers::new();

        headers.accept.insert( Mime::new( "*".to_string(), "*".to_string(), Some( 0.6 ) ) );
        headers.accept.insert( Mime::new( "*".to_string(), "json".to_string(), Some( 0.9 ) ) );
        headers.accept.insert( Mime::new( "application".to_string(), "json".to_string(), Some( 1.0 ) ) );
        headers.accept.insert( Mime::new( "application".to_string(), "*".to_string(), Some( 0.8 ) ) );

        let media_type = headers.content_negotiation( AVAILABLES );

        assert!( media_type.is_some() );

        let media_type = media_type.unwrap();

        assert_eq!( media_type, "application/json" );
    }

    #[test]
    fn test_content_negotiation_accept_all()
    {
        let mut headers = Headers::new();

        headers.accept.insert( Mime::new( "*".to_string(), "*".to_string(), Some( 1.2 ) ) );
        headers.accept.insert( Mime::new( "*".to_string(), "json".to_string(), Some( 0.9 ) ) );
        headers.accept.insert( Mime::new( "application".to_string(), "json".to_string(), Some( 1.0 ) ) );
        headers.accept.insert( Mime::new( "application".to_string(), "*".to_string(), Some( 0.8 ) ) );

        let media_type = headers.content_negotiation( AVAILABLES );

        assert!( media_type.is_some() );

        let media_type = media_type.unwrap();

        assert_eq!( media_type, "text/html" );
    }

    #[test]
    fn test_content_negotiation_accept_all_media_types()
    {
        let mut headers = Headers::new();

        headers.accept.insert( Mime::new( "*".to_string(), "*".to_string(), Some( 0.6 ) ) );
        headers.accept.insert( Mime::new( "*".to_string(), "plain".to_string(), Some( 1.9 ) ) );
        headers.accept.insert( Mime::new( "application".to_string(), "json".to_string(), Some( 1.0 ) ) );
        headers.accept.insert( Mime::new( "application".to_string(), "*".to_string(), Some( 0.8 ) ) );

        let media_type = headers.content_negotiation( AVAILABLES );

        assert!( media_type.is_some() );

        let media_type = media_type.unwrap();

        assert_eq!( media_type, "text/plain" );
    }

    #[test]
    fn test_content_negotiation_accept_all_subtypes()
    {
        let mut headers = Headers::new();

        headers.accept.insert( Mime::new( "*".to_string(), "*".to_string(), Some( 0.6 ) ) );
        headers.accept.insert( Mime::new( "*".to_string(), "json".to_string(), Some( 0.9 ) ) );
        headers.accept.insert( Mime::new( "application".to_string(), "json".to_string(), Some( 1.0 ) ) );
        headers.accept.insert( Mime::new( "text".to_string(), "*".to_string(), Some( 1.2 ) ) );

        let media_type = headers.content_negotiation( AVAILABLES );

        assert!( media_type.is_some() );

        let media_type = media_type.unwrap();

        assert_eq!( media_type, "text/html" );
    }
}