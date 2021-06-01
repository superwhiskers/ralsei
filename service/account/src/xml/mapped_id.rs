//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use async_trait::async_trait;
use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Reader, Writer,
};
use std::{
    borrow::Cow,
    io::{BufRead, Read, Write},
};

use crate::xml::errors::{Error as XmlErrorExtension, Result};
use ralsei_util::xml::{
    errors::Error as XmlError,
    framework::{BufferPool, FromXml, ToXml},
    helpers::{generate_xml_field_write, generate_xml_struct_read, generate_xml_struct_read_check},
};

/// A representation of a Nintendo Network id mapping document
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct MappedIds<'a> {
    /// A vector of [`MappedId`] types
    pub mapped_ids: Vec<MappedId<'a>>,
}

#[async_trait]
impl<'a> ToXml<XmlErrorExtension> for MappedIds<'a> {
    async fn to_xml<W>(&self, writer: &mut Writer<W>) -> Result<()>
    where
        W: Write + Send + Sync,
    {
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"mapped_ids")))?;

        for mapped_id in &self.mapped_ids {
            mapped_id.to_xml(writer).await?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(b"mapped_ids")))?;

        Ok(())
    }
}

#[async_trait]
impl<'a> FromXml<XmlErrorExtension> for MappedIds<'a> {
    async fn from_xml<R>(&mut self, reader: &mut Reader<R>, buffer_pool: BufferPool) -> Result<()>
    where
        R: Read + BufRead + Send + Sync,
    {
        generate_xml_struct_read_check!(b"mapped_ids", reader, buffer_pool.clone());

        generate_xml_struct_read!(
            b"mapped_ids",
            reader, buffer_pool,
            c,
            b"mapped_id" => {
                let mut mapped_id = MappedId::default();
                mapped_id.from_xml(reader, buffer_pool.clone()).await?;
                self.mapped_ids.push(mapped_id)
            }
        )
    }
}

/// A Nintendo Network user identifier mapping
///
/// Because of the lack of information provided during the deserialization phase, both of the
/// fields are [`Cow<'a, str>`](Cow)s instead of being specialized types
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct MappedId<'a> {
    /// The identifier being mapped
    pub input: Option<Cow<'a, str>>,

    /// The resulting identifier
    pub output: Option<Cow<'a, str>>,
}

#[async_trait]
impl<'a> ToXml<XmlErrorExtension> for MappedId<'a> {
    async fn to_xml<W>(&self, writer: &mut Writer<W>) -> Result<()>
    where
        W: Write + Send + Sync,
    {
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"mapped_id")))?;

        // the identifier being mapped
        if let Some(ref input) = &self.input {
            generate_xml_field_write!(b"in_id", writer, BytesText::from_plain_str(input));
        }

        // the resulting identifier
        if let Some(ref output) = &self.output {
            generate_xml_field_write!(b"out_id", writer, BytesText::from_plain_str(input));
        } else {
            writer.write_event(Event::Empty(BytesStart::borrowed_name(b"out_id")))?;
        }

        writer.write_event(Event::Start(BytesStart::borrowed_name(b"mapped_id")))?;

        Ok(())
    }
}

#[async_trait]
impl<'a> FromXml<XmlErrorExtension> for MappedId<'a> {
    async fn from_xml<R>(&mut self, reader: &mut Reader<R>, buffer_pool: BufferPool) -> Result<()>
    where
        R: Read + BufRead + Send + Sync,
    {
        generate_xml_struct_read!(
            b"mapped_id",
            reader, buffer_pool,
            c,

            // the identifier being mapped
            b"in_id" => {
                self.input = Some(reader.read_text(c.name(), &mut *buffer_pool.get().await?)?);
            },

            // the resulting identifier
            b"out_id" => {
                self.output = Some(reader.read_text(c.name(), &mut *buffer_pool.get().await?)?);
            }
        )
    }
}
