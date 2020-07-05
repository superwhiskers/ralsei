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
use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
    string::FromUtf8Error,
};
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
    /// The message that appears when a panic occurs while trying to convert a c-style enum to a
    /// u32
    const CSTYLE_ENUM_TO_U32_PANIC_MESSAGE: &'static str =
        "unable to convert a c-style enum to a u32 (this should be impossible)";

    /// The message that appears when a panic occurs while trying to convert a slice into an array
    const SLICE_TO_ARRAY_PANIC_MESSAGE: &'static str =
        "unable to convert a slice into an array (this should be impossible)";

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

    /// Converts a [`Certificate`] into a byte vector
    ///
    /// [`Certificate`]: ./struct.Certificate.html
    pub fn into_bytes(&self) -> Vec<u8> {
        [
            match &self.signature {
                Signature::Rsa4096WithSha1(signature) => [
                    &SignatureMagic::Rsa4096WithSha1
                        .to_u32()
                        .expect(Self::CSTYLE_ENUM_TO_U32_PANIC_MESSAGE)
                        .to_le_bytes(),
                    signature.as_ref(),
                    &[0; 0x3c],
                ]
                .concat(),
                Signature::Rsa2048WithSha1(signature) => [
                    &SignatureMagic::Rsa2048WithSha1
                        .to_u32()
                        .expect(Self::CSTYLE_ENUM_TO_U32_PANIC_MESSAGE)
                        .to_le_bytes(),
                    signature.as_ref(),
                    &[0; 0x3c],
                ]
                .concat(),
                Signature::EllipticCurveWithSha1(signature) => [
                    &SignatureMagic::EllipticCurveWithSha1
                        .to_u32()
                        .expect(Self::CSTYLE_ENUM_TO_U32_PANIC_MESSAGE)
                        .to_le_bytes(),
                    signature.as_ref(),
                    &[0; 0x40],
                ]
                .concat(),
                Signature::Rsa4096WithSha256(signature) => [
                    &SignatureMagic::Rsa4096WithSha256
                        .to_u32()
                        .expect(Self::CSTYLE_ENUM_TO_U32_PANIC_MESSAGE)
                        .to_le_bytes(),
                    signature.as_ref(),
                    &[0; 0x3c],
                ]
                .concat(),
                Signature::Rsa2048WithSha256(signature) => [
                    &SignatureMagic::Rsa2048WithSha256
                        .to_u32()
                        .expect(Self::CSTYLE_ENUM_TO_U32_PANIC_MESSAGE)
                        .to_le_bytes(),
                    signature.as_ref(),
                    &[0; 0x3c],
                ]
                .concat(),
                Signature::EcdsaWithSha256(signature) => [
                    &SignatureMagic::EcdsaWithSha256
                        .to_u32()
                        .expect(Self::CSTYLE_ENUM_TO_U32_PANIC_MESSAGE)
                        .to_le_bytes(),
                    signature.as_ref(),
                    &[0; 0x40],
                ]
                .concat(),
            },
            self.issuer.as_ref().into(),
            match &self.key {
                Key::Rsa4096(key) => [
                    &KeyMagic::Rsa4096
                        .to_u32()
                        .expect(Self::CSTYLE_ENUM_TO_U32_PANIC_MESSAGE)
                        .to_le_bytes(),
                    self.identity.as_ref(),
                    key.as_ref(),
                    &[0; 0x34],
                ]
                .concat(),
                Key::Rsa2048(key) => [
                    &KeyMagic::Rsa2048
                        .to_u32()
                        .expect(Self::CSTYLE_ENUM_TO_U32_PANIC_MESSAGE)
                        .to_le_bytes(),
                    self.identity.as_ref(),
                    key.as_ref(),
                    &[0; 0x34],
                ]
                .concat(),
                Key::EllipticCurve(key) => [
                    &KeyMagic::EllipticCurve
                        .to_u32()
                        .expect(Self::CSTYLE_ENUM_TO_U32_PANIC_MESSAGE)
                        .to_le_bytes(),
                    self.identity.as_ref(),
                    key.as_ref(),
                    &[0; 0x3c],
                ]
                .concat(),
            },
        ]
        .concat()
    }
}

