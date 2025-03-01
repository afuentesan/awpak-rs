use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, FnArg, Ident, ItemFn, PatIdent, PatType, ReturnType, Signature, Visibility};

use crate::util::utils::get_attributes;

#[derive(FromMeta)]
struct MacroEndpointData
{
    url : String
}

pub fn methods_impl( args: TokenStream, item: TokenStream, method : &str ) -> TokenStream
{
    let MacroEndpointData { url } = match get_attributes( args ) {
        Ok( v ) => v,
        Err( e ) => return e
    };

    let ItemFn {
        block,
        attrs,
        sig,
        vis

    } = syn::parse_macro_input!( item as ItemFn );

    let output = sig.output.clone();

    let return_type = match output {
        ReturnType::Type( _a, b ) => quote! {#b},
        _ => quote! {()}
    };
    
    let ( variables, post_variables ) = get_variables( &sig, &url );

    let new_signature = get_signature( &vis, &sig.ident );
    let new_ident = sig.ident;

    let salida = quote! {

        __io.response.body = awpak_rs::serialize_value::<#return_type>( __result );

        __io.response.headers.replace_headers( __response_headers );

        Ok( __io )
    };

    quote! {

        #(#attrs)*
        #new_signature
        {
            std::boxed::Box::pin( async move {

                #variables
                
                let mut __response_headers = awpak_rs::io::headers::headers::Headers::new();

                let __result = async #block.await;

                #post_variables

                #salida

            } )
        }
        
        awpak_rs::inventory::submit! {
            awpak_rs::endpoint::endpoint::Endpoint::new( #url, #method, | __awpak_rs_param_a | { #new_ident( __awpak_rs_param_a ) } )
        }
    }.into()
}

fn get_signature( vis : &Visibility, sig : &Ident) -> proc_macro2::TokenStream
{
    quote! {
        #vis fn #sig( mut __io : awpak_rs::io::io::IO ) -> awpak_rs::endpoint::types::EndpointReturnType
    }
}

fn get_variables( sig : &Signature, url : &String ) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    let mut variables = quote! {};
    let mut post_variables = quote! {};

    sig.inputs.iter().for_each( | i | {

        let ( variable, post_variable ) =  get_variable( i, sig, url );

        variables.extend( variable );
        post_variables.extend( post_variable );

    } );

    ( variables, post_variables )
}

fn get_variable( arg : &FnArg, sig : &Signature, url : &String ) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    if let syn::FnArg::Typed( argument ) = arg {
        
        let from = get_variable_attribute( argument );

        if from.is_none()
        {
            return ( quote! {}, quote! {} )
        }

        if let syn::Pat::Ident( pat_ident ) = *argument.pat.clone()
        {
            let from = from.unwrap();
            let ty = argument.ty.clone();

            let priv_pat_ident = Ident::new( &format!( "__{}", pat_ident.ident.to_string() ), sig.span() );

            let fake_attr = Ident::new( &from, sig.span() );

            return declare_variable( from, ty, priv_pat_ident, fake_attr, pat_ident, url )
        }
    }

    ( quote! {}, quote! {} )
}

fn declare_variable( 
    from : String, 
    ty : Box<syn::Type>, 
    priv_pat_ident : Ident, 
    fake_attr : Ident, 
    pat_ident : PatIdent,
    url : &String
) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    match from.as_str()
    {
        "request_body" => declare_variable_object( &from, ty, priv_pat_ident, fake_attr, pat_ident ),
        "body_param" => declare_variable_body_param( ty, priv_pat_ident, fake_attr, pat_ident ),
        "query_params" => declare_variable_object( &from, ty, priv_pat_ident, fake_attr, pat_ident ),
        "part_file" => declare_variable_file( ty, priv_pat_ident, fake_attr, pat_ident ),
        "part_files" => declare_variable_file( ty, priv_pat_ident, fake_attr, pat_ident ),
        "path_variable" => declare_variable_path( ty, priv_pat_ident, fake_attr, pat_ident, url ),
        "context" => declare_variable_context( ty, fake_attr, pat_ident ),
        "request_headers" => declare_variable_headers( ty, fake_attr, pat_ident, true ),
        "response_headers" => declare_variable_headers( ty, fake_attr, pat_ident, false ),
        "request_cookies" => declare_variable_cookies( ty, fake_attr, pat_ident, true ),
        "response_cookies" => declare_variable_cookies( ty, fake_attr, pat_ident, false ),
        "query_param" => declare_variable_query_param( ty, priv_pat_ident, fake_attr, pat_ident ),
        _ => unreachable!()
    }
}

