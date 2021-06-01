//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use std::{borrow::Cow, fmt};

/// An enumeration over possible identifiers used on Nintendo Network
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Identifier<'a> {
    Nnid(Nnid<'a>),
    Pid(Pid),
}

impl<'a> Identifier<'a> {
    /// Convert the [`Identifier`] into a [`Cow<'a, str>`](Cow), borrowing it if it is an [`Nnid`]
    /// or creating a new [`String`] if it is a [`Pid`]
    pub fn to_cow(&'a self) -> Cow<'a, str> {
        match &self {
            Self::Nnid(nnid) => Cow::Borrowed(nnid.0.as_ref()),
            Self::Pid(pid) => Cow::Owned(pid.0.to_string()),
        }
    }

    /// Returns the corresponding [`IdentifierKind`] for the [`Identifier`]
    pub fn kind(&self) -> IdentifierKind {
        match &self {
            Self::Nnid(_) => IdentifierKind::Nnid,
            Self::Pid(_) => IdentifierKind::Pid,
        }
    }
}

impl<'a> fmt::Display for Identifier<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Nnid(nnid) => formatter.write_str(nnid.0.as_ref()),
            Self::Pid(pid) => pid.0.fmt(formatter),
        }
    }
}

/// An enumeration over different kinds of identifiers on Nintendo Network
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum IdentifierKind {
    Nnid,
    Pid,
}

/// A Nintendo Network Id
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct Nnid<'a>(pub Cow<'a, str>);

/// A PID associated with a Nintendo Network Id
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct Pid(pub u32);
