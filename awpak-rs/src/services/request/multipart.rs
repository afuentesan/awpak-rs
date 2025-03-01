use std::str::FromStr as _;

use futures::StreamExt as _;
use http_body_util::BodyStream;
use hyper::body::{Bytes, Incoming};
use multer::{Field, Multipart};
use serde_json::{Map, Value};

use crate::{io::request::request_body::{RequestBody, FileData}, ContentTypeStrategy};

use super::service_request::REQUEST_MIME_TYPES_AVAILABLES;

// Process the request body as multipart/form-data.
pub async fn get_body_from_multipart( body: Incoming, boundary: String ) -> multer::Result<RequestBody>
{  
    let mut multipart = get_multipart( body, boundary );

    let mut body_data = RequestBody { value : Some( Value::Object( Map::new() ) ), files : vec![] };

    // Iterate over the fields, `next_field` method will return the next field if
    // available.
    while let Some(field) = multipart.next_field().await?
    {
        process_part( field, &mut body_data ).await;
    }

    Ok( body_data )
}

fn get_multipart<'a>( body: Incoming, boundary: String ) -> Multipart<'a>
{
    let body_stream = BodyStream::new(body)
        .filter_map(|result| async move { result.map(|frame| frame.into_data().ok()).transpose() });

    Multipart::new( body_stream, boundary )
}

async fn process_part( field : Field<'_>, body_data : &mut RequestBody )
{
    if is_file( &field )
    {
        process_part_as_file( field, body_data ).await;
    }
    else
    {
        process_part_as_json( field, body_data ).await;
    }
}

async fn process_part_as_file( field : Field<'_>, body_data : &mut RequestBody )
{
    let name = match field.name() {
        Some( v ) => v.to_string(),
        _ => return
    };

    let content_type = field.content_type().unwrap().to_string();

    let filename = match field.file_name() {
        Some( v ) => v.to_string(),
        _ => String::from( "blank" )
    };

    let bytes = field.bytes().await;

    if bytes.is_err()
    {
        return;
    }

    let bytes = bytes.unwrap().to_vec();

    body_data.files.push(
        FileData::new( name, filename, bytes, content_type )
    );
}

async fn process_part_as_json( field : Field<'_>, body_data : &mut RequestBody )
{
    let name = field.name();

    let name = if name.is_some()
    {
        name.unwrap().to_string()
    }
    else
    {
        return;
    };

    let content_type = &get_content_type( &field );

    let text = field.text().await;

    if text.is_ok()
    {
        let text = text.unwrap();

        match Value::from_str( &text ) {
            Ok( v ) => { 
                body_data.value.as_mut().unwrap().as_object_mut().unwrap().insert( name, v );
            },
            _ => {
                let body_bytes = Bytes::from( text );

                let body = ContentTypeStrategy::exec( content_type, body_bytes );

                if body.is_ok()
                {
                    let body = body.unwrap();

                    if body.is_ok()
                    {
                        body_data.value.as_mut().unwrap().as_object_mut().unwrap().insert( name, body.unwrap() );
                    }
                }
            }
        };
    }
}

fn get_content_type( field : &Field<'_> ) -> String
{
    match field.content_type()
    {
        Some( v ) => 
        {
            let content_type = format!( "{}/{}", v.type_().to_string().to_lowercase(), v.subtype().to_string().to_lowercase() );

            if REQUEST_MIME_TYPES_AVAILABLES.contains( &content_type.as_str() )
            {
                content_type.to_string()
            }
            else
            {
                REQUEST_MIME_TYPES_AVAILABLES[ 0 ].to_string()    
            }
        },
        _ => REQUEST_MIME_TYPES_AVAILABLES[ 0 ].to_string()
    }
}

// TODO: Detectar mejor cuando es un fichero
fn is_file( field : &Field ) -> bool
{
    let content_type = field.content_type();

    if content_type.is_some()
    {
        let c = content_type.unwrap().type_();

        if c.as_str() != "text"
        {
            return true;    
        }
    }

    false
}