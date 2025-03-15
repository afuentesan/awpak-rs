
use std::sync::{Arc, Mutex};

use awpak_rs::{get, io::{deserializer::deserialize_with_io::DeserializeWithIO, io::IO, request::request_data::RequestData, response::response_data::ResponseData}, query_param, query_params, request_body, DeserializeWithIO};
use serde::{Deserialize, Deserializer, Serialize};

use crate::Point;


#[derive(Deserialize, Serialize, DeserializeWithIO)]
struct AsyncPoint
{
    x : f32,
    #[serde(deserialize_with = "deserialize_y")]
    y : f32
}

fn deserialize_y<'de, D>( deserializer : D ) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let input : &str = Deserialize::deserialize( deserializer )?;

    let ( tx, rx ) = std::sync::mpsc::channel();

    let input = input.to_string();

    let handle = awpak_rs::tokio::runtime::Handle::current();

    std::thread::spawn( move ||
        {
        
            handle.block_on( async move
                {
                    let result = deserialize_y_async( input ).await;

                    match tx.send( result )
                    {
                        _ => {}    
                    }
                }
            );
        }
    );

    let val = match rx.recv()
    {
        Ok( v ) => v,
        Err( e ) => return Err( <D::Error as serde::de::Error>::custom( e ) )
    };

    val.map_err( <D::Error as serde::de::Error>::custom )
}

async fn deserialize_y_async( input : String ) -> Result<f32, String>
{
    match input.parse::<f32>()
    {
        Ok( v ) => Ok( v + 1.0 ),
        Err( e ) => Err( e.to_string() )
    }
}

#[get( url="/get_async_deserializer" )]
pub fn get_async_deserializer(
    #[query_params]
    point : AsyncPoint
) -> AsyncPoint
{
    point
}

// #[derive(DeserializeWithContext)]
// struct PointWithContexto<'a>
// {
//     x: f32,
//     #[with_context(deserialize_with = deserialize_y2)]
//     y: f32,
//     // _phantom : &'a PhantomData<String>
// }

// struct State {
//     a : f32
// }

// fn deserialize_y2( value : &str, ctx : &State ) -> Result<f32, String>
// {
//     todo!()
// }

#[derive(Serialize, Deserialize, DeserializeWithIO)]
pub struct PointWithContext
{
    x : f32,
    #[io_deserializer(deserialize_y_with_context_sync)]
    y : f32,
    #[serde(default)]
    point : Option<Point>
}

fn deserialize_y_with_context_sync<'a>( input : String, io : Arc<Mutex<Option<awpak_rs::io::io::IO>>> ) -> Result<f32, String>
{
    let ( tx, rx ) = std::sync::mpsc::channel();

    // let input = input.trim().to_string();

    let handle = awpak_rs::tokio::runtime::Handle::current();

    let io_clone = io.clone();

    let _ = std::thread::spawn( move ||
        {
        
            handle.block_on( async move
                {
                    // let io_ref = io_clone.lock().unwrap().as_ref().take().unwrap();

                    let result = deserialize_y_with_context( 
                        input, 
                        io_clone.lock().unwrap().as_ref().take().unwrap() 
                    ).await;

                    match tx.send( result )
                    {
                        _ => {}    
                    }

                    // match tx.send( Ok( 2.0 ) )
                    // {
                    //     _ => {}    
                    // }
                }
            );
        }
    ).join();

    let val = match rx.recv()
    {
        Ok( v ) => v,
        Err( e ) => return Err( e.to_string() )
    };

    val
}

async fn deserialize_y_with_context( input : String, io : &awpak_rs::io::io::IO ) -> Result<f32, String>
{
    match input.trim().parse::<f32>()
    {
        Ok( v ) => Ok( v + io.response.status as f32 ),
        Err( e ) => Err( e.to_string() )
    }
}

// struct PointWithContextDeserializer<'a>
// {
//     io : &'a IO
// }

// impl<'de> DeserializeSeed<'de> for PointWithContextDeserializer<'_>
// {
//     type Value = PointWithContext;
    
//     fn deserialize<D>( self, deserializer : D ) -> Result<Self::Value, D::Error>
//     where
//         D: Deserializer<'de>
//     {
//         struct PointWithContextVisitor<'a>
//         {
//             io: &'a IO,
//         }

//         impl<'de> Visitor<'de> for PointWithContextVisitor<'_>
//         {
//             type Value = PointWithContext;

//             fn expecting( &self, formatter: &mut fmt::Formatter ) -> fmt::Result
//             {
//                 formatter.write_str( "struct PointWithContext" )
//             }

//             fn visit_map<A>( self, mut map: A ) -> Result<Self::Value, A::Error>
//             where
//                 A: MapAccess<'de>,
//             {
//                 let mut x = None;
//                 let mut y = None;
//                 let mut point = None;

//                 while let Some( key ) = map.next_key::<&str>()?
//                 {
//                     println!( "Key: {}", key );

//                     match key
//                     {
//                         "x" => x = Some( map.next_value()? ),
//                         "y" => y = {
//                             let value = map.next_value::<&RawValue>()?;

//                             println!( "Raw value: {}", value.to_string() );

//                             Some( self.io.response.status as f32 )
//                         },
//                         "point" => point = Some( map.next_value()? ),
//                         // _ => return Err( serde::de::Error::unknown_field(key, &[ "x", "y" ] ) )
//                         _ => continue
//                     }
//                 }

//                 Ok( 
//                     PointWithContext
//                     { 
//                         x : x.ok_or_else(|| serde::de::Error::missing_field( "x" ) )?, 
//                         y : y.ok_or_else(|| serde::de::Error::missing_field( "y" ) )?,
//                         point : point.ok_or_else(|| serde::de::Error::missing_field( "point" ) )?,
//                     } 
//                 )
//             }

            
//         }

//         deserializer.deserialize_struct(
//             "PointWithContext",
//             &[ "x", "y", "point" ],
//             PointWithContextVisitor { io : self.io }
//         )
//     }
// }

#[get( url = "/get_point_with_context" )]
pub fn get_point_with_context(
    #[query_param]
    _input : String
) -> PointWithContext
{
    // let point_str = r#"
    // {
    //     "x" : 1,
    //     "y" : 2,
    //     "point" : {
    //         "y" : 22
    //     }
    // }
    // "#;

    // // let point_str = r#"
    // // {
    // //     "x" : 1,
    // //     "y" : 2
    // // }
    // // "#;

    // __io.request.body.value.as_ref().unwrap().to_string();
    
    // let mut deserializer = serde_json::Deserializer::new(
    //     serde_json::de::StrRead::new( &point_str )
    // );

    // let __arc_io = Arc::new( __io );

    // let point = crate::SeedContextDeserializer
    // {
    //     io : __arc_io.clone(),
    // }.deserialize( &mut deserializer );

    // __io = __arc_io.try_into().unwrap();

    // match point {
    //     Ok( p ) => p,
    //     Err( e ) => {
    //         println!( "{:?}", e );
    //         PointWithContext { x : 0.0, y : 0.0, point : None }
    //     } 
    // }

    // let __arc_io = std::sync::Arc::new( std::sync::Mutex::new( Some( __io ) ) );

    // fake( __arc_io.clone() );

    // __io = __arc_io.lock().unwrap().take().unwrap();

    // drop( __arc_io );

    PointWithContext { x : 0.0, y : 0.0, point : None }
}

#[derive(Deserialize, DeserializeWithIO)]
struct Prueba
{
    #[io_deserializer(deserialize_y_with_context_sync)]
    x : f32,
    y : f32
}

#[get( url = "/get_prueba_deserialize_with_io")]
fn get_prueba_deserialize_with_io() -> String
{
    let bytes = r#"{"x":2,"y":3}"#.as_bytes().to_vec();

    let mut io = IO::new( RequestData::default(), ResponseData::default(), None );

    io.request.body.data = Arc::new( bytes.into() );

    let result = Prueba::deserialize_with_io( Arc::clone( &io.request.body.data ), Arc::new( Mutex::new( Some( io ) ) ) );

    match result
    {
        Ok( p ) => format!( "x:{}, y:{}", p.x, p.y ),
        _ => "Err".to_string()
    }
}

#[get( url = "/get_prueba_deserialize_context")]
fn get_prueba_deserialize_context(
    #[request_body]
    prueba : Prueba
) -> String
{
    format!( "x:{}, y:{}", prueba.x, prueba.y )
}

#[get( url = "/get_prueba_deserialize_context_vec")]
fn get_prueba_deserialize_context_vec(
    #[request_body]
    prueba : Vec<Prueba>
) -> String
{
    let mut ret = String::new();

    for p in &prueba
    {
        ret = format!( "{}, x:{}::y:{}", ret, p.x, p.y );
    } 

    format!( "xises: {}", ret )
}

#[get( url = "/get_option_prueba_deserialize_context")]
fn get_option_prueba_deserialize_context(
    #[request_body]
    prueba : Option<Prueba>
) -> String
{
    match prueba
    {
        Some( p ) => format!( "x:{}, y:{}", p.x, p.y ),
        _ => "None".to_string()    
    }
}

#[derive(Deserialize, DeserializeWithIO, Serialize)]
struct PruebaWrapper
{
    #[io_deserializer(deserialize_y_with_context_sync)]
    x : f32,
    y : f32,
    inner : PruebaInner
}

#[derive(Deserialize, DeserializeWithIO, Serialize)]
struct PruebaInner
{
    x : u32,
    #[io_deserializer(deserialize_y_with_context_sync)]
    y : f32
}

#[get( url = "/get_pruebas_async_deserializer")]
fn get_pruebas_async_deserializer(
    #[request_body]
    prueba : PruebaWrapper
) -> PruebaWrapper
{
    prueba
}
