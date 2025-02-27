use awpak_rs::{io::io::IO, MiddlewareResponse};
use awpak_rs::{get, middleware, post, query_params, request_body, FromValue};
use serde::{Deserialize, Serialize};

use crate::Point;

#[derive(Serialize, Deserialize, FromValue)]
struct Point3DZOrder
{
    x : f32,
    y : f32,
    z : f32,
    has_z : bool
}

#[post( url = "/post_add_z_in_middleware" )]
fn post_add_z_in_middleware(
    #[request_body]
    point : Point
) -> Point
{
    point
}

#[post( url = "/post_add_one_to_x_in_middleware" )]
fn post_add_one_to_x_in_middleware(
    #[request_body]
    point : Point
) -> Point
{
    point
}

#[middleware(
    execute_after=true, 
    urls=[
        "/post_add_z_in_middleware"
    ]
)]
fn middleware_add_z( mut io : IO ) -> MiddlewareResponse
{
    match &mut io.response.body {
        Some( b ) => match b.as_object_mut() {
            Some( o ) => {
                o.insert( "z".to_string(), awpak_rs::Value::from( 333 ) );
            },
            _ => {}
        },
        _ => {}
    };

    MiddlewareResponse::Next( io )
}

#[middleware(
    execute_after=true, 
    urls=[
        "/post_add_z_in_middleware",
        "/post_add_one_to_x_in_middleware"
    ]
)]
fn middleware_add_one_to_x( mut io : IO ) -> MiddlewareResponse
{
    match &mut io.response.body {
        Some( b ) => match b.as_object_mut() {
            Some( o ) => {
                let x = o.get_mut( "x" );

                match x {
                    Some( v ) => {
                        let n = v.as_f64();

                        if n.is_some()
                        {
                            let val = n.unwrap() + 1.0;

                            o.insert( "x".to_string(), awpak_rs::Value::from( val ) );
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        },
        _ => {}
    };

    MiddlewareResponse::Next( io )
}

#[middleware(
    execute_after=true, 
    urls=[
        "^/post_add_.+$"
    ]
)]
fn middleware_regex_add_one_to_y( mut io : IO ) -> MiddlewareResponse
{
    match &mut io.response.body {
        Some( b ) => match b.as_object_mut() {
            Some( o ) => {
                let y = o.get_mut( "y" );

                match y {
                    Some( v ) => {
                        let n = v.as_f64();

                        if n.is_some()
                        {
                            let val = n.unwrap() + 1.0;

                            o.insert( "y".to_string(), awpak_rs::Value::from( val ) );
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        },
        _ => {}
    };

    MiddlewareResponse::Next( io )
}

#[get( url = "/add_one_to_y_if_get_or_x_if_post" )]
fn add_one_to_y(
    #[query_params]
    point : Point
) -> Point
{
    point
}

#[post( url = "/add_one_to_y_if_get_or_x_if_post" )]
fn add_one_to_x(
    #[request_body]
    point : Point
) -> Point
{
    point
}

#[middleware(
    execute_after=true, 
    urls=[
        "/add_one_to_y_if_get_or_x_if_post"
    ],
    method="get"
)]
fn middleware_add_one_to_y_get( mut io : IO ) -> MiddlewareResponse
{
    match &mut io.response.body {
        Some( b ) => match b.as_object_mut() {
            Some( o ) => {
                let y = o.get_mut( "y" );

                match y {
                    Some( v ) => {
                        let n = v.as_f64();

                        if n.is_some()
                        {
                            let val = n.unwrap() + 1.0;

                            o.insert( "y".to_string(), awpak_rs::Value::from( val ) );
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        },
        _ => {}
    };

    MiddlewareResponse::Next( io )
}

#[middleware(
    execute_after=true, 
    urls=[
        "/add_one_to_y_if_get_or_x_if_post"
    ],
    method="post"
)]
fn middleware_add_one_to_x_post( mut io : IO ) -> MiddlewareResponse
{
    match &mut io.response.body {
        Some( b ) => match b.as_object_mut() {
            Some( o ) => {
                let x = o.get_mut( "x" );

                match x {
                    Some( v ) => {
                        let n = v.as_f64();

                        if n.is_some()
                        {
                            let val = n.unwrap() + 1.0;

                            o.insert( "x".to_string(), awpak_rs::Value::from( val ) );
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        },
        _ => {}
    };

    MiddlewareResponse::Next( io )
}

#[post( url = "/add_z_test_order" )]
fn add_z_test_order(
    #[request_body]
    point : Point
) -> Point
{
    point
}

#[post( url = "/add_z_test_order_false" )]
fn add_z_test_order_false(
    #[request_body]
    point : Point
) -> Point
{
    point
}

#[middleware(
    execute_after=true, 
    urls=[
        "/add_z_test_order_false"
    ],
    order=2
)]
fn middleware_add_z_test_order_false( mut io : IO ) -> MiddlewareResponse
{
    match &mut io.response.body {
        Some( b ) => match b.as_object_mut() {
            Some( o ) => {
                o.insert( "z".to_string(), awpak_rs::Value::from( 333 ) );
            },
            _ => {}
        },
        _ => {}
    };

    MiddlewareResponse::Next( io )
}

#[middleware(
    execute_after=true, 
    urls=[
        "/add_z_test_order"
    ],
    order=0
)]
fn middleware_add_z_test_order( mut io : IO ) -> MiddlewareResponse
{
    match &mut io.response.body {
        Some( b ) => match b.as_object_mut() {
            Some( o ) => {
                o.insert( "z".to_string(), awpak_rs::Value::from( 333 ) );
            },
            _ => {}
        },
        _ => {}
    };

    MiddlewareResponse::Next( io )
}

#[middleware(
    execute_after=true, 
    urls=[
        "/add_z_test_order",
        "/add_z_test_order_false"
    ],
    order=1
)]
fn middleware_test_z_exists( mut io : IO ) -> MiddlewareResponse
{
    match &mut io.response.body {
        Some( b ) => match b.as_object_mut() {
            Some( o ) => {
                let z = o.get( "z" );

                match z {
                    Some( _v ) => {
                        o.insert( "has_z".to_string(), awpak_rs::Value::from( true ) );
                    },
                    _ => {
                        o.insert( "has_z".to_string(), awpak_rs::Value::from( false ) );
                    }
                };

                
            },
            _ => {}
        },
        _ => {}
    };

    MiddlewareResponse::Next( io )
}

#[post( url = "/add_z_test_pre_order" )]
fn add_z_test_pre_order(
    #[request_body]
    point : Point3DZOrder
) -> Point3DZOrder
{
    point
}

#[post( url = "/add_z_test_pre_order_false" )]
fn add_z_test_pre_order_false(
    #[request_body]
    point : Point3DZOrder
) -> Point3DZOrder
{
    point
}

#[middleware(
    urls=[
        "/add_z_test_pre_order_false"
    ],
    order=2
)]
fn middleware_add_z_test_pre_order_false( mut io : IO ) -> MiddlewareResponse
{
    match &mut io.request.body.value {
        Some( b ) => match b.as_object_mut() {
            Some( o ) => {
                o.insert( "z".to_string(), awpak_rs::Value::from( 333 ) );
            },
            _ => {}
        },
        _ => {}
    };

    MiddlewareResponse::Next( io )
}

#[middleware(
    urls=[
        "/add_z_test_pre_order"
    ],
    order=0
)]
fn middleware_add_z_test_pre_order( mut io : IO ) -> MiddlewareResponse
{
    match &mut io.request.body.value {
        Some( b ) => match b.as_object_mut() {
            Some( o ) => {
                o.insert( "z".to_string(), awpak_rs::Value::from( 333 ) );
            },
            _ => {}
        },
        _ => {}
    };

    MiddlewareResponse::Next( io )
}

#[middleware(
    execute_after=false, 
    urls=[
        "/add_z_test_pre_order",
        "/add_z_test_pre_order_false"
    ],
    order=1
)]
fn middleware_test_pre_z_exists( mut io : IO ) -> MiddlewareResponse
{
    match &mut io.request.body.value {
        Some( b ) => match b.as_object_mut() {
            Some( o ) => {
                let z = o.get( "z" );

                match z {
                    Some( _v ) => {
                        o.insert( "has_z".to_string(), awpak_rs::Value::from( true ) );
                    },
                    _ => {
                        o.insert( "has_z".to_string(), awpak_rs::Value::from( false ) );
                    }
                };

                
            },
            _ => {}
        },
        _ => {}
    };

    MiddlewareResponse::Next( io )
}