fn declare_variable_query_param( 
    ty : Box<syn::Type>, priv_pat_ident : Ident, fake_attr : Ident, pat_ident : PatIdent 
) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    let name = pat_ident.ident.to_string();

    let optional_part = quote! {
        if #priv_pat_ident.is_none()
        {
            println!( "Parse query param value err" );
            
            return Err( awpak_rs::error::error::Error::ParserError( format!( "Query param error: {}", #name ) ) )
        }
    };

    let final_assign = quote! {
        let #pat_ident = #priv_pat_ident.unwrap();
    };

    (
        quote! {
            #fake_attr!();
            let #priv_pat_ident = awpak_rs::parse_query_param_value::<#ty>( &__io, #name );
            #optional_part
            #final_assign
        },
        quote! {}
    )
}

fn declare_variable_cookies(
    ty : Box<syn::Type>, 
    fake_attr : Ident, 
    pat_ident : PatIdent,
    request : bool
) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    let ident_from = if request
    {
        quote! { request }
    }
    else
    {
        quote! { response }
    };

    let ident_assign = quote! {
        let #pat_ident : #ty = __io.#ident_from.get_cookies();
    };

    let post_ident_assign = if pat_ident.mutability.is_some()
    {
        let post_pat_ident = pat_ident.ident;

        quote! {
            __io.#ident_from.cookies = #post_pat_ident;
        }
    }
    else
    {
        quote! {}
    };

    (
        quote! {
            #fake_attr!();
            #ident_assign
        },
        post_ident_assign
    )
}

fn declare_variable_headers(
    ty : Box<syn::Type>, 
    fake_attr : Ident, 
    pat_ident : PatIdent,
    request : bool
) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    let ident_from = if request
    {
        quote! { request }
    }
    else
    {
        quote! { response }
    };

    let ident_assign = quote! {
        let #pat_ident : #ty = __io.#ident_from.get_headers();
    };

    let post_ident_assign = if pat_ident.mutability.is_some()
    {
        let post_pat_ident = pat_ident.ident;

        quote! {
            __io.#ident_from.headers = #post_pat_ident;
        }
    }
    else
    {
        quote! {}
    };

    (
        quote! {
            #fake_attr!();
            #ident_assign
        },
        post_ident_assign
    )
}

fn declare_variable_context(
    ty : Box<syn::Type>, 
    fake_attr : Ident, 
    pat_ident : PatIdent
) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    let ident_assign = if ty.clone().to_token_stream().to_string().contains( "& mut " )
    {
        quote! {
            let #pat_ident : #ty = __io.get_context_mut();
        }
    }
    else
    {
        quote! {
            let #pat_ident : #ty = __io.get_context();
        }
    };

    (
        quote! {
            #fake_attr!();
            #ident_assign
        },
        quote! {}
    )
}

