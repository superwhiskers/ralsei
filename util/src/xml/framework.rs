//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use async_trait::async_trait;
use deadpool::managed::{Manager, Pool, RecycleResult};
use quick_xml::{Reader, Writer};
use std::{
    borrow::Cow,
    error::Error as StdError,
    fmt::Debug,
    io::{BufRead, Read, Write},
    result::Result as StdResult,
};

use crate::xml::errors::ResultWithError;

/// A type alias for a [`Pool`] of [`Vec<u8>`]s
///
/// [`Pool`]: https://docs.rs/deadpool/0.6.0/deadpool/managed/struct.Pool.html
/// [`Vec<u8>`]: https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html
pub type BufferPool = Pool<Vec<u8>, !>;

/// The [`Manager`] implementation used by [`Pool`]s that are passed to implementors of [`FromXml`]
///
/// [`Manager`]: https://docs.rs/deadpool/0.6.0/deadpool/managed/trait.Manager.html
/// [`Pool`]: https://docs.rs/deadpool/0.6.0/deadpool/managed/struct.Pool.html
/// [`FromXml`]: ./trait.FromXml.html
pub struct BufferPoolManager;

#[async_trait]
impl Manager<Vec<u8>, !> for BufferPoolManager {
    async fn create(&self) -> StdResult<Vec<u8>, !> {
        Ok(Vec::new())
    }

    async fn recycle(&self, buffer: &mut Vec<u8>) -> RecycleResult<!> {
        buffer.clear();
        Ok(())
    }
}

/// A convenience trait for indicating that a given thing can be serialized to XML
#[async_trait]
pub trait ToXml<E>
where
    E: StdError + Debug,
{
    /// Serializes the data structure into XML
    async fn to_xml<W>(&self, writer: &mut Writer<W>) -> ResultWithError<(), E>
    where
        W: Write + Send + Sync;
}

/// A convenience trait indicating that a given thing can be deserialized from XML
#[async_trait]
pub trait FromXml<E>: Sized
where
    E: StdError + Debug,
{
    /// Deserializes the data structure from XML
    async fn from_xml<R>(
        &mut self,
        reader: &mut Reader<R>,
        buffer_pool: BufferPool,
    ) -> ResultWithError<(), E>
    where
        R: Read + BufRead + Send + Sync;
}

/// A function that serializes the given data structure into a string using its [`ToXml`]
/// implementation
///
/// [`ToXml`]: ./trait.ToXml.html
pub async fn to_string<T, E>(value: &T) -> ResultWithError<String, E>
where
    T: ToXml<E>,
    E: StdError + Debug,
{
    let mut writer = Writer::new(Vec::new());
    value.to_xml(&mut writer).await?;
    Ok(String::from_utf8(writer.into_inner())?)
}

/// A function that deserializes the string into the given data structure using its [`FromXml`]
/// implementation
///
/// [`FromXml`]: ./trait.FromXml.html
pub async fn from_string<T, E>(
    value: Cow<'_, str>,
    buffer_pool: &mut BufferPool,
) -> ResultWithError<T, E>
where
    T: FromXml<E> + Default,
    E: StdError + Debug,
{
    let mut reader = Reader::from_str(&value);
    let mut result = T::default();
    result.from_xml(&mut reader, buffer_pool.clone()).await?;
    Ok(result)
}
