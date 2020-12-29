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
    str::FromStr,
    string::FromUtf8Error,
};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
use thiserror::Error;

use crate::model::console::common::Kind as ConsoleKind;

/// A Nintendo certificate container
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Certificate<'a> {
    pub signature: Signature<'a>,
    pub issuer: Issuer<'a>,
    pub key: Key<'a>,
    pub name: Name<'a>,
    pub key_id: KeyId,
}

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
        issuer: Issuer<'a>,
        key: Key<'a>,
        name: Name<'a>,
        key_id: KeyId,
    ) -> Self {
        Self {
            signature,
            issuer,
            key,
            name,
            key_id,
        }
    }

    /// Converts a [`Certificate`] into a byte vector
    ///
    /// [`Certificate`]: ./struct.Certificate.html
    pub fn to_bytes(&self) -> Result<Vec<u8>, CertificateError> {
        let mut certificate = Vec::new();

        macro signature_match_clause($signature_kind:ident, $signature_data:ident, $padding_size:literal) {{
            certificate.extend(
                &SignatureMagic::$signature_kind
                    .to_u32()
                    .expect(Self::CSTYLE_ENUM_TO_U32_PANIC_MESSAGE)
                    .to_be_bytes(),
            );
            certificate.extend($signature_data.as_ref());
            certificate.extend([0; $padding_size].as_ref());
        }}

        match &self.signature {
            Signature::Rsa4096WithSha1(signature) => {
                signature_match_clause!(Rsa4096WithSha1, signature, 0x3c)
            }
            Signature::Rsa2048WithSha1(signature) => {
                signature_match_clause!(Rsa2048WithSha1, signature, 0x3c)
            }
            Signature::EllipticCurveWithSha1(signature) => {
                signature_match_clause!(EllipticCurveWithSha1, signature, 0x40)
            }
            Signature::Rsa4096WithSha256(signature) => {
                signature_match_clause!(Rsa4096WithSha256, signature, 0x3c)
            }
            Signature::Rsa2048WithSha256(signature) => {
                signature_match_clause!(Rsa2048WithSha256, signature, 0x3c)
            }
            Signature::EcdsaWithSha256(signature) => {
                signature_match_clause!(EcdsaWithSha256, signature, 0x40)
            }
        }

        {
            let len = certificate.len();
            certificate.extend(self.issuer.0.as_ref().bytes());
            certificate.resize(len + 0x40, 0);
        }

        macro key_match_clause($key_kind:ident, $key_data:ident, $padding_size:literal) {{
            certificate.extend(
                &KeyMagic::$key_kind
                    .to_u32()
                    .expect(Self::CSTYLE_ENUM_TO_U32_PANIC_MESSAGE)
                    .to_be_bytes(),
            );
            {
                let len = certificate.len();
                certificate.extend(self.name.0.as_ref().bytes());
                certificate.resize(len + 0x40, 0);
            }
            certificate.extend(&self.key_id.0.to_be_bytes());
            certificate.extend($key_data.as_ref());
            certificate.extend([0; $padding_size].as_ref());
        }};

        match &self.key {
            Key::Rsa4096(key) => key_match_clause!(Rsa4096, key, 0x34),
            Key::Rsa2048(key) => key_match_clause!(Rsa2048, key, 0x34),
            Key::EllipticCurve(key) => key_match_clause!(EllipticCurve, key, 0x3c),
        }

        Ok(certificate)
    }
}

