use awpak_rs::io::cookies::cookies::Cookies;

use awpak_rs::{post, request_cookies, request_body, response_cookies};

#[post( url = "/post_echo_request_cookies" )]
fn post_echo_request_cookies(
    #[request_cookies]
    cookies : Cookies,
    #[response_cookies]
    mut res_cookies : Cookies,
    #[request_body]
    names : Vec<String>
) -> String
{
    if names.len() <= 0
    {
        return "None".to_string()
    }

    for name in names
    {
        let list_cookies = cookies.find_all_by_name( &name );

        for c in list_cookies
        {
            let _ = res_cookies.add_cookie( &c.to_string() );
        }
    }

    "Some".to_string()
}