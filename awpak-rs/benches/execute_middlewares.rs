// extern crate awpak_rs;

// // use awpak_rs::benches::bench_execute_middlewares;

// use awpak_rs::{initialize_middlewares, io::{io::IO, response::response_data::ResponseData}, middleware, MiddlewareResponse};

// use criterion::{
//     black_box, criterion_group, criterion_main, Criterion
// };
// use criterion::async_executor::FuturesExecutor;

// #[middleware(
//     urls=[
//         "^/b.+$"
//     ]
// )]
// pub fn test_middleware( io : IO ) -> MiddlewareResponse
// {
//     MiddlewareResponse::Next( io )
// }

// fn get_io() -> IO
// {
//     let mut io = IO::with_response( ResponseData::default() );

//     io.request.method = "get".to_string();

//     io.request.uri.path = "/bench".to_string();

//     io
// }

// fn execute_middlewares_bench( c : &mut Criterion )
// {
//     initialize_middlewares();
    
//     c.bench_function(
//         "execute_middlewares", 
//         | b | b.to_async(FuturesExecutor).iter(|| {
//             let io = black_box( get_io() );
//             awpak_rs::services::middleware::middleware::pre_middlewares_exec( io ) 
//         })
//     );
// }

// // fn from_elem(c: &mut Criterion) {
    

// //     c.bench_function(
// //         BenchmarkId::new("input_example" ), |b| {
// //         // Insert a call to `to_async` to convert the bencher to async mode.
// //         // The timing loops are the same as with the normal bencher.
// //         b.to_async(FuturesExecutor).iter(|| {
// //             let io = black_box( get_io() );
// //             pre_middlewares_exec( io ) 
// //         });
// //     });
// // }

// criterion_group!( benches, execute_middlewares_bench );
// criterion_main!( benches );