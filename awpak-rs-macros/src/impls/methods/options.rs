use proc_macro::TokenStream;

use super::{get::get_docs, methods::methods_impl};

pub fn options_impl( args: TokenStream, item: TokenStream ) -> TokenStream
{
    methods_impl( args, item, "options", get_docs() )
}