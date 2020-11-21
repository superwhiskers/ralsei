//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use async_trait::async_trait;
use deadpool::managed::{Manager, Pool, RecycleResult};
use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Reader, Writer,
};
use std::{
    borrow::Cow,
    io::{BufRead, Read, Write},
    result::Result as StdResult,
    str,
};

use super::errors::{Error, FormattingError, Result};

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

/// A type alias for a [`Pool`] of [`Vec<u8>`]s
///
/// [`Pool`]: https://docs.rs/deadpool/0.6.0/deadpool/managed/struct.Pool.html
/// [`Vec<u8>`]: https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html
pub type BufferPool = Pool<Vec<u8>, !>;

/// A convenience trait for indicating that a given thing can be serialized to XML
#[async_trait]
pub trait ToXml {
    /// Serializes the data structure into XML
    async fn to_xml<W>(&self, writer: &mut Writer<W>) -> Result<()>
    where
        W: Write + Send + Sync;
}

/// A convenience trait indicating that a given thing can be deserialized from XML
#[async_trait]
pub trait FromXml: Sized {
    /// Deserializes the data structure from XML
    async fn from_xml<R>(&mut self, reader: &mut Reader<R>, buffer_pool: BufferPool) -> Result<()>
    where
        R: Read + BufRead + Send + Sync;
}

/// A function that serializes the given data structure into a string using its [`ToXml`]
/// implementation
///
/// [`ToXml`]: ./trait.ToXml.html
pub async fn to_string<T>(value: &T) -> Result<String>
where
    T: ToXml,
{
    let mut writer = Writer::new(Vec::new());
    value.to_xml(&mut writer).await?;
    Ok(String::from_utf8(writer.into_inner())?)
}

/// A function that deserializes the string into the given data structure using its [`FromXml`]
/// implementation
///
/// [`FromXml`]: ./trait.FromXml.html
pub async fn from_string<T>(value: Cow<'_, str>, buffer_pool: &mut BufferPool) -> Result<T>
where
    T: FromXml + Default,
{
    let mut reader = Reader::from_str(&value);
    let mut result = T::default();
    result.from_xml(&mut reader, buffer_pool.clone()).await?;
    Ok(result)
}

/// A helper macro used to make writing simple field writes easier
pub(crate) macro generate_xml_field_write($name:expr, $writer:ident, $bytes_text:expr) {
    $writer.write_event(Event::Start(BytesStart::borrowed_name($name)))?;
    $writer.write_event(Event::Text($bytes_text))?;
    $writer.write_event(Event::End(BytesEnd::borrowed($name)))?;
}

/// A helper macro to aid in calling a contained item's [`ToXml`] implementation, reducing
/// boilerplate
///
/// [`ToXml`]: ./trait.ToXml.html
pub(crate) macro generate_xml_field_write_by_propagation($name:expr, $writer:ident, $contained:expr) {
    $writer.write_event(Event::Start(BytesStart::borrowed_name($name)))?;
    $contained.to_xml($writer).await?;
    $writer.write_event(Event::End(BytesEnd::borrowed($name)))?;
}

/// A helper macro to aid in calling a contained item's [`FromXml`] implementation, reducing
/// boilerplate
///
/// [`FromXml`]: ./trait.FromXml.html
pub(crate) macro generate_xml_field_read_by_propagation($container:expr, $reader:ident, $buffer_pool:expr, $name:expr) {
    $container.from_xml($reader, $buffer_pool.clone()).await?;

    let mut buffer = $buffer_pool.get().await?;
    let event = $reader.read_event(&mut *buffer)?;
    if let Event::End(c) = event {
        if c.name() != $name {
            return Err(Error::Formatting(FormattingError::UnexpectedClosingTag(
                str::from_utf8(c.name())?.to_string(),
            )));
        }
    } else {
        return Err(Error::Formatting(FormattingError::UnexpectedEvent(
            format!("{:?}", event),
        )));
    }
}

/// A helper macro to aid in checking that the first XML element is correct for a structure
pub(crate) macro generate_xml_struct_read_check($name:expr, $reader:ident, $buffer_pool:expr) {
    let mut buffer = $buffer_pool.get().await?;
    loop {
        let event = $reader.read_event(&mut *buffer)?;
        if let Event::Start(c) = event {
            if c.name() != $name {
                return Err(Error::Formatting(FormattingError::UnexpectedOpeningTag(
                    str::from_utf8(c.name())?.to_string(),
                )));
            }
            break;
        } else if let Event::End(c) = event {
            return Err(Error::Formatting(FormattingError::UnexpectedClosingTag(
                str::from_utf8(c.name())?.to_string(),
            )));
        }
    }
}

/// A helper macro that simplifies writing [`FromXml`] implementations
///
/// [`FromXml`]: ./trait.FromXml.html
pub(crate) macro generate_xml_struct_read($name:expr, $reader:ident, $buffer_pool:expr, $content:ident, $($item:expr => $result:block),*) {
    {
        // loop over the rest of the events until they're all gone
        loop {
            match $reader.read_event(&mut *$buffer_pool.get().await?)? {
                Event::Start($content) => match $content.name() {
                    $($item => $result),*
                    n => {
                        return Err(Error::Formatting(FormattingError::UnexpectedOpeningTag(
                            str::from_utf8(n)?.to_string(),
                        )))
                    }
                }
                Event::End(c) => match c.name() {
                    $name => break,
                    n => {
                        return Err(Error::Formatting(FormattingError::UnexpectedClosingTag(
                            str::from_utf8(n)?.to_string(),
                        )))
                    }
                }
                Event::Text(_) => continue,
                e => {
                    return Err(Error::Formatting(FormattingError::UnexpectedEvent(
                        format!("{:?}", e),
                    )))
                }
            }
        }

        Ok(())
    }
}
