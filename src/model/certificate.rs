//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Type definitions related to a general Nintendo certificate container format
//!
//! This certificate format is specific to Nintendo consoles and does not appear to match standards
//! such as ASN.1, creating a need for a standalone implementation of the format.

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::cast::{FromPrimitive, ToPrimitive};
use std::{borrow::Cow, convert::TryInto, string::FromUtf8Error};
use thiserror::Error;

/// A Nintendo certificate container
///
/// There is absolutely a better way to do this. This current method of containerization is
/// absolute trash and should not be used anywhere.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Certificate<'a> {
    pub signature: Signature<'a>,
    pub issuer: Cow<'a, str>,
    pub key: Key<'a>,
    pub identity: Cow<'a, [u8]>,
}

// TODO(superwhiskers): above, we should add in a wrapper struct for specific usages of this data,
// e.g. ctcert usage

impl<'a> Certificate<'a> {
    /// Creates a new [`Certificate`] from its portions
    ///
    /// [`Certificate`]: ./struct.Certificate.html
    pub const fn new(
        signature: Signature<'a>,
        issuer: Cow<'a, str>,
        key: Key<'a>,
        identity: Cow<'a, [u8]>,
    ) -> Self {
        Self {
            signature,
            issuer,
            key,
            identity,
        }
    }

    /// Creates a new [`Certificate`] from a byte slice
    ///
    /// [`Certificate`]: ./struct.Certificate.html
    pub fn from_bytes(source: &[u8]) -> Result<Self, CertificateParseError> {
        let signature_type = u32::from_le_bytes(
            source[..0x4]
                .try_into()
                .expect("unable to convert a slice into an array (this should be impossible)"),
        );

        let (signature, offset) = match SignatureMagic::from_u32(signature_type).ok_or(
            CertificateParseError::UnsupportedSignatureType(signature_type),
        )? {
            SignatureMagic::Rsa4096WithSha1 => (
                Signature::Rsa4096WithSha1(Cow::Owned(source[0x4..0x204].to_owned())),
                0x240,
            ),
            SignatureMagic::Rsa2048WithSha1 => (
                Signature::Rsa2048WithSha1(Cow::Owned(source[0x4..0x104].to_owned())),
                0x140,
            ),
            SignatureMagic::EllipticCurveWithSha1 => (
                Signature::EllipticCurveWithSha1(Cow::Owned(source[0x4..0x40].to_owned())),
                0x80,
            ),
            SignatureMagic::Rsa4096WithSha256 => (
                Signature::Rsa4096WithSha256(Cow::Owned(source[0x4..0x204].to_owned())),
                0x240,
            ),
            SignatureMagic::Rsa2048WithSha256 => (
                Signature::Rsa2048WithSha256(Cow::Owned(source[0x4..0x104].to_owned())),
                0x140,
            ),
            SignatureMagic::EcdsaWithSha256 => (
                Signature::EcdsaWithSha256(Cow::Owned(source[0x4..0x40].to_owned())),
                0x80,
            ),
        };
        let issuer = Cow::Owned(String::from_utf8(source[offset..offset + 0x40].to_owned())?);
        let key_type = u32::from_le_bytes(
            source[offset + 0x40..offset + 0x44]
                .try_into()
                .expect("unable to convert a slice into an array (this should be impossible)"),
        );
        let key = match KeyMagic::from_u32(key_type)
            .ok_or(CertificateParseError::UnsupportedKeyType(key_type))?
        {
            KeyMagic::Rsa4096 => {
                Key::Rsa4096(Cow::Owned(source[offset + 0x88..offset + 0x28c].to_owned()))
            }
            KeyMagic::Rsa2048 => {
                Key::Rsa2048(Cow::Owned(source[offset + 0x88..offset + 0x18c].to_owned()))
            }
            KeyMagic::EllipticCurve => {
                Key::EllipticCurve(Cow::Owned(source[offset + 0x88..offset + 0xc4].to_owned()))
            }
        };
        let identity = Cow::Owned(source[offset + 0x44..offset + 0x88].to_owned());

        Ok(Self {
            signature,
            issuer,
            key,
            identity,
        })
    }
}

#[derive(Error, Debug)]
pub enum CertificateParseError {
    #[error("The UTF-8 data inside of the Certificate is invalid")]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error("`{0}` is an unsupported signature type")]
    UnsupportedSignatureType(u32),

    #[error("`{0}` is an unsupported key type")]
    UnsupportedKeyType(u32),
}

#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum SignatureMagic {
    Rsa4096WithSha1 = 0x010000,
    Rsa2048WithSha1 = 0x010001,
    EllipticCurveWithSha1 = 0x010002,
    Rsa4096WithSha256 = 0x010003,
    Rsa2048WithSha256 = 0x010004,
    EcdsaWithSha256 = 0x010005,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Signature<'a> {
    Rsa4096WithSha1(Cow<'a, [u8]>),
    Rsa2048WithSha1(Cow<'a, [u8]>),
    EllipticCurveWithSha1(Cow<'a, [u8]>),
    Rsa4096WithSha256(Cow<'a, [u8]>),
    Rsa2048WithSha256(Cow<'a, [u8]>),
    EcdsaWithSha256(Cow<'a, [u8]>),
}

#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum KeyMagic {
    Rsa4096 = 0x0,
    Rsa2048 = 0x1,
    EllipticCurve = 0x2,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Key<'a> {
    Rsa4096(Cow<'a, [u8]>),
    Rsa2048(Cow<'a, [u8]>),
    EllipticCurve(Cow<'a, [u8]>),
}
