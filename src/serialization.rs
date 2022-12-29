use std::{fmt, marker::PhantomData};

use serde::{
  de::{self, IntoDeserializer, MapAccess, Visitor},
  Deserialize, Deserializer,
};

pub(crate) fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
  D: serde::Deserializer<'de>,
  T: serde::Deserialize<'de>,
{
  let opt = Option::<String>::deserialize(de)?;
  let opt = opt.as_ref().map(String::as_str);
  match opt {
    None | Some("") => Ok(None),
    Some(s) => T::deserialize(s.into_deserializer()).map(Some),
  }
}

pub(crate) fn string_or_struct<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
  T: Deserialize<'de>,
  D: Deserializer<'de>,
{
  struct StringOrStruct<T>(PhantomData<fn() -> Option<T>>);

  impl<'de, T> Visitor<'de> for StringOrStruct<Option<T>>
  where
    T: Deserialize<'de>,
  {
    type Value = Option<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("string or map")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      match value {
        "" => Ok(None),
        s => T::deserialize(s.into_deserializer()).map(Some),
      }
    }

    fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
    where
      M: MapAccess<'de>,
    {
      Deserialize::deserialize(de::value::MapAccessDeserializer::new(map)).map(Some)
    }
  }

  deserializer.deserialize_any(StringOrStruct(PhantomData))
}

pub(crate) fn opt_string_or_struct<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
  T: Deserialize<'de>,
  D: Deserializer<'de>,
{
  struct OptStringOrStruct<T>(PhantomData<T>);

  impl<'de, T> Visitor<'de> for OptStringOrStruct<T>
  where
    T: Deserialize<'de>,
  {
    type Value = Option<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("a nul, a string or map")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(None)
    }

    fn visit_some<D>(self, de: D) -> Result<Self::Value, D::Error>
    where
      D: Deserializer<'de>,
    {
      string_or_struct(de)
    }
  }

  deserializer.deserialize_option(OptStringOrStruct(PhantomData))
}
