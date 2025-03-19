use awpak_rs::post;

use crate::{query_param::custom_deserialize_point, Point};


#[post( url = "/post_request_body_custom_deserializer" )]
fn post_request_body_custom_deserializer(
    #[body_param( deserialize_with = custom_deserialize_point )]
    point : Point
) -> Point
{
    point
}

#[post( url = "/post_request_body_custom_deserializer_change_name" )]
fn post_request_body_custom_deserializer_change_name(
    #[body_param( deserialize_with = custom_deserialize_point, name = "point_renamed" )]
    point : Point
) -> Point
{
    point
}