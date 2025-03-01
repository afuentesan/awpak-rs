
use std::collections::BTreeSet;

use http_body_util::BodyExt as _;
use hyper::header::{HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE};

use crate::{io::{cookies::cookies::Cookies, headers::{header_data::HeaderData, headers::Headers, mime::Mime}, request::{body::BodyData, request_data::{RequestData, Uri}}}, ContentTypeStrategy};

use super::multipart::get_body_from_multipart;


pub async fn get_request_data( request : hyper::Request<hyper::body::Incoming> ) -> Result<RequestData, hyper::Error>
{
    let boundary = request
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .and_then(|ct| multer::parse_boundary(ct).ok());

    let ( parts, body ) = request.into_parts();
    
    let headers = get_headers( &parts );

    let cookies = get_cookies( &parts );

    let uri = get_uri( &parts );

    let body = get_body( body, &headers, boundary ).await?;

    Ok( RequestData::new( uri, parts.method.to_string(), headers, cookies, body ) )
}

fn get_cookies( parts : &hyper::http::request::Parts ) -> Cookies
{
    let mut cookies = Cookies::new();

    for value in parts.headers.get_all( hyper::header::COOKIE )
    {
        match value.to_str() {
            Ok(s) => {
                let _ = cookies.add_cookie( s );
            },
            Err(_) => continue,
        };
    }

    cookies
}

async fn get_body( body : hyper::body::Incoming, headers : &Headers, boundary : Option<String> ) -> Result<BodyData, hyper::Error>
{
    if boundary.is_some()
    {
        return match get_body_from_multipart( body, boundary.unwrap() ).await
        {
            Ok( b ) => Ok( b ),
            Err( e ) => 
            {
                eprintln!( "{}", e );

                Ok( BodyData { value: None, files: vec![] } )
            }
        }
    }

    let value = match body.collect().await
    {
        Ok( v ) =>
        {
            let content_type = &get_content_type( headers.get( CONTENT_TYPE.as_str() ) );

            match ContentTypeStrategy::exec( content_type, v.to_bytes() )
            {
                Ok( v ) => match v
                {
                    Ok( v ) => Some( v ),
                    Err( e ) => 
                    {
                        eprintln!( "{:?}", e );

                        None
                    }
                },
                Err( e ) =>
                {
                    eprintln!( "{}", e );

                    None
                }
            }
        },
        Err( e ) =>
        {
            eprintln!( "{}", e );

            None
        }
    };

    Ok( 
        BodyData
        {
            value,
            files : vec![]
        }
    )
}

pub const REQUEST_MIME_TYPES_AVAILABLES : &[&str] = &[ "text/plain", "application/json" ];

pub fn get_content_type( header : Option<&HeaderData> ) -> String
{
    let content_type = match header
    {
        Some( v ) => match &v.value {
            Some( v ) => v,
            _ => REQUEST_MIME_TYPES_AVAILABLES[ 0 ]
        },
        _ => REQUEST_MIME_TYPES_AVAILABLES[ 0 ]
    };

    if REQUEST_MIME_TYPES_AVAILABLES.contains( &content_type )
    {
        return content_type.to_string()
    }

    REQUEST_MIME_TYPES_AVAILABLES[ 0 ].to_string()
}

fn get_uri( parts : &hyper::http::request::Parts ) -> Uri
{
    Uri::new(
        match parts.uri.host() {
            Some( v ) => Some( v.to_string() ),
            _ => None
        }, 
        parts.uri.path().to_string(), 
        match parts.uri.query() {
            Some( v ) => Some( v.to_string() ),
            _ => None
        }, 
        parts.uri.port_u16(), 
        match parts.uri.scheme_str() {
            Some( v ) => Some( v.to_string() ),
            _ => None
        }
    )
}

fn get_headers( parts : &hyper::http::request::Parts ) -> Headers
{
    let headers = &parts.headers;

    let mut ret = Headers::new();

    for header in headers
    {
        if header.0 == ACCEPT
        {
            ret.set_accept( parse_accept_header( header.1 ) );
        }

        let _ = ret.add_header_data( get_header_from( header ) );
    }

    ret
}

fn get_header_from( data : ( &HeaderName, &HeaderValue ) ) -> HeaderData
{
    let name = data.0.to_string();
    let value_bytes = data.1.as_bytes().to_vec();

    let value = match data.1.to_str() {
        Ok( str ) => Some( str.to_string() ),
        Err( _ ) => None
    };

    HeaderData::new( name, value_bytes, value )
}

fn parse_accept_header( value : &HeaderValue ) -> BTreeSet<Mime>
{
    let mut mimes : BTreeSet<Mime> = BTreeSet::new();

    let value = value.to_str();

    if value.is_err()
    {
        match value.err()
        {
            Some( v ) => eprintln!( "{}", v ),
            _ => {}
        };
        
        return mimes
    }

    value.unwrap().split( "," ).for_each( | v | {

        let mime = Mime::from_accept_str( v );

        if mime.is_ok()
        {
            mimes.insert( mime.unwrap() );
        }

    } );

    mimes
}