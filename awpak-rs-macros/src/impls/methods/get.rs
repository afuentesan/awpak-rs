use proc_macro::TokenStream;
use quote::quote;

use super::methods::methods_impl;

pub fn get_impl( args: TokenStream, item: TokenStream ) -> TokenStream
{
    methods_impl( args, item, "get", get_docs() )
}

pub fn get_docs() -> proc_macro2::TokenStream
{
    quote!
    {
        #[doc="`query_params` macro deserializes multiple query parameters into a struct."]
    }
}