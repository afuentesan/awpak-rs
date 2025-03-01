use crate::error::error::Error;


pub fn normalize_url( url : &str ) -> String
{
    let mut url = url.to_lowercase();

    if ! url.starts_with( "/" )
    {
        url.insert_str( 0, "/" );
    }

    if url.len() > 1 && url.ends_with( "/" )
    {
        url.pop();
    }

    return url;
}

pub fn get_regex( url : &str ) -> Result<Option<regex::Regex>, Error>
{
    match get_regex_str( url )
    {
        Some( r ) => match regex::Regex::new( &r )
        {
            Ok( r ) => Ok( Some( r ) ),
            Err( e ) => Err( Error::RegexError( e.to_string() ) )
        },
        _ => Ok( None )
    }
}

fn get_regex_str( url : &str ) -> Option<String>
{
    if url.len() < 3
    {
        return None;
    }

    let mut str_ret = String::from( "" );

    let parts = url.split( "/" ).collect::<Vec<&str>>();

    let re = regex::Regex::new( r"^\{.+\}$" ).unwrap();

    let mut is_regex = false;

    for part in parts
    {
        if part.trim() == ""
        {
            continue;
        }

        if re.is_match( part.trim() )
        {
            is_regex = true;

            str_ret = format!( r"{}\/[^\/]+", str_ret );
        }
        else
        {
            str_ret = format!( r"{}\/{}", str_ret, part );    
        }
    }

    if ! is_regex
    {
        return None
    }

    Some( format!( r"^{}$", str_ret ) )
}