impl TryFrom<&[u8]> for Certificate<'_> {
    type Error = CertificateParseError;

    /// Creates a new [`Certificate`] from a byte slice
    ///
    /// [`Certificate`]: ./struct.Certificate.html
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let signature_type = u32::from_le_bytes(
            value[..0x4]
                .try_into()
                .expect(Self::SLICE_TO_ARRAY_PANIC_MESSAGE),
        );

        let (signature, offset) = match SignatureMagic::from_u32(signature_type).ok_or(
            CertificateParseError::UnsupportedSignatureType(signature_type),
        )? {
            SignatureMagic::Rsa4096WithSha1 => (
                Signature::Rsa4096WithSha1(Cow::Owned(value[0x4..0x204].to_owned())),
                0x240,
            ),
            SignatureMagic::Rsa2048WithSha1 => (
                Signature::Rsa2048WithSha1(Cow::Owned(value[0x4..0x104].to_owned())),
                0x140,
            ),
            SignatureMagic::EllipticCurveWithSha1 => (
                Signature::EllipticCurveWithSha1(Cow::Owned(value[0x4..0x40].to_owned())),
                0x80,
            ),
            SignatureMagic::Rsa4096WithSha256 => (
                Signature::Rsa4096WithSha256(Cow::Owned(value[0x4..0x204].to_owned())),
                0x240,
            ),
            SignatureMagic::Rsa2048WithSha256 => (
                Signature::Rsa2048WithSha256(Cow::Owned(value[0x4..0x104].to_owned())),
                0x140,
            ),
            SignatureMagic::EcdsaWithSha256 => (
                Signature::EcdsaWithSha256(Cow::Owned(value[0x4..0x40].to_owned())),
                0x80,
            ),
        };
        let issuer = Cow::Owned(String::from_utf8(value[offset..offset + 0x40].to_owned())?);
        let key_type = u32::from_le_bytes(
            value[offset + 0x40..offset + 0x44]
                .try_into()
                .expect(Self::SLICE_TO_ARRAY_PANIC_MESSAGE),
        );
        let key = match KeyMagic::from_u32(key_type)
            .ok_or(CertificateParseError::UnsupportedKeyType(key_type))?
        {
            KeyMagic::Rsa4096 => {
                Key::Rsa4096(Cow::Owned(value[offset + 0x88..offset + 0x28c].to_owned()))
            }
            KeyMagic::Rsa2048 => {
                Key::Rsa2048(Cow::Owned(value[offset + 0x88..offset + 0x18c].to_owned()))
            }
            KeyMagic::EllipticCurve => {
                Key::EllipticCurve(Cow::Owned(value[offset + 0x88..offset + 0xc4].to_owned()))
            }
        };
        let identity = Cow::Owned(value[offset + 0x44..offset + 0x88].to_owned());

        Ok(Self {
            signature,
            issuer,
            key,
            identity,
        })
    }
}

/// A list of all possible errors encountered while parsing a [`Certificate`] from a byte slice
///
/// [`Certificate`]: ./struct.Certificate.html
#[derive(Error, Debug)]
pub enum CertificateParseError {
    #[error("The UTF-8 data inside of the Certificate is invalid")]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error("`{0}` is an unsupported signature type")]
    UnsupportedSignatureType(u32),

    #[error("`{0}` is an unsupported key type")]
    UnsupportedKeyType(u32),
}

/// An enumeration over the possible magic numbers representing a kind of [`Signature`]
///
/// [`Signature`]: ./enum.Signature.html
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum SignatureMagic {
    Rsa4096WithSha1 = 0x010000,
    Rsa2048WithSha1 = 0x010001,
    EllipticCurveWithSha1 = 0x010002,
    Rsa4096WithSha256 = 0x010003,
    Rsa2048WithSha256 = 0x010004,
    EcdsaWithSha256 = 0x010005,
}

/// An enumeration over all possible signature kinds, containing the internal signature data
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Signature<'a> {
    Rsa4096WithSha1(Cow<'a, [u8]>),
    Rsa2048WithSha1(Cow<'a, [u8]>),
    EllipticCurveWithSha1(Cow<'a, [u8]>),
    Rsa4096WithSha256(Cow<'a, [u8]>),
    Rsa2048WithSha256(Cow<'a, [u8]>),
    EcdsaWithSha256(Cow<'a, [u8]>),
}

/// An enumeration over all possible magic numbers representing a kind of [`Key`]
///
/// [`Key`]: ./enum.Key.html
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum KeyMagic {
    Rsa4096 = 0x0,
    Rsa2048 = 0x1,
    EllipticCurve = 0x2,
}

/// An enumeration over all possible key kinds, containing the internal key data
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Key<'a> {
    Rsa4096(Cow<'a, [u8]>),
    Rsa2048(Cow<'a, [u8]>),
    EllipticCurve(Cow<'a, [u8]>),
}
