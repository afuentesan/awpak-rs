use std::cmp::Ordering;

#[derive(Clone)]
pub struct Mime
{
    pub media_type : String,
    pub subtype : String,
    pub q : Option<f32>
}

impl Mime
{
    pub fn new( media_type : String, subtype : String, q : Option<f32> ) -> Self
    {
        Self
        {
            media_type,
            subtype,
            q
        }
    }

    pub fn get_mime_type( &self ) -> String
    {
        format!( "{}/{}", self.media_type, self.subtype )
    }

    pub fn accept_all( &self ) -> bool
    {
        self.accept_all_media_types() && self.accept_all_subtypes()
    }

    pub fn accept_all_media_types( &self ) -> bool
    {
        self.media_type == "*"
    }

    pub fn accept_all_subtypes( &self ) -> bool
    {
        self.subtype == "*"
    }

    pub fn from_accept_str( value : &str ) -> Result<Mime, ()>
    {
        let v = value.trim();

        if v == ""
        {
            return Err( () );
        }

        let q = if v.contains( ";" )
        {
            let parts : Vec<&str> = v.split( ";q=" ).collect();

            if parts.len() == 2
            {
                let val = parts[ 1 ].parse::<f32>();

                if val.is_ok()
                {
                    val.unwrap()
                }
                else
                {
                    return Err( () )  
                }
            }
            else
            {
                return Err( () ) 
            }
        }
        else
        {
            1.0
        };

        let v = v.split( ";" ).next().unwrap().trim();

        let parts = v.split( "/" ).collect::<Vec<&str>>();

        if parts.len() != 2
        {
            return Err( () );
        }

        let media_type = parts[ 0 ].trim();
        let subtype = parts[ 1 ].trim();

        if media_type == "" || subtype == ""
        {
            return Err( () );
        }

        Ok(
            Mime::new( media_type.to_string(), subtype.to_string(), Some( q ) )
        )
    }
}

impl Ord for Mime
{
    fn cmp( &self, other: &Self ) -> std::cmp::Ordering
    {
        if self.q.is_none() && other.q.is_none()
        {
            return Ordering::Equal
        }

        if self.q.is_none()
        {
            return Ordering::Greater
        }

        if other.q.is_none()
        {
            return Ordering::Less
        }

        other.q.unwrap().total_cmp( &self.q.unwrap() )
    }
}

impl PartialOrd for Mime
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Mime
{
    fn eq (&self, other: &Self ) -> bool
    {
        if self.q.is_none() && other.q.is_none()
        {
            return true
        }

        if self.q.is_none() || other.q.is_none()
        {
            return false
        }
        
        self.q.unwrap() == other.q.unwrap()
    }
}

impl Eq for Mime { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_accept_str_application_json()
    {
        // let accept = "application/json, text/html;q=0.9, text/plain;q=0.8, */*;q=0.6";

        let accept = "application/json";

        let mime = Mime::from_accept_str( &accept );

        assert!( mime.is_ok() );

        let mime = mime.unwrap();

        assert_eq!( mime.q.unwrap(), 1.0 );

        assert_eq!( mime.media_type, "application" );
        assert_eq!( mime.subtype, "json" );
    }

    #[test]
    fn test_from_accept_str_text_html_q_0_9()
    {
        // let accept = "application/json, text/html;q=0.9, text/plain;q=0.8, */*;q=0.6";

        let accept = "text/html;q=0.9";

        let mime = Mime::from_accept_str( &accept );

        assert!( mime.is_ok() );

        let mime = mime.unwrap();

        assert_eq!( mime.q.unwrap(), 0.9 );

        assert_eq!( mime.media_type, "text" );
        assert_eq!( mime.subtype, "html" );
    }

    #[test]
    fn test_from_accept_str_stars_q_0_6()
    {
        // let accept = "application/json, text/html;q=0.9, text/plain;q=0.8, */*;q=0.6";

        let accept = "*/*;q=0.6";

        let mime = Mime::from_accept_str( &accept );

        assert!( mime.is_ok() );

        let mime = mime.unwrap();

        assert_eq!( mime.q.unwrap(), 0.6 );

        assert_eq!( mime.media_type, "*" );
        assert_eq!( mime.subtype, "*" );
    }

    #[test]
    fn test_from_accept_str_invalid_1()
    {
        let accept = "*/*;rq=0.6";

        let mime = Mime::from_accept_str( &accept );

        assert!( mime.is_err() );
    }

    #[test]
    fn test_from_accept_str_invalid_2()
    {
        let accept = "application/json, text/html;q=0.9, text/plain;q=0.8, */*;q=0.6";

        let mime = Mime::from_accept_str( &accept );

        assert!( mime.is_err() );
    }

    #[test]
    fn test_from_accept_str_invalid_3()
    {
        let accept = "*/*;q=s0.6";

        let mime = Mime::from_accept_str( &accept );

        assert!( mime.is_err() );
    }

    #[test]
    fn test_from_accept_str_invalid_4()
    {
        let accept = "application";

        let mime = Mime::from_accept_str( &accept );

        assert!( mime.is_err() );
    }
}