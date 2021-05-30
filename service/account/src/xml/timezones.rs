//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use async_trait::async_trait;
use chrono::offset::FixedOffset;
use iso::language::{Iso639_1, Language};
use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Reader, Writer,
};
use std::{
    borrow::Cow,
    io::{BufRead, Read, Write},
    str::FromStr,
};

use crate::xml::errors::{Error as XmlErrorExtension, Result};
use ralsei_util::xml::{
    errors::Error as XmlError,
    framework::{BufferPool, FromXml, ToXml},
    helpers::{generate_xml_field_write, generate_xml_struct_read, generate_xml_struct_read_check},
};

/// A representation of a Nintendo Network timezone document
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct Timezones<'a> {
    /// A vector of [`Timezone`] types
    ///
    /// [`Timezone`]: ./struct.Timezone.html
    pub timezones: Vec<Timezone<'a>>,
}

#[async_trait]
impl<'a> ToXml<XmlErrorExtension> for Timezones<'a> {
    async fn to_xml<W>(&self, writer: &mut Writer<W>) -> Result<()>
    where
        W: Write + Send + Sync,
    {
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"timezones")))?;

        for timezone in &self.timezones {
            timezone.to_xml(writer).await?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(b"timezones")))?;

        Ok(())
    }
}

#[async_trait]
impl<'a> FromXml<XmlErrorExtension> for Timezones<'a> {
    async fn from_xml<R>(&mut self, reader: &mut Reader<R>, buffer_pool: BufferPool) -> Result<()>
    where
        R: Read + BufRead + Send + Sync,
    {
        generate_xml_struct_read_check!(b"timezones", reader, buffer_pool.clone());

        generate_xml_struct_read!(
            b"timezones",
            reader, buffer_pool,
            c,
            b"timezone" => {
                let mut timezone = Timezone::default();
                timezone.from_xml(reader, buffer_pool.clone()).await?;
                self.timezones.push(timezone)
            }
        )
    }
}

/// A Nintendo Network account server timezone document
///
/// Contained within is the timezone's area name, the language used to localize the name of the
/// location the timezone is centered around, the name of the location mentioned prior, the
/// timezone's utc offset (in seconds,) and the intended location of the timezone in a list of
/// those returned by the endpoint that provides this xml document
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct Timezone<'a> {
    /// The name of the timezone (as used in zoneinfo)
    pub area: Option<Cow<'a, str>>, // i may be able to supplant this with a zoneinfo library
    // but i'd probably lose information doing that
    /// The language the timezone's name is in
    pub language: Option<Iso639_1>,

    /// The name of the location the timezone is centered around
    ///
    /// This is translated into the language specified by the [`language`] field
    ///
    /// [`language`]: #field.language
    pub name: Option<Cow<'a, str>>,

    /// The UTC offset of the timezone
    pub offset: Option<FixedOffset>,

    /// The intended location of the timezone in a list (one-indexed)
    pub order: Option<u8>, // i don't think there will ever be more than 255. if this ever
                           // happens, please let me know
}

#[async_trait]
impl<'a> ToXml<XmlErrorExtension> for Timezone<'a> {
    async fn to_xml<W>(&self, writer: &mut Writer<W>) -> Result<()>
    where
        W: Write + Send + Sync,
    {
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"timezone")))?;

        // the timezone's area name
        if let Some(ref area) = &self.area {
            generate_xml_field_write!(b"area", writer, BytesText::from_plain_str(area));
        }

        // the language the timezone name is represented in
        if let Some(ref language) = &self.language {
            generate_xml_field_write!(
                b"language",
                writer,
                BytesText::from_plain_str(language.code())
            );
        }

        // the proper name of the timezone
        if let Some(ref name) = &self.name {
            generate_xml_field_write!(b"name", writer, BytesText::from_plain_str(name));
        }

        // the utc offset of the timezone
        if let Some(ref offset) = &self.offset {
            generate_xml_field_write!(
                b"utc_offset",
                writer,
                BytesText::from_plain_str(&offset.local_minus_utc().to_string())
            );
        }

        // the intended location of the timezone in a list
        if let Some(ref order) = &self.order {
            generate_xml_field_write!(
                b"order",
                writer,
                BytesText::from_plain_str(&order.to_string())
            );
        }

        writer.write_event(Event::End(BytesEnd::borrowed(b"timezone")))?;

        Ok(())
    }
}

#[async_trait]
impl<'a> FromXml<XmlErrorExtension> for Timezone<'a> {
    async fn from_xml<R>(&mut self, reader: &mut Reader<R>, buffer_pool: BufferPool) -> Result<()>
    where
        R: Read + BufRead + Send + Sync,
    {
        generate_xml_struct_read!(
            b"timezone",
            reader, buffer_pool,
            c,

            // the timezone's area name
            b"area" => {
                self.area = Some(Cow::Owned(reader.read_text(c.name(), &mut *buffer_pool.get().await?)?));
            },

            // the language the timezone's name is in
            b"language" => {
                self.language = Some(Iso639_1::from_str(reader.read_text(c.name(), &mut *buffer_pool.get().await?)?.as_str()).map_err(|e| XmlError::CustomError(XmlErrorExtension::LanguageCodeParseError(e)))?);
            },

            // the timezone's name
            b"name" => {
                self.name = Some(Cow::Owned(reader.read_text(c.name(), &mut *buffer_pool.get().await?)?));
            },

            // the utc offset of the timezone
            b"utc_offset" => {
                let converted_offset = i32::from_str(reader.read_text(c.name(), &mut *buffer_pool.get().await?)?.as_str()).map_err(|e| XmlError::CustomError(XmlErrorExtension::IntegerParseError(e)))?;
                self.offset = Some(FixedOffset::east_opt(converted_offset).ok_or(XmlError::CustomError(XmlErrorExtension::UtcOffsetOutOfBounds(converted_offset)))?);
            },

            // the intended location of the timezone in a list
            b"order" => {
                self.order = Some(u8::from_str(reader.read_text(c.name(), &mut *buffer_pool.get().await?)?.as_str()).map_err(|e| XmlError::CustomError(XmlErrorExtension::IntegerParseError(e)))?);
            }
        )
    }
}
