use proc_macro::TokenStream;

use super::{get::get_docs, methods::methods_impl};

pub fn delete_impl( args: TokenStream, item: TokenStream ) -> TokenStream
{
    methods_impl( args, item, "delete", get_docs() )
}