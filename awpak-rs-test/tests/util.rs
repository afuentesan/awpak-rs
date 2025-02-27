use std::str::FromStr;

use cucumber::{World, Parameter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Point
{
    pub x : Option<f32>,
    pub y : f32
}

// #[derive(Debug, Default, Deserialize, Serialize)]
// pub struct Param
// {
//     pub param : Option<String>
// }

#[allow(dead_code)]
#[derive(Debug, Default, World)]
pub struct PointWorld {
    pub point : Point,
    pub received_point : Option<Point>
}

#[allow(dead_code)]
#[derive(Debug, Default, World)]
pub struct ParamWorld {
    pub param : Option<String>,
    pub received_param : Option<String>
}

#[allow(dead_code)]
#[derive(Debug, Default, Parameter)]
#[param(name = "optionf32", regex = r"Some\([0-9]+\.?[0-9]*\)|None")]
pub struct Optionf32
{
    pub number : Option<f32>
}

impl FromStr for Optionf32
{
    type Err = String;

    fn from_str( s : &str ) -> Result<Self, Self::Err>
    {
        if s == "None"
        {
            return Ok( Optionf32 { number : None } )
        }

        let s = s.trim();
        let s = &s[5..];
        let s = &s[..(s.len() - 1)];
        let s = s.trim();

        let number : f32 = f32::from_str( s ).unwrap();

        Ok( Optionf32 { number : Some( number )} )
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Parameter)]
#[param(name = "optionparam", regex = r"Some\([^)]*\)|None")]
pub struct OptionParam
{
    pub param : Option<String>
}

impl FromStr for OptionParam
{
    type Err = String;

    fn from_str( s : &str ) -> Result<Self, Self::Err>
    {
        if s == ""
        {
            return Ok( OptionParam { param : Some( "".to_string() ) } )
        }

        if s == "None"
        {
            return Ok( OptionParam { param : None } )
        }

        let s = s.trim();
        let s = &s[5..];
        let s = &s[..(s.len() - 1)];
        let s = s.trim();

        Ok( OptionParam { param : Some( s.to_string() )} )
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, World)]
pub struct FileWorld
{
    pub path : Vec<String>,
    pub name : Vec<String>,
    pub filename : Vec<String>,
    pub content_type : Vec<String>,
    pub point : Option<Point>,
    pub len : Option<usize>
}