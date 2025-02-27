
#[derive(Debug)]
pub enum Error
{
    EndpointNotFound( String ),
    EndpointExecution( String ),
    ParserError( String ),
    RegexError( String )
}

impl Error
{
    pub fn get_status_code( &self ) -> u16
    {
        match self
        {
            Error::EndpointNotFound( _ ) => 404,
            _ => 500
        }
    }
}