impl TryFrom<&[u8]> for Certificate<'_> {
    type Error = CertificateError;

    /// Creates a new [`Certificate`] from a byte slice
    ///
    /// [`Certificate`]: ./struct.Certificate.html
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let signature_type = u32::from_be_bytes(
            value
                .get(..0x4)
                .ok_or(CertificateError::OutOfBounds)?
                .try_into()
                .expect(Self::SLICE_TO_ARRAY_PANIC_MESSAGE),
        );

        macro signature_magic_match_clause($signature_kind:ident, $signature_limit:literal, $padding_end:literal) {
            (
                Signature::$signature_kind(Cow::Owned(
                    value
                        .get(0x4..$signature_limit)
                        .ok_or(CertificateError::OutOfBounds)?
                        .to_owned(),
                )),
                $padding_end,
            )
        }

        let (signature, offset) = match SignatureMagic::from_u32(signature_type)
            .ok_or(CertificateError::UnsupportedSignatureType(signature_type))?
        {
            SignatureMagic::Rsa4096WithSha1 => {
                signature_magic_match_clause!(Rsa4096WithSha1, 0x204, 0x240)
            }
            SignatureMagic::Rsa2048WithSha1 => {
                signature_magic_match_clause!(Rsa2048WithSha1, 0x104, 0x140)
            }
            SignatureMagic::EllipticCurveWithSha1 => {
                signature_magic_match_clause!(EllipticCurveWithSha1, 0x40, 0x80)
            }
            SignatureMagic::Rsa4096WithSha256 => {
                signature_magic_match_clause!(Rsa4096WithSha256, 0x204, 0x240)
            }
            SignatureMagic::Rsa2048WithSha256 => {
                signature_magic_match_clause!(Rsa2048WithSha256, 0x104, 0x140)
            }
            SignatureMagic::EcdsaWithSha256 => {
                signature_magic_match_clause!(EcdsaWithSha256, 0x40, 0x80)
            }
        };

        let mut issuer = value
            .get(offset..offset + 0x40)
            .ok_or(CertificateError::OutOfBounds)?
            .to_owned();
        while let Some(&value) = issuer.last() {
            if value == 0 {
                issuer.pop();
            } else {
                break;
            }
        }
        let issuer = Issuer(Cow::Owned(String::from_utf8(issuer)?));

        let key_type = u32::from_be_bytes(
            value
                .get(offset + 0x40..offset + 0x44)
                .ok_or(CertificateError::OutOfBounds)?
                .try_into()
                .expect(Self::SLICE_TO_ARRAY_PANIC_MESSAGE),
        );

        macro key_magic_match_clause($key_kind:ident, $key_limit:literal) {
            Key::$key_kind(Cow::Owned(
                value
                    .get(offset + 0x88..offset + $key_limit)
                    .ok_or(CertificateError::OutOfBounds)?
                    .to_owned(),
            ))
        }

        let key = match KeyMagic::from_u32(key_type)
            .ok_or(CertificateError::UnsupportedKeyType(key_type))?
        {
            KeyMagic::Rsa4096 => key_magic_match_clause!(Rsa4096, 0x28c),
            KeyMagic::Rsa2048 => key_magic_match_clause!(Rsa2048, 0x18c),
            KeyMagic::EllipticCurve => key_magic_match_clause!(EllipticCurve, 0xc4),
        };

        let mut name = value
            .get(offset + 0x44..offset + 0x84)
            .ok_or(CertificateError::OutOfBounds)?
            .to_owned();
        while let Some(&value) = name.last() {
            if value == 0 {
                name.pop();
            } else {
                break;
            }
        }
        let name = Name(Cow::Owned(String::from_utf8(name)?));

        let key_id = KeyId(u32::from_be_bytes(
            value
                .get(offset + 0x84..offset + 0x88)
                .ok_or(CertificateError::OutOfBounds)?
                .try_into()
                .expect(Self::SLICE_TO_ARRAY_PANIC_MESSAGE),
        ));

        Ok(Self {
            signature,
            issuer,
            key,
            name,
            key_id,
        })
    }
}

/// A list of all possible errors encountered while working with a [`Certificate`]
///
/// [`Certificate`]: ./struct.Certificate.html
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum CertificateError {
    #[error("The UTF-8 data inside of the Certificate is invalid")]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error("`{0}` is an unsupported signature type")]
    UnsupportedSignatureType(u32),

    #[error("`{0}` is an unsupported key type")]
    UnsupportedKeyType(u32),

    #[error("The provided byte certificate is not large enough")]
    OutOfBounds,
}

