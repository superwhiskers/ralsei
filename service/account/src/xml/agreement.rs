//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use async_trait::async_trait;
use chrono::{
    offset::{TimeZone, Utc},
    DateTime,
};
use iso::language::{Iso639_1, Language};
use isocountry::CountryCode;
use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Reader, Writer,
};
use std::{
    borrow::Cow,
    fmt,
    io::{BufRead, Read, Write},
    str::FromStr,
};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

use crate::xml::errors::{Error as XmlErrorExtension, Result};
use ralsei_util::xml::{
    errors::{Error as XmlError, FormattingError},
    framework::{BufferPool, FromXml, ToXml},
    helpers::{
        generate_xml_field_write, generate_xml_field_write_by_propagation,
        generate_xml_field_read_by_propagation,
        generate_xml_cdata_write, generate_xml_cdata_field_read,
        generate_xml_struct_read, generate_xml_struct_read_check,
    },
};

//TODO(superwhiskers): hook this into the custom serialization/deserialization infrastructure

//TODO(superwhiskers): replace usage of the isocountry crate with my iso crate

/// A representation of a Nintendo Network EULA document
#[derive(Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct Agreements<'a> {
    /// A vector of [`Agreement`] types
    ///
    /// [`Agreement`]: ./struct.Agreement.html
    pub agreements: Vec<Agreement<'a>>,
}

impl<'a> Agreements<'a> {
    /// Returns the first [`Agreement`] or `None` if there are none
    ///
    /// [`Agreement`]: ./struct.Agreement.html
    pub fn first(&self) -> Option<&Agreement> {
        self.agreements.first()
    }

    /// Returns the first [`Agreement`]'s [`AgreementKind`] or `None` if there are none
    ///
    /// [`Agreement`]: ./struct.Agreement.html
    /// [`AgreementKind`]: ./enum.AgreementKind.html
    pub fn first_kind(&self) -> Option<&AgreementKind> {
        self.agreements.first().map(|v| &v.kind)
    }
}

#[async_trait]
impl<'a> ToXml<XmlErrorExtension> for Agreements<'a> {
    async fn to_xml<W>(&self, writer: &mut Writer<W>) -> Result<()>
    where
        W: Write + Send + Sync,
    {
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"agreements")))?;

        for agreement in &self.agreements {
            agreement.to_xml(writer).await?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(b"agreements")))?;

        Ok(())
    }
}

#[async_trait]
impl<'a> FromXml<XmlErrorExtension> for Agreements<'a> {
    async fn from_xml<R>(&mut self, reader: &mut Reader<R>, buffer_pool: BufferPool) -> Result<()>
    where
        R: Read + BufRead + Send + Sync,
    {
        generate_xml_struct_read_check!(b"agreements", reader, buffer_pool.clone());

        generate_xml_struct_read!(
            b"agreements",
            reader, buffer_pool,
            c,
            b"agreement" => {
                let mut agreement = Agreement::default();
                agreement.from_xml(reader, buffer_pool.clone()).await?;
                self.agreements.push(agreement)
            }
        )
    }
}

/// A Nintendo Network account server agreement
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct Agreement<'a> {
    /// The country code representing the country the agreement is intended for in the iso 3166-1
    /// alpha-2 format
    pub country: Option<CountryCode>,

    /// The language code representing the language the agreement is written it, within the iso
    /// 639-1 language code format
    pub language: Option<Iso639_1>,

    /// The date at which this specific agreement was published, formatted as specified by iso 8601
    pub publish_date: Option<DateTime<Utc>>,

    /// The text to be displayed on the `accept` button
    pub accept_text: Option<Cow<'a, str>>,

    /// The text to be displayed on the `cancel` button
    pub cancel_text: Option<Cow<'a, str>>,

    /// The title of the agreement
    pub title_text: Option<Cow<'a, str>>,

    /// The body of the agreement
    pub body_text: Option<Cow<'a, str>>,

    /// The kind of agreement
    pub kind: AgreementKind<'a>,

    /// The agreement's version
    pub version: Option<u16>,
}

impl<'a> Agreement<'a> {
    //TODO(superwhiskers): placeholder section b/c this may be necessary later
}

