use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

use crate::util::utils::get_attributes;

const DEFAULT_IP : &str = "127.0.0.1";
const DEFAULT_PORT : &str = "3000";

#[derive(FromMeta)]
struct MacroServerParams
{
    ip : Option<String>,
    port : Option<String>
}

impl MacroServerParams
{
    pub fn default() -> Self
    {
        Self
        {
            ip : None,
            port : None
        }
    }

    fn get_token_ip( &self ) -> proc_macro2::TokenStream
    {
        if self.ip.is_none()
        {
            quote! { #DEFAULT_IP }
        }
        else
        {
            let ip = self.ip.as_ref().unwrap();

            quote! { #ip }
        }
    }

    fn get_token_port( &self ) -> proc_macro2::TokenStream
    {
        if self.port.is_none()
        {
            quote! { #DEFAULT_PORT }
        }
        else
        {
            let port = self.port.as_ref().unwrap();

            quote! { #port }
        }
    }
}

pub fn awpak_main_impl( args: TokenStream, item: TokenStream ) -> TokenStream
{
    let ItemFn {
        block,
        attrs,
        ..
    } = parse_macro_input!( item as ItemFn );

    let uses = get_uses();
    let macros = get_macros();
    let signature = get_signature();

    let statements = block.stmts;

    let server_statement = get_server_statement( args );

    let initialize_middlewares = quote! { awpak_rs::initialize_middlewares(); };

    quote! {
        #uses

        #(#attrs)*
        #macros
        #signature
        {
            #initialize_middlewares
            
            #(#statements)*

            #server_statement
        }
    }.into()
}

fn get_server_statement( args : TokenStream ) -> proc_macro2::TokenStream
{
    let server_params = get_server_params( args );

    let ip = server_params.get_token_ip();

    let port = server_params.get_token_port();

    generate_server_statement( ip, port )
}

fn get_server_params( args : TokenStream ) -> MacroServerParams
{
    match get_attributes( args )
    {
        Ok( v ) => v,
        _ => MacroServerParams::default()    
    }
}

fn generate_server_statement( ip : proc_macro2::TokenStream, port : proc_macro2::TokenStream ) -> proc_macro2::TokenStream
{
    quote! {
        awpak_rs::server::server::server( #ip, #port ).await
    }
}

fn get_uses() -> proc_macro2::TokenStream
{
    quote! {
        use awpak_rs::tokio;
    }
}

fn get_macros() -> proc_macro2::TokenStream
{
    quote! {
        #[awpak_rs::tokio::main]
    }
}

fn get_signature() -> proc_macro2::TokenStream
{
    quote! {
        async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    }
}