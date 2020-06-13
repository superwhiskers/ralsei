//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use ux::{u4, u6};

/// a bitmask for extracting the major version
/// from a title version
pub const MAJOR_VERSION_BITMASK: u16 = 0b11111100_00000000;

/// a bitmask for extracting the minor version
/// from a title version
pub const MINOR_VERSION_BITMASK: u16 = 0b00000011_11110000;

/// a bitmask for extracting the micro version
/// from a title version
pub const MICRO_VERSION_BITMASK: u16 = 0b00000000_00001111;

/// an integer representing the version of a
/// title on either the 3ds or the wiiu
#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TitleVersion(pub u16);

impl TitleVersion {
    /// constructs a title version from its segments
    #[inline]
    pub fn from_segments(major: u6, minor: u6, micro: u4) -> Self {
        Self((u16::from(major) << 10) | (u16::from(minor) << 4) | u16::from(micro))
    }

    /// returns the title version's major version
    #[inline]
    pub fn major(&self) -> u6 {
        u6::new((self.0 >> 10) as u8)
    }

    /// returns the title version's minor version
    #[inline]
    pub fn minor(&self) -> u6 {
        u6::new(((self.0 & MINOR_VERSION_BITMASK) >> 4) as u8)
    }

    /// returns the title version's micro version
    #[inline]
    pub fn micro(&self) -> u4 {
        u4::new((self.0 & MICRO_VERSION_BITMASK) as u8)
    }
}
