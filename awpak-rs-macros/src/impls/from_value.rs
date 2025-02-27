use proc_macro::TokenStream;
use quote::quote;


pub fn from_value_impl( item : TokenStream ) -> TokenStream
{
    let input = syn::parse_macro_input!( item as syn::DeriveInput );

    let ident = input.ident;

    quote! {

        impl awpak_rs::from_value::FromValue for #ident
        {
            fn from_value( value : &awpak_rs::Value ) -> Option<Self>
            {
                awpak_rs::parse_from_value( value )
            }
        }

    }.into()
}