use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::services::main_service::main_service_fn;
use crate::util::signals_utils::shutdown_signal;

#[derive(Clone, Copy)]
pub struct ServerParams<'a>
{
    pub ip : &'a str,
    pub port : &'a str
}

impl<'a> ServerParams<'a>
{
    pub fn new( ip : &'a str, port : &'a str ) -> Self
    {
        Self
        {
            ip,
            port
        }
    }

    pub fn get_ip_port( &self ) -> String
    {
        format!( "{}:{}", self.ip, self.port )
    }
}

pub async fn server( ip : &'static str, port : &'static str ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let server_params : ServerParams = ServerParams::new( ip, port );

    let addr = server_params.get_ip_port();

    // We create a TcpListener and bind it
    let listener = TcpListener::bind( addr ).await?;

    // specify our HTTP settings (http1, http2, auto all work)
    let http = http1::Builder::new();

    // the graceful watcher
    let graceful = hyper_util::server::graceful::GracefulShutdown::new();
    // when this signal completes, start shutdown
    let mut signal = std::pin::pin!(shutdown_signal());

    // Our server accept loop
    loop {
        tokio::select! {
            Ok((stream, _addr)) = listener.accept() => {

                let io = TokioIo::new(stream);

                let main_service = main_service_fn( server_params );
                
                let conn = http.serve_connection(io, service_fn(main_service));
                // watch this connection
                let fut = graceful.watch(conn);
                tokio::spawn(async move {
                    if let Err(e) = fut.await {
                        eprintln!("Error serving connection: {:?}", e);
                    }
                });
            },

            _ = &mut signal => {
                eprintln!("graceful shutdown signal received");
                // stop the accept loop
                break;
            }
        }
    }

    // Now start the shutdown and wait for them to complete
    // Optional: start a timeout to limit how long to wait.

    tokio::select! {
        _ = graceful.shutdown() => {
            eprintln!("all connections gracefully closed");
        },
        _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
            eprintln!("timed out wait for all connections to close");
        }
    }

    Ok( () )
}