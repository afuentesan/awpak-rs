use proc_macro::TokenStream;

use super::methods::methods_impl;

pub fn head_impl( args: TokenStream, item: TokenStream ) -> TokenStream
{
    methods_impl( args, item, "head" )
}