use awpak_rs::{get, redirect_to};

use awpak_rs::query_param;

#[get( url = "/get_redirect_default" )]
fn get_redirect_default(
    #[query_param]
    url : String
) -> Option<String>
{
    redirect_to!( url );

    None
}

#[get( url = "/get_redirect_with_status" )]
fn get_redirect_with_status(
    #[query_param]
    url : String,
    #[query_param]
    status : u16
) -> Option<String>
{
    redirect_to!( url, status );

    None
}