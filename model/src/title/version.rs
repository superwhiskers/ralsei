//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Type definitions and operations for the title version construct that was used by Nintendo
//! consoles from the DS up until the Switch (unverified)
//!
//! This module provides the [`TitleVersion`] tuple struct, which contains a `u16` encoded in the
//! version format. There are three methods provided for accessing the data inside: [`major`],
//! [`minor`], and [`micro`]--which return the major, minor, and micro versions of the title,
//! respectively.
//!
//! # Basic usage
//!
//! ```rust
//! # use ralsei_model::title::version::TitleVersion;
//! # use num_x::{u4, u6};
//! // the title version of the 3ds' system settings application as of 19-02-2021
//! let mset_title_version = TitleVersion(10241);
//!
//! println!("mset's major version is {:?}!!", mset_title_version.major());
//!
//! assert_eq!(mset_title_version.minor(), u6::new(0));
//! assert_eq!(mset_title_version.micro(), u4::new(1));
//! ```
//!
//! [`TitleVersion`]: ./struct.TitleVersion.html
//! [`major`]: ./struct.TitleVersion.html#method.major
//! [`minor`]: ./struct.TitleVersion.html#method.minor
//! [`micro`]: ./struct.TitleVersion.html#method.micro

use unin::{u4, u6};

/// A bitmask representing the major version portion of a [`TitleVersion`]
///
/// [`TitleVersion`]: ./struct.TitleVersion.html
pub const MAJOR_VERSION_BITMASK: u16 = 0b1111_1100_0000_0000;

/// A bitmask representing the minor version portion of a [`TitleVersion`]
///
/// [`TitleVersion`]: ./struct.TitleVersion.html
pub const MINOR_VERSION_BITMASK: u16 = 0b0000_0011_1111_0000;

/// A bitmask representing the micro version portion of a [`TitleVersion`]
///
/// [`TitleVersion`]: ./struct.TitleVersion.html
pub const MICRO_VERSION_BITMASK: u16 = 0b0000_0000_0000_1111;

/// An integer that represents the version of a corresponding title
///
/// There are three distinct segments: the major, minor, and micro segments, with the major segment
/// being six bits in length, the minor version being six, and the micro being four again.
#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TitleVersion(pub u16);

impl TitleVersion {
    /// Create a [`TitleVersion`] from its components
    ///
    /// [`TitleVersion`]: ./struct.TitleVersion.html
    #[inline]
    pub fn from_segments(major: u6, minor: u6, micro: u4) -> Self {
        Self((u16::from(major) << 10) | (u16::from(minor) << 4) | u16::from(micro))
    }

    /// Extract the major segment from the [`TitleVersion`]
    ///
    /// [`TitleVersion`]: ./struct.TitleVersion.html
    #[inline]
    pub fn major(self) -> u6 {
        u6::new((self.0 >> 10) as u8)
    }

    /// Extract the minor segment from the [`TitleVersion`]
    ///
    /// [`TitleVersion`]: ./struct.TitleVersion.html
    #[inline]
    pub fn minor(self) -> u6 {
        u6::new(((self.0 & MINOR_VERSION_BITMASK) >> 4) as u8)
    }

    /// Extract the micro segment from the [`TitleVersion`]
    ///
    /// [`TitleVersion`]: ./struct.TitleVersion.html
    #[inline]
    pub fn micro(self) -> u4 {
        u4::new((self.0 & MICRO_VERSION_BITMASK) as u8)
    }
}

#[cfg(test)]
mod test {
    use std::lazy::Lazy;

    use super::*;

    const MSET_TITLE_VERSION: TitleVersion = TitleVersion(10241);
    const MSET_TITLE_VERSION_MAJOR: Lazy<u6> = Lazy::new(|| u6::new(10));
    const MSET_TITLE_VERSION_MINOR: Lazy<u6> = Lazy::new(|| u6::new(0));
    const MSET_TITLE_VERSION_MICRO: Lazy<u4> = Lazy::new(|| u4::new(1));

    #[test]
    fn title_version_from_segments() {
        assert_eq!(
            TitleVersion::from_segments(
                *MSET_TITLE_VERSION_MAJOR,
                *MSET_TITLE_VERSION_MINOR,
                *MSET_TITLE_VERSION_MICRO
            ),
            MSET_TITLE_VERSION
        );
    }

    #[test]
    fn title_version_major() {
        assert_eq!(MSET_TITLE_VERSION.major(), *MSET_TITLE_VERSION_MAJOR);
    }

    #[test]
    fn title_version_minor() {
        assert_eq!(MSET_TITLE_VERSION.minor(), *MSET_TITLE_VERSION_MINOR);
    }

    #[test]
    fn title_version_micro() {
        assert_eq!(MSET_TITLE_VERSION.micro(), *MSET_TITLE_VERSION_MICRO);
    }
}
