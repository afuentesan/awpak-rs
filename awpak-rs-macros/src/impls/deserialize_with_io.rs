use proc_macro::TokenStream;
use quote::{quote, ToTokens as _};
use syn::{Data, Fields};


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

    let mut field_names = Vec::new();
    let mut field_parsers = Vec::new();
    let mut field_assigns = Vec::new();
    let mut field_not_found = Vec::new();

    for field in fields.named.iter()
    {
        let field_name = &field.ident;
        field_names.push( field_name );

        let field_type = field.ty.clone();
        
        match get_with_context_function( &field.attrs )
        {
            Some( f ) => {

                field_not_found.push(
                    quote!
                    {
                        if #field_name.is_none() && unused_fields.contains( &stringify!(#field_name) )
                        {
                            #field_name = Some( #f( "".to_string(), io.clone() ).map_err(serde::de::Error::custom)? );
                        }
                    }
                );

                field_parsers.push( 
                    quote!
                    {
                        if let Some(index) = unused_fields.iter().position(|value| *value == stringify!(#field_name) ) {
                            unused_fields.swap_remove(index);
                        };

                        let __value = map.next_value::<&awpak_rs::RawValue>()?;

                        #field_name = Some( #f( __value.to_string(), io.clone() ).map_err(serde::de::Error::custom)? );
                    }
                );
                
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

                        let __value = map.next_value::<&awpak_rs::RawValue>()?;

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
                // T: for<'a> serde::Deserialize<'a> + awpak_rs::io::deserializer::deserialize_with_io::DeserializeWithIO
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
                                // if let Some(index) = unused_fields.iter().position(|value| *value == stringify!(#field_names) ) {
                                //     unused_fields.swap_remove(index);
                                // };

                                #field_parsers 
                            }
                        )*
                        _ => continue
                    }
                }

                #( #field_not_found )*
                
                Ok(#ident {
                    //#( #field_names: #field_names.ok_or_else(|| serde::de::Error::missing_field(stringify!(#field_names)))? ),*

                    #( #field_assigns )*
                })
            }
        }

    }.into()
}

fn get_with_context_function( attrs: &[ syn::Attribute ] ) -> Option<syn::Ident>
{
    for attr in attrs
    {
        // println!( "Attribute: {:#?}", attr );

        if let Ok( syn::Meta::List( meta_list ) ) = attr.meta.clone().try_into()
        {
            // println!( "Entra en meta list" );

            if attr.path().is_ident( "io_deserializer" )
            {
                // println!( "Is ident io_deserializer" );

                for nested_meta in meta_list.tokens.into_iter()
                {
                    // println!( "{:#?}", nested_meta );

                    if let Ok( f ) = syn::parse2::<syn::Ident>( nested_meta.into() )
                    {
                        return Some( f )
                    }

                    // if let Ok(syn::Meta::NameValue( name_value ) ) = syn::parse2::<syn::Meta>( nested_meta.into() )
                    // {
                    //     return Some( syn::Ident::new( &name_value.to_token_stream().to_string(), attr.span() ) )
                    // }
                }
            }
        }
    }

    None
}
