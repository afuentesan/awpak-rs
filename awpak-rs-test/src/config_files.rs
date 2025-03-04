use awpak_rs::{config_file, get};
use serde::Deserialize;


#[config_file( path="tests/files/config/config_1.json" )]
#[derive(Deserialize)]
pub struct DbConfigFile
{
    pub db_user : String,
    pub db_password : String
}

#[config_file( path="tests/files/config/config_2.json" )]
pub struct DbConfigFileValue;

#[config_file( path="tests/files/config/config_3.json" )]
#[derive(Deserialize)]
pub struct DbConfigFileEnvVar
{
    pub db_user : String,
    pub db_password : String
}

#[get( url = "/get_db_config_file_user_pass" )]
fn get_db_config_file_user_pass() -> String
{
    format!( "{}:{}", DbConfigFile::get_config_file().db_user, DbConfigFile::get_config_file().db_password )
}

#[get( url = "/get_db_config_file_user_pass_value" )]
fn get_db_config_file_user_pass_value() -> String
{
    format!( "{}:{}", 
        DbConfigFileValue::get_config_file().get( "db_user" ).unwrap().as_str().unwrap(),
        DbConfigFileValue::get_config_file().get( "db_password" ).unwrap().as_str().unwrap()
    )
}

#[get( url = "/get_db_config_file_user_pass_env_vars" )]
fn get_db_config_file_user_pass_env_vars() -> String
{
    format!( "{}:{}", DbConfigFileEnvVar::get_config_file().db_user, DbConfigFileEnvVar::get_config_file().db_password )
}