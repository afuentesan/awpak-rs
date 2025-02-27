use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;


pub fn set_status_code_impl( item : TokenStream ) -> TokenStream
{
    let args = parse_macro_input!( item as proc_macro2::TokenStream );

    quote! {
        awpak_rs::set_status_code_mcr!( __io, #args );
    }.into()
}