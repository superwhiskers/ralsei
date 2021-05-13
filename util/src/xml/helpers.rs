//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

//! something

/// A helper macro used to make writing simple field writes easier
pub macro generate_xml_field_write($name:expr, $writer:ident, $bytes_text:expr) {{
    use quick_xml::events::{BytesEnd, BytesStart, Event};

    $writer.write_event(Event::Start(BytesStart::borrowed_name($name)))?;
    $writer.write_event(Event::Text($bytes_text))?;
    $writer.write_event(Event::End(BytesEnd::borrowed($name)))?;
}}

/// A helper macro used to make writing simple cdata writes easier
pub macro generate_xml_cdata_write($name:expr, $writer:ident, $bytes_text:expr) {{
    use quick_xml::events::{BytesEnd, BytesStart, Event};

    $writer.write_event(Event::Start(BytesStart::borrowed_name($name)))?;
    $writer.write_event(Event::CData($bytes_text))?;
    $writer.write_event(Event::End(BytesEnd::borrowed($name)))?;
}}

/// A helper macro to aid in calling a contained item's [`ToXml`] implementation, reducing
/// boilerplate
///
/// [`ToXml`]: ./trait.ToXml.html
pub macro generate_xml_field_write_by_propagation($name:expr, $writer:ident, $contained:expr) {{
    use quick_xml::events::{BytesEnd, BytesStart, Event};

    $writer.write_event(Event::Start(BytesStart::borrowed_name($name)))?;
    $contained.to_xml($writer).await?;
    $writer.write_event(Event::End(BytesEnd::borrowed($name)))?;
}}

/// A helper macro to aid in calling a contained item's [`FromXml`] implementation, reducing
/// boilerplate
///
/// [`FromXml`]: ./trait.FromXml.html
pub macro generate_xml_field_read_by_propagation($container:expr, $reader:ident, $buffer_pool:expr, $name:expr) {{
    use quick_xml::events::Event;
    use std::str;

    use $crate::xml::errors::{Error, FormattingError};

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
}}

/// A helper macro to aid in reading a field that contains its data in CDATA tags instead of simply leaving it as text
pub macro generate_xml_cdata_field_read($reader:ident, $content:ident, $buffer_pool:expr, $name:expr, $result:block) {{
    use quick_xml::events::Event;
    use std::str;

    use $crate::xml::errors::{Error, FormattingError};

    let mut buffer = $buffer_pool.get().await?;

    loop {
        match $reader.read_event(&mut *buffer)? {
            Event::CData(c) => {
                let unescaped = &c.unescaped()?;
                let $content = $reader.decode(unescaped)?;
                $result
            }
            Event::End(c) => match c.name() {
                $name => break,
                n => {
                    return Err(Error::Formatting(FormattingError::UnexpectedClosingTag(
                        str::from_utf8(n)?.to_string(),
                    )))
                }
            },

            // this is necessary to skip any extra text events that may happen before reading the
            // xml body
            Event::Text(_) => continue,

            e => {
                return Err(Error::Formatting(FormattingError::UnexpectedEvent(
                    format!("{:?}", e),
                )));
            }
        }
    }
}}

/// A helper macro to aid in checking that the first XML element is correct for a structure
pub macro generate_xml_struct_read_check($name:expr, $reader:ident, $buffer_pool:expr) {{
    use quick_xml::events::Event;
    use std::str;

    use $crate::xml::errors::{Error, FormattingError};

    let mut buffer = $buffer_pool.get().await?;

    loop {
        match $reader.read_event(&mut *buffer)? {
            Event::Start(c) if c.name() == $name => break,
            Event::Start(c) => {
                return Err(Error::Formatting(FormattingError::UnexpectedOpeningTag(
                    str::from_utf8(c.name())?.to_string(),
                )))
            }
            Event::End(c) => {
                return Err(Error::Formatting(FormattingError::UnexpectedClosingTag(
                    str::from_utf8(c.name())?.to_string(),
                )))
            }

            // this probably isn't necessary for xml documents coming from Nintendo themselves, but
            // just in case
            Event::Decl(_) => continue,
            Event::Comment(_) => continue,

            // this is necessary to skip any extra text events that may happen before reading the
            // xml body
            Event::Text(_) => continue,

            e => {
                return Err(Error::Formatting(FormattingError::UnexpectedEvent(
                    format!("{:?}", e),
                )))
            }
        }
    }
}}

/// A helper macro that simplifies writing [`FromXml`] implementations
///
/// [`FromXml`]: ./trait.FromXml.html
pub macro generate_xml_struct_read($name:expr, $reader:ident, $buffer_pool:expr, $content:ident, $($item:expr => $result:block),*) {{
    use quick_xml::events::Event;
    use std::str;

    use $crate::xml::errors::{Error, FormattingError};

    {
        // loop over the rest of the events until they're all gone
        loop {
            match $reader.read_event(&mut *$buffer_pool.get().await?)? {
                Event::Start($content) => match $content.name() {
                    $($item => $result),*
                    n => {
                        // skip unexpected fields
                        $reader.read_text($content.name(), &mut *$buffer_pool.get().await?)?;
                    },
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
}}
