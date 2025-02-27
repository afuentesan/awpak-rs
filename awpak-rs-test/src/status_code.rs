use awpak_rs::{post, set_status_code};

use awpak_rs::body_param;

#[post( url = "/post_echo_status_code" )]
fn post_echo_status_code(
    #[body_param]
    status : u16
) -> String
{
    // __io.response.status = status;

    set_status_code!( status );

    "".to_string()
}