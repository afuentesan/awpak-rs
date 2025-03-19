use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{quote, ToTokens as _};
use syn::{Data, Fields};

use crate::util::utils::get_attributes;


pub fn deserialize_with_io_impl( item : TokenStream ) -> TokenStream
{
    let input = syn::parse_macro_input!( item as syn::DeriveInput );

    let ident = input.ident;

    let Data::Struct(data_struct) = input.data else {
        panic!("DeserializeWithIO solo soporta structs");
    };

    let Fields::Named(fields) = data_struct.fields else {
        panic!("Solo soporta structs con campos nombrados");
    };

    let fn_name_prefix = format!( "__awpak_rs_async_fn_deserializer_{}_", ident.to_string().to_lowercase() );

    let mut field_names = Vec::new();
    let mut field_parsers = Vec::new();
    let mut field_assigns = Vec::new();
    let mut field_not_found = Vec::new();
    let mut async_des_fncs = Vec::new();


    for field in fields.named.iter()
    {
        let field_name = &field.ident;
        field_names.push( field_name );

        let field_type = field.ty.clone();
        
        match get_with_context_function( &field.attrs )
        {
            Some( f ) => {

                let ( fn_definition, fn_name ) = get_wrapper_function(
                    &fn_name_prefix, 
                    field_name, 
                    field_type.clone(), 
                    f.clone()
                );

                field_not_found.push(
                    quote!
                    {
                        if #field_name.is_none() && unused_fields.contains( &stringify!(#field_name) )
                        {
                            #field_name = Some( #fn_name( "".to_string(), io.clone() ).map_err(serde::de::Error::custom)? );
                        }
                    }
                );

                field_parsers.push( 
                    quote!
                    {
                        if let Some(index) = unused_fields.iter().position(|value| *value == stringify!(#field_name) ) {
                            unused_fields.swap_remove(index);
                        };

                        let __value = map.next_value::<awpak_rs::Value>()?;

                        let __value = match __value
                        {
                            awpak_rs::Value::String( s ) => s,
                            _ => __value.to_string()
                        };

                        #field_name = Some( #fn_name( __value, io.clone() ).map_err(serde::de::Error::custom)? );
                    }
                );

                async_des_fncs.push( fn_definition );
                
            },
            _ =>
            {
                let deserialize_token = quote!
                {
                    match awpak_rs::parse_bytes_with_io::<#field_type>( io.clone(), std::sync::Arc::new( __value.into() ) )
                    {
                        Some( v ) => {
                            #field_name = Some( v );
                        },
                        _ => return Err( serde::de::Error::missing_field(stringify!(#field_name)) )
                    }
                };

                field_not_found.push(
                    quote!
                    {
                        if #field_name.is_none() && unused_fields.contains( &stringify!(#field_name) )
                        {
                            use awpak_rs::io::deserializer::deserialize_with_io::DeserializeWithIO;

                            let __value : Vec<u8> = Vec::new();

                            #deserialize_token
                        }
                    }
                );

                field_parsers.push( 

                    quote! 
                    {
                        if let Some(index) = unused_fields.iter().position(|value| *value == stringify!(#field_name) ) {
                            unused_fields.swap_remove(index);
                        };

                        use awpak_rs::io::deserializer::deserialize_with_io::DeserializeWithIO;

                        let __value = map.next_value::<awpak_rs::Value>()?;

                        let __value = match __value
                        {
                            awpak_rs::Value::String( s ) => s,
                            _ => __value.to_string()
                        };

                        let __value = __value.to_string().as_bytes().to_vec();

                        #deserialize_token
                    } 

                );
            }    
        };

        let default_fn = field.attrs.iter().any(|attr| attr.path().is_ident("serde") && attr.to_token_stream().to_string().contains("default"));

        if default_fn
        {
            field_assigns.push(
                quote!
                {
                    #field_name : #field_name.unwrap_or_else( Default::default ),
                }
            );
        }
        else
        {
            field_assigns.push(
                quote!
                {
                    #field_name : #field_name.ok_or_else(|| serde::de::Error::missing_field(stringify!(#field_name)))?,
                }
            );
        }
    }

    quote!
    {
        impl awpak_rs::io::deserializer::deserialize_with_io::DeserializeWithIO for #ident
        {
            fn get_name() -> &'static str
            {
                stringify!(#ident)
            }

            fn get_fields() -> &'static [&'static str]
            {
                &[ #( stringify!(#field_names) ),* ]
            }

            fn deserialize_seed_body<'de, A>( io : std::sync::Arc<std::sync::Mutex<Option<awpak_rs::io::io::IO>>>, map : &mut A ) -> Result<#ident, A::Error>
            where 
                A: serde::de::MapAccess<'de>,
            {
                let mut unused_fields : Vec<&str> = vec![];

                #( 
                    let mut #field_names = None; 
                    unused_fields.push( stringify!( #field_names ) );
                )*

                while let Some( key ) = map.next_key::<&str>()?
                {
                    match key
                    {
                        #( stringify!(#field_names) => 
                            {
                                #field_parsers 
                            }
                        )*
                        _ => continue
                    }
                }

                #( #field_not_found )*
                
                Ok(#ident {
                    #( #field_assigns )*
                })
            }
        }

        #( #async_des_fncs )*

    }.into()
}

#[derive(FromMeta)]
struct AttributeOptions
{
    deserialize_with : proc_macro2::Ident
}

fn get_with_context_function( attrs: &[ syn::Attribute ] ) -> Option<syn::Ident>
{
    for attr in attrs
    {
        if let Ok( syn::Meta::List( _ ) ) = attr.meta.clone().try_into()
        {
            if attr.path().is_ident( "io_deserializer" )
            {
                let AttributeOptions { deserialize_with } = match get_attributes( attr.meta.require_list().ok()?.tokens.clone().into() ) {
                    Ok( v ) => v,
                    Err( _e ) => continue
                };

                return Some( deserialize_with )

                // for nested_meta in meta_list.tokens.into_iter()
                // {
                //     if let Ok( f ) = syn::parse2::<syn::Ident>( nested_meta.into() )
                //     {
                //         return Some( f )
                //     }
                // }
            }
        }
    }

    None
}

fn get_wrapper_function( 
    prefix : &str, 
    field : &Option<proc_macro2::Ident>,
    ty : syn::Type,
    inner_fn : proc_macro2::Ident
) -> ( proc_macro2::TokenStream, proc_macro2::Ident )
{
    let fn_name = proc_macro2::Ident::new(
        &format!( "{}{}", prefix, field.as_ref().unwrap().to_string() ), 
        field.as_ref().unwrap().span()
    );

    (
        quote!
        {
            fn #fn_name( 
                input : std::string::String, 
                io : std::sync::Arc<std::sync::Mutex<std::option::Option<awpak_rs::io::io::IO>>> 
            ) -> std::result::Result<#ty, std::string::String>
            {
                let ( tx, rx ) = std::sync::mpsc::channel();

                let handle = awpak_rs::tokio::runtime::Handle::current();

                let io_clone = io.clone();

                let _ = std::thread::spawn( move ||
                    {
                    
                        handle.block_on( async move
                            {
                                let result = #inner_fn( 
                                    input, 
                                    io_clone.lock().unwrap().as_ref().take().unwrap() 
                                ).await;

                                match tx.send( result )
                                {
                                    _ => {}    
                                }
                            }
                        );
                    }
                ).join();

                let val = match rx.recv()
                {
                    std::result::Result::Ok( v ) => v,
                    std::result::Result::Err( e ) => return Err( e.to_string() )
                };

                val
            }
        },
        fn_name
    )
}
