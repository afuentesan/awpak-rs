use std::{marker::PhantomData, sync::{Arc, Mutex}};

use serde::{de::{MapAccess, Visitor}, Deserializer};

use crate::io::io::IO;

use super::deserialize_with_io::DeserializeWithIO;

pub struct IODeserializeSeed<T>
where T: for<'a> serde::Deserialize<'a>
{
    pub io : Arc<Mutex<Option<IO>>>, 
    _phantom : PhantomData<T>
}

impl<T> IODeserializeSeed<T>
where T: for<'a> serde::Deserialize<'a>
{
    pub fn new( io : Arc<Mutex<Option<IO>>> ) -> Self
    {
        Self
        {
            io,
            _phantom : PhantomData::<T>
        }
    }
}

impl<'de, T> serde::de::DeserializeSeed<'de> for IODeserializeSeed<T>
where T: for<'a> serde::Deserialize<'a> + DeserializeWithIO
{
    type Value = T;

    fn deserialize<D>( self, deserializer: D ) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct __VisitorWithContext<T>
        where T: for<'a> serde::Deserialize<'a> + DeserializeWithIO
        {
            io: std::sync::Arc<std::sync::Mutex<std::option::Option<IO>>>,
            _phantom : PhantomData<T>
        }

        impl<'de, T> Visitor<'de> for __VisitorWithContext<T>
        where T: for<'a> serde::Deserialize<'a> + DeserializeWithIO
        {
            type Value = T;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result
            {
                formatter.write_str( &format!( "struct {}", Self::Value::get_name() ) )
            }

            fn visit_map<A>( self, mut map: A ) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                Self::Value::deserialize_seed_body(
                    self.io.clone(),
                    &mut map
                )
            }
        }
        
        deserializer.deserialize_struct
        (
            Self::Value::get_name(),
            Self::Value::get_fields(),
            __VisitorWithContext { io : self.io, _phantom : PhantomData::<T> }
        )
    }
}