fn declare_variable_path( 
    ty : Box<syn::Type>, 
    priv_pat_ident : Ident, 
    fake_attr : Ident, 
    pat_ident : PatIdent,
    url : &String
) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    let name = pat_ident.to_token_stream().to_string();

    match get_ind_path_variable( name.clone(), url )
    {
        Ok( v ) => {
            (
                quote! {
                    #fake_attr!();
                    let #priv_pat_ident = awpak_rs::parse_path_variable::<#ty>( &__io, #v ).await;
                    if #priv_pat_ident.is_none()
                    {
                        return Err( awpak_rs::error::error::Error::ParserError( format!( "Path variable error: {}", #name ) ) )
                    }
                    let #pat_ident = #priv_pat_ident.unwrap();
                },
                quote! {}
            )
        },
        Err( e ) => panic!( "{}", e )
    }
}

fn get_ind_path_variable( name : String, url : &String ) -> Result<usize, String>
{
    let name = format!( "{{{}}}", name );

    match url.split( "/" ).enumerate().find(  | v | v.1 == name ).map( | v | v.0 )
    {
        Some( v ) => Ok( v ),
        _ => Err( "".to_string() )
    }
}

fn declare_variable_body_param( 
    ty : Box<syn::Type>, priv_pat_ident : Ident, fake_attr : Ident, pat_ident : PatIdent 
) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    let name = pat_ident.ident.to_string();

    let optional_part = quote! {
        if #priv_pat_ident.is_none()
        {
            return Err( awpak_rs::error::error::Error::ParserError( format!( "Body param error: {}", #name ) ) )
        }
    };

    let final_assign = quote! {
        let #pat_ident = #priv_pat_ident.unwrap();
    };

    (
        quote! {
            #fake_attr!();
            let #priv_pat_ident = awpak_rs::parse_body_param_value::<#ty>( &__io, #name );
            #optional_part
            #final_assign
        },
        quote! {}
    )
}

fn declare_variable_file( 
    ty : Box<syn::Type>, priv_pat_ident : Ident, fake_attr : Ident, pat_ident : PatIdent 
) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    let filename = pat_ident.ident.to_string();

    let priv_pat_ident_assign = quote! {
        let #priv_pat_ident = {
            use awpak_rs::io::request::request_body::ToFileData;
            <#ty>::to_file_data( &__io.request.body, #filename )
        };
    };

    let optional_part = quote! {
        if #priv_pat_ident.is_err()
        {
            return Err( awpak_rs::error::error::Error::ParserError( format!( "File param error: {}", #filename ) ) )
        }
    };

    let final_assign = quote! {
        let #pat_ident = #priv_pat_ident.unwrap();
    };

    (
        quote! {
            #fake_attr!();
            #priv_pat_ident_assign
            #optional_part
            #final_assign
        },
        quote! {}
    )
}

fn declare_variable_object( 
    from : &str, ty : Box<syn::Type>, priv_pat_ident : Ident, fake_attr : Ident, pat_ident : PatIdent 
) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    let name = pat_ident.ident.to_string();

    (
        quote! {

            #fake_attr!();

            let #priv_pat_ident = awpak_rs::parse_value::<#ty>( &__io, #from );

            if #priv_pat_ident.is_none()
            {
                return Err( awpak_rs::error::error::Error::ParserError( format!( "{} error: {}", #from, #name ) ) )
            }

            let #pat_ident = #priv_pat_ident.unwrap();
        },
        quote! {}
    )
}

fn get_variable_attribute( arg : &PatType ) -> Option<String>
{
    for i in 0..arg.attrs.len()
    {
        let found = match_variable_attribute( &arg.attrs[ i ].meta.path().to_token_stream().to_string() );

        if found.is_some()
        {
            return found;
        }
    }

    None
}

fn match_variable_attribute( attr : &str ) -> Option<String>
{
    match attr {
        "request_body" => Some( "request_body".to_string() ),
        "body_param" => Some( "body_param".to_string() ),
        "query_params" => Some( "query_params".to_string() ),
        "part_file" => Some( "part_file".to_string() ),
        "part_files" => Some( "part_files".to_string() ),
        "path_variable" => Some( "path_variable".to_string() ),
        "context" => Some( "context".to_string() ),
        "request_headers" => Some( "request_headers".to_string() ),
        "response_headers" => Some( "response_headers".to_string() ),
        "request_cookies" => Some( "request_cookies".to_string() ),
        "response_cookies" => Some( "response_cookies".to_string() ),
        "query_param" => Some( "query_param".to_string() ),
        _ => None
    }
}