use proc_macro::TokenStream;
use darling::FromMeta;
use quote::quote;
use syn::{ItemFn, LitStr, Signature};

use crate::util::utils::get_attributes;

#[derive(FromMeta)]
struct MacroMiddlewareData
{
    urls : Option<Vec<LitStr>>,
    order : Option<usize>,
    execute_after : Option<bool>,
    method : Option<String>
}

pub fn middleware_impl( args: TokenStream, item: TokenStream ) -> TokenStream
{
    let middleware_data : MacroMiddlewareData = match get_attributes( args ) {
        Ok( v ) => v,
        Err( e ) => return e
    };

    let ItemFn {
        block,
        attrs,
        mut sig,
        ..

    } = syn::parse_macro_input!( item as ItemFn );

    sig.output = syn::parse( quote! {
        -> awpak_rs::MiddlewareResponseType
    }.into() ).unwrap();

    let inventory = get_inventory( 
        &sig,
        &middleware_data
    );

    quote! {
        #(#attrs)*
        #sig
        {
            std::boxed::Box::pin( async move #block )
        }

        #inventory
    }.into()
}

fn get_inventory( 
    sig : &Signature, 
    data : &MacroMiddlewareData
) -> proc_macro2::TokenStream
{
    let regex = get_regex( &data.urls );
    let order = get_order( &data.order );
    let execute_after = get_execute_after( &data.execute_after );

    let fnc_ident = sig.ident.clone();

    let regex = match regex {
        Some( v ) => quote! { Some( #v ) },
        _ => quote! { None }
    };
    
    let order = quote! { #order };

    let exec_order = if execute_after
    {
        quote! { awpak_rs::MiddlewareExecOrder::POST }
    }
    else
    {
        quote! { awpak_rs::MiddlewareExecOrder::PRE }
    };

    let method = match &data.method {
        Some( v ) => {
            
            let v = v.to_lowercase();

            quote! { Some( #v ) }
        },
        _ => quote! { None }
    };

    quote!
    {
        awpak_rs::inventory::submit! {
            awpak_rs::Middleware::new(
                #regex, 
                #order,
                #method,
                | __middleware_io | { #fnc_ident( __middleware_io ) },
                #exec_order
            )
        }
    }
}

fn get_regex( urls : &Option<Vec<LitStr>> ) -> Option<String>
{
    match urls {
        Some( v ) => {
            let mut ret : String = get_item_regex( &v[ 0 ].value() );

            for i in 1..v.len()
            {
                let s = v[ i ].clone();
                ret = format!( "{}|{}", ret, get_item_regex( &s.value() ) );
            }

            Some( ret )
        },
        _ => {
            None
        }
    }
}

fn get_item_regex( regex : &String ) -> String
{
    let mut regex = regex.clone().trim().to_string();

    if ! regex.starts_with( "^" )
    {
        regex = format!( "^{}", regex );
    }

    if ! regex.ends_with( "$" )
    {
        regex = format!( "{}$", regex );
    }

    regex
}

fn get_order( order : &Option<usize> ) -> usize
{
    match order
    {
        Some( v ) => *v,
        _ => 10000    
    }
}

fn get_execute_after( execute_after : &Option<bool> ) -> bool
{
    match execute_after
    {
        Some( v ) => *v,
        _ => false    
    }
}