use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, ItemStruct};
use darling::FromMeta;

use crate::util::utils::get_attributes;

#[derive(FromMeta)]
struct MacroConfigFile
{
    path : String
}

pub fn config_file_impl( args : TokenStream, item : TokenStream ) -> TokenStream
{
    let config_file_data : MacroConfigFile = match get_attributes( args ) {
        Ok( v ) => v,
        Err( e ) => return e
    };

    let item_struct = syn::parse_macro_input!( item as ItemStruct );

    let fn_name = get_fn_name( &item_struct );
    let return_type = get_return_type( &item_struct );

    let static_fn = get_static_fn( &item_struct, &fn_name, &return_type, get_path( &config_file_data ) );
    let struct_impl = get_struct_impl( &item_struct, &fn_name, &return_type );

    let inventory = get_inventory( &fn_name );

    quote!
    {
        #item_struct

        #static_fn

        #struct_impl

        #inventory

    }.into()
}

fn get_return_type( item_struct : &ItemStruct ) -> proc_macro2::TokenStream
{
    if item_struct.fields.is_empty()
    {
        return quote!
        {
            awpak_rs::Value
        };
        // return syn::Ident::new( "awpak_rs::Value", item_struct.span() );
    }

    let ident = item_struct.ident.clone();

    quote!
    {
        #ident
    }
}

fn get_static_fn( item_struct : &ItemStruct, fn_name : &syn::Ident, return_type : &proc_macro2::TokenStream, file_path : String ) -> proc_macro2::TokenStream
{
    let name_obj = syn::Ident::new( 
        &format!( "__AWPAK_RS_CONFIG_FILE_{}", item_struct.ident.to_string().to_uppercase() ),
        item_struct.span()
    );

    quote!
    {
        fn #fn_name() -> &'static #return_type {
            static #name_obj: std::sync::OnceLock<#return_type> = std::sync::OnceLock::new();
            #name_obj.get_or_init(|| awpak_rs::config_data::config_file::config_file::parse_config_file::<#return_type>( #file_path ).unwrap() )
        }
    }
}

fn get_struct_impl( item_struct : &ItemStruct, fn_name : &syn::Ident, return_type : &proc_macro2::TokenStream ) -> proc_macro2::TokenStream
{
    let struct_ident = item_struct.ident.clone();

    quote!
    {
        impl #struct_ident
        {
            pub fn get_config_file() -> &'static #return_type
            {
                #fn_name()
            }
        }
    }
}

fn get_fn_name( item_struct : &ItemStruct ) -> syn::Ident
{
    syn::Ident::new(
        &format!( "__awpak_rs_get_config_{}", item_struct.ident.to_string().to_lowercase() ), 
        item_struct.span()
    )
}

fn get_path( config : &MacroConfigFile ) -> String
{
    let mut path = config.path.clone();

    while path.starts_with( std::path::MAIN_SEPARATOR_STR )
    {
        path = path[ 1.. ].to_string();
    }

    path
}

fn get_inventory( fn_name : &syn::Ident ) -> proc_macro2::TokenStream
{
    quote!
    {
        awpak_rs::inventory::submit! {
            awpak_rs::config_data::config_file::config_file::InitConfigFile
            {
                fnc : || { let _ = #fn_name(); }
            }
        }
    }
    
}