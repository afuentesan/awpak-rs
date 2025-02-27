use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;


pub fn redirect_to_impl( item : TokenStream ) -> TokenStream
{
    let args = parse_macro_input!( item as proc_macro2::TokenStream );

    quote! {
        awpak_rs::redirect_to_mcr!( __io, __response_headers, #args );
    }.into()
}