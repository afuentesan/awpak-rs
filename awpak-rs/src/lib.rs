use hyper::body::Bytes;
use strategy_pattern_rs::strategy_pattern_type;
pub use tokio;
use hyper;
pub use inventory;

mod util;
mod services;
pub mod server;
pub mod io;
pub mod endpoint;
pub mod body;
mod parser;

pub use awpak_rs_macros::*;

pub use services::error::Error;
pub use services::middleware::middleware::MiddlewareResponse;
pub use services::middleware::middleware::MiddlewareResponseType;
pub use services::middleware::middleware::initialize_middlewares;
pub use services::middleware::middleware::Middleware;
pub use services::middleware::middleware::MiddlewareExecOrder;

pub use parser::parser::parse_value;
pub use parser::parser::parse_from_value;
pub use parser::parser::parse_body_param_value;
pub use parser::parser::serialize_value;
pub use parser::parser::parse_path_variable;
pub use parser::parser::parse_query_param_value;
pub use parser::from_value::from_value;
pub use parser::from_async_str::from_async_str;
pub use serde_json::Value;

#[strategy_pattern_type( search = "IgnoreCase" )]
pub struct ContentTypeStrategy(fn( Bytes ) -> Result<serde_json::Value, Error>);

#[strategy_pattern_type( search = "IgnoreCase" )]
pub struct ResponseContentTypeStrategy(fn( serde_json::Value ) -> Result<Bytes, Error>);


// pub mod benches
// {
//     use crate::{io::io::IO, services::middleware::middleware::pre_middlewares_exec, MiddlewareResponse};

//     pub async fn bench_execute_middlewares( io : IO ) -> MiddlewareResponse
//     {
//         pre_middlewares_exec( io ).await
//     }
// }
