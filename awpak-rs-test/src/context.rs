use awpak_rs::{io::io::IO, MiddlewareResponse, middleware, post, context};


struct ContextTest
{
    x : usize
}

#[middleware(
    order=0
)]
fn middleware_set_context( mut io : IO ) -> MiddlewareResponse
{
    io.set_context( ContextTest { x : 77 } );

    MiddlewareResponse::Next( io )
}

#[middleware(
    order=1
)]
fn middleware_set_context_mut( mut io : IO ) -> MiddlewareResponse
{
    let mut ctx = io.get_context_mut::<ContextTest>();

    change_context( ctx.as_mut().unwrap() ).await;

    MiddlewareResponse::Next( io )
}

async fn change_context( ctx : &mut ContextTest )
{
    ctx.x = ctx.x + 1;
}

#[post( url = "/post_echo_context" )]
fn post_echo_context(
    #[context]
    ctx : Option<&ContextTest>
) -> String
{
    match ctx
    {
        Some( v ) => format!( "x:{}", v.x ),
        _ => "None".to_string()
    }
}

#[post( url = "/post_echo_context_mut" )]
fn post_echo_context_mut(
    #[context]
    ctx : Option<&mut ContextTest>
) -> String
{
    match ctx
    {
        Some( v ) => {
            v.x = v.x + 1;
            format!( "x:{}", v.x )
        },
        _ => "None".to_string()
    }
}