#[async_trait]
impl<'a> ToXml<XmlErrorExtension> for Agreement<'a> {
    async fn to_xml<W>(&self, writer: &mut Writer<W>) -> Result<()>
    where
        W: Write + Send + Sync,
    {
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"agreement")))?;

        // the agreement's intended country
        if let Some(ref country) = &self.country {
            generate_xml_field_write!(
                b"country",
                writer,
                BytesText::from_plain_str(country.alpha2())
            );
        }

        // the agreement's language
        if let Some(ref language) = &self.language {
            generate_xml_field_write!(
                b"language",
                writer,
                BytesText::from_plain_str(language.code())
            );
            generate_xml_field_write!(
                b"language_name",
                writer,
                BytesText::from_plain_str(language.name())
            );
        }

        // the agreement's publish date
        if let Some(ref publish_date) = &self.publish_date {
            let publish_date_str = publish_date.format("%Y-%m-%dT%H:%M:%S").to_string();
            generate_xml_field_write!(
                b"publish_date",
                writer,
                BytesText::from_plain_str(&publish_date_str)
            );
        }

        // the texts section of the agreement
        if self.accept_text.is_some()
            || self.cancel_text.is_some()
            || self.title_text.is_some()
            || self.body_text.is_some()
        {
            writer.write_event(Event::Start(BytesStart::borrowed_name(b"texts")))?;

            // the accept button text
            if let Some(ref accept_text) = &self.accept_text {
                generate_xml_cdata_write!(
                    b"agree_text",
                    writer,
                    BytesText::from_plain_str(accept_text)
                );
            }

            // the cancel button text
            if let Some(ref cancel_text) = &self.cancel_text {
                generate_xml_cdata_write!(
                    b"non_agree_text",
                    writer,
                    BytesText::from_plain_str(cancel_text)
                );
            }

            // the agreement's title
            if let Some(ref title_text) = &self.title_text {
                generate_xml_cdata_write!(
                    b"main_title",
                    writer,
                    BytesText::from_plain_str(title_text)
                );
            }

            // the body of the agreement
            if let Some(ref body_text) = &self.body_text {
                generate_xml_cdata_write!(
                    b"main_text",
                    writer,
                    BytesText::from_plain_str(body_text)
                );
            }

            writer.write_event(Event::End(BytesEnd::borrowed(b"texts")))?;
        }

        // the agreement's kind
        generate_xml_field_write_by_propagation!(b"type", writer, self.kind);

        // the agreement's version
        if let Some(ref version) = &self.version {
            generate_xml_field_write!(
                b"version",
                writer,
                BytesText::from_escaped_str(Cow::Owned(format!("{:0>4}", version)))
            );
        }

        writer.write_event(Event::End(BytesEnd::borrowed(b"agreement")))?;

        Ok(())
    }
}

#[async_trait]
impl<'a> FromXml<XmlErrorExtension> for Agreement<'a> {
    async fn from_xml<R>(&mut self, reader: &mut Reader<R>, buffer_pool: BufferPool) -> Result<()>
    where
        R: Read + BufRead + Send + Sync,
    {
        generate_xml_struct_read!(
            b"agreement",
            reader, buffer_pool,
            c,

            // the agreement's intended country
            b"country" => {
                self.country = Some(CountryCode::for_alpha2(&reader.read_text(c.name(), &mut *buffer_pool.get().await?)?).map_err(|e| XmlError::CustomError(XmlErrorExtension::CountryCodeParseError(e)))?);
            },

            // the agreement's language
            b"language" => {
                self.language = Some(Iso639_1::from_str(reader.read_text(c.name(), &mut *buffer_pool.get().await?)?.as_str()).map_err(|e| XmlError::CustomError(XmlErrorExtension::LanguageCodeParseError(e)))?);
            },

            // the agreement's publish date
            b"publish_date" => {
                self.publish_date = Some(Utc.datetime_from_str(reader.read_text(c.name(), &mut *buffer_pool.get().await?)?.as_str(), "%Y-%m-%dT%H:%M:%S").map_err(|e| XmlError::CustomError(XmlErrorExtension::DateTimeParseError(e)))?);
            },

            // the agreement's texts
            b"texts" => {
                let _: Result<()> = generate_xml_struct_read!(
                    b"texts",
                    reader, buffer_pool,
                    c,

                    // the accept button text
                    b"agree_text" => {
                        generate_xml_cdata_field_read!(reader, c, buffer_pool, b"agree_text", {
                            self.accept_text = Some(Cow::Owned(c.to_string()));
                        });
                    },

                    // the cancel button text
                    b"non_agree_text" => {
                        generate_xml_cdata_field_read!(reader, c, buffer_pool, b"non_agree_text", {
                            self.cancel_text = Some(Cow::Owned(c.to_string()));
                        });
                    },

                    // the agreement's title
                    b"main_title" => {
                        generate_xml_cdata_field_read!(reader, c, buffer_pool, b"main_title", {
                            self.title_text = Some(Cow::Owned(c.to_string()));
                        });
                    },

                    // the body of the agreement
                    b"main_text" => {
                        generate_xml_cdata_field_read!(reader, c, buffer_pool, b"main_text", {
                            self.body_text = Some(Cow::Owned(c.to_string()));
                        });
                    }
                );
            },

            // the agreement's kind
            b"type" => {
                generate_xml_field_read_by_propagation!(self.kind, reader, buffer_pool, b"type");
            },

            // the agreement's version
            b"version" => {
                self.version = Some(u16::from_str(reader.read_text(c.name(), &mut *buffer_pool.get().await?)?.as_str()).map_err(|e| XmlError::CustomError(XmlErrorExtension::IntegerParseError(e)))?);
            }
        )
    }
}

/// A container for a Nintendo Network account server agreement kind, handling unknown kinds as
/// well as known ones
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum AgreementKind<'a> {
    Known(AgreementKindValue),

    Unknown(Cow<'a, str>),
}

impl<'a> AgreementKind<'a> {
    /// Returns the [`AgreementKind`] represented by the provided [`Cow<'a, str>`]
    ///
    /// [`AgreementKind`]: ./enum.AgreementKind.html
    /// [`Cow<'a, str>`]: https://doc.rust-lang.org/nightly/std/borrow/enum.Cow.html
    pub fn from_cow(value: Cow<'a, str>) -> Self {
        match AgreementKindValue::from_str(&value) {
            Ok(kind) => Self::Known(kind),
            Err(_) => Self::Unknown(value),
        }
    }

    /// Returns the [`AgreementKind`] represented as a [`&str`]
    ///
    /// [`AgreementKind`]: ./enum.AgreementKind.html
    /// [`&str`]: https://doc.rust-lang.org/nightly/std/primitive.str.html
    pub fn to_str(&self) -> &str {
        match &self {
            Self::Known(value) => value.as_ref(),
            Self::Unknown(value) => &value,
        }
    }
}

impl<'a> fmt::Display for AgreementKind<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.to_str())
    }
}

#[async_trait]
impl<'a> ToXml<XmlErrorExtension> for AgreementKind<'a> {
    async fn to_xml<W>(&self, writer: &mut Writer<W>) -> Result<()>
    where
        W: Write + Send + Sync,
    {
        writer
            .write_event(Event::Text(match &self {
                Self::Known(value) => BytesText::from_escaped_str(Cow::Borrowed(value.as_ref())),
                Self::Unknown(value) => BytesText::from_plain_str(value),
            }))
            .map_err(XmlError::from)
    }
}

#[async_trait]
impl<'a> FromXml<XmlErrorExtension> for AgreementKind<'a> {
    async fn from_xml<R>(&mut self, reader: &mut Reader<R>, buffer_pool: BufferPool) -> Result<()>
    where
        R: Read + BufRead + Send + Sync,
    {
        //TODO(superwhiskers): consider looking at this again. i'm not sure if it's optimal
        match reader.read_event(&mut *buffer_pool.get().await?)? {
            Event::Text(c) => {
                *self = AgreementKind::from_cow(Cow::Owned(
                    reader.decode(&c.unescaped()?)?.to_string(),
                ));
                Ok(())
            }
            e => Err(XmlError::Formatting(FormattingError::UnexpectedEvent(
                format!("{:?}", e),
            ))),
        }
    }
}

impl<'a> Default for AgreementKind<'a> {
    fn default() -> Self {
        Self::Unknown(Cow::Borrowed(""))
    }
}

/// An enumeration over possible [`Agreement`] kinds
///
/// [`Agreement`]: ./struct.Agreement.html
#[non_exhaustive]
#[derive(
    IntoStaticStr, AsRefStr, EnumString, Display, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord,
)]
pub enum AgreementKindValue {
    #[strum(to_string = "NINTENDO-NETWORK-EULA")]
    Eula,
    //TODO(superwhiskers): figure out all possible agreements, if there even are more
}