/// An enumeration over the possible magic numbers representing a kind of [`Signature`]
///
/// [`Signature`]: ./enum.Signature.html
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Signature<'a> {
    Rsa4096WithSha1(Cow<'a, [u8]>),
    Rsa2048WithSha1(Cow<'a, [u8]>),
    EllipticCurveWithSha1(Cow<'a, [u8]>),
    Rsa4096WithSha256(Cow<'a, [u8]>),
    Rsa2048WithSha256(Cow<'a, [u8]>),
    EcdsaWithSha256(Cow<'a, [u8]>),
}

impl Signature<'_> {
    /// Converts a [`Signature`] into a [`SignatureMagic`]
    ///
    /// [`Signature`]: ./enum.Signature.html
    /// [`SignatureMagic`]: ./enum.SignatureMagic.html
    pub fn to_signature_magic(&self) -> Option<SignatureMagic> {
        match &self {
            Self::Rsa4096WithSha1(_) => Some(SignatureMagic::Rsa4096WithSha1),
            Self::Rsa2048WithSha1(_) => Some(SignatureMagic::Rsa2048WithSha1),
            Self::EllipticCurveWithSha1(_) => Some(SignatureMagic::EllipticCurveWithSha1),
            Self::Rsa4096WithSha256(_) => Some(SignatureMagic::Rsa4096WithSha256),
            Self::Rsa2048WithSha256(_) => Some(SignatureMagic::Rsa2048WithSha256),
            Self::EcdsaWithSha256(_) => Some(SignatureMagic::EcdsaWithSha256),
        }
    }
}

/// A newtype that defines various operations on a [`Certificate`]'s issuer section
///
/// [`Certificate`]: ./struct.Certificate.html
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Issuer<'a>(pub Cow<'a, str>);

impl<'a> Issuer<'a> {
    pub fn known_issuer(&self) -> Option<KnownIssuer> {
        KnownIssuer::from_str(&self.0).ok()
    }
}

/// An enumeration over known certificate issuers
#[non_exhaustive]
#[derive(
    IntoStaticStr,
    AsRefStr,
    EnumString,
    Display,
    Copy,
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Ord,
)]
pub enum KnownIssuer {
    #[strum(to_string = "Root-CA00000003-MS00000012")]
    RootCa00000003Ms00000012,
    #[strum(to_string = "Nintendo CA - G3_NintendoCTR2prod")]
    NintendoCaG3NintendoCtr2Prod,
    #[strum(to_string = "Nintendo CA - G3_NintendoCTR2dev")]
    NintendoCaG3NintendoCtr2Dev,
}

/// An enumeration over all possible magic numbers representing a kind of [`Key`]
///
/// [`Key`]: ./enum.Key.html
#[non_exhaustive]
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum KeyMagic {
    Rsa4096 = 0x0,
    Rsa2048 = 0x1,
    EllipticCurve = 0x2,
}

/// An enumeration over all possible key kinds, containing the internal key data
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Key<'a> {
    Rsa4096(Cow<'a, [u8]>),
    Rsa2048(Cow<'a, [u8]>),
    EllipticCurve(Cow<'a, [u8]>),
}

impl Key<'_> {
    /// Converts a [`Key`] into a [`KeyMagic`]
    ///
    /// [`Key`]: ./enum.Key.html
    /// [`KeyMagic`]: ./enum.KeyMagic.html
    pub fn to_key_magic(&self) -> Option<KeyMagic> {
        match &self {
            Self::Rsa4096(_) => Some(KeyMagic::Rsa4096),
            Self::Rsa2048(_) => Some(KeyMagic::Rsa2048),
            Self::EllipticCurve(_) => Some(KeyMagic::EllipticCurve),
        }
    }
}

/// A newtype that defines various operations on a [`Certificate`]'s name section
///
/// [`Certificate`]: ./struct.Certificate.html
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Name<'a>(pub Cow<'a, str>);

impl<'a> Name<'a> {
    /// Extract the device id from the [`Name`], if possible
    ///
    /// [`Name`]: ./struct.Name.html
    pub fn device_id(&self) -> Option<u32> {
        self.0
            .get(..2)
            .map_or(false, |prefix| matches!(prefix, "NG" | "CT"))
            .then(|| {
                self.0
                    .get(2..10)
                    .and_then(|id| u32::from_str_radix(id, 16).ok())
            })
            .flatten()
    }

    /// Attempt to get the [`console::common::Kind`] from the [`Name`], is possible
    ///
    /// [`console::common::Kind`]: ../console/common/enum.Kind.html
    /// [`Name`]: ./struct.Name.html
    pub fn console_kind(&self) -> Option<ConsoleKind> {
        self.0
            .get(..2)
            .map(|k| match k {
                "NG" => Some(ConsoleKind::WiiU),
                "CT" => Some(ConsoleKind::N3ds),
                _ => None,
            })
            .flatten()
    }
}

/// A newtype that defines various operations on a [`Certificate`]'s key id section
///
/// [`Certificate`]: ./struct.Certificate.html
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct KeyId(pub u32);
