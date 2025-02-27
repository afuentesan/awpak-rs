use crate::io::io::IO;

use super::types::EndpointReturnType;


pub struct Endpoint
{
    pub url : &'static str,
    pub method : &'static str,
    pub fnc : fn( IO ) -> EndpointReturnType
}

impl Endpoint
{
    pub const fn new( url : &'static str, method : &'static str, fnc : fn( IO ) -> EndpointReturnType ) -> Self
    {
        Self
        {
            url,
            method,
            fnc
        }
    }
}

inventory::collect!( Endpoint );