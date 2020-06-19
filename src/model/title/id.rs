//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use bitflags::bitflags;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::cast::FromPrimitive;
use num_x::u24;

/// a bitmask for extracting the platform section
/// of a titleid
pub const PLATFORM_BITMASK: u64 =
    0b1111_1111_1111_1111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

/// a bitmask for extracting the content category
/// section of a titleid
pub const CATEGORY_BITMASK: u64 =
    0b0000_0000_0000_0000_1111_1111_1111_1111_0000_0000_0000_0000_0000_0000_0000_0000;

/// a bitmask for extracting the unique id section
/// of a titleid
pub const UNIQUE_ID_BITMASK: u64 =
    0b0000_0000_0000_0000_0000_0000_0000_0000_1111_1111_1111_1111_1111_1111_0000_0000;

/// a bitmask for extracting the title id variation
/// section of a title id
pub const VARIATION_BITMASK: u64 =
    0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111_1111;

/// a bitmask for extracting the title id high
/// section of a title id
pub const HIGH_BITMASK: u64 =
    0b1111_1111_1111_1111_1111_1111_1111_1111_0000_0000_0000_0000_0000_0000_0000_0000;

/// a bitmask for extracting the title id low
/// section of a title id
pub const LOW_BITMASK: u64 =
    0b0000_0000_0000_0000_0000_0000_0000_0000_1111_1111_1111_1111_1111_1111_1111_1111;

/// an identifying integer that corresponds to
/// a title on either the 3ds or the wiiu
#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TitleId(pub u64);

impl TitleId {
    /// constructs a title id from a high and low segment
    #[inline]
    pub const fn from_high_and_low(high: TitleIdHigh, low: TitleIdLow) -> Self {
        Self(((high.0 as u64) << 32) | low.0 as u64)
    }

    /// constructs a title id from all four segments of one
    #[inline]
    pub fn from_segments(
        platform: Platform,
        category: Category,
        unique_id: UniqueId,
        variation: Variation,
    ) -> Self {
        Self(
            ((platform as u64) << 48)
                | (u64::from(category.bits()) << 32)
                | (u64::from(unique_id.0) << 8)
                | u64::from(variation.0),
        )
    }

    /// returns the title id's platform segment
    #[inline]
    pub fn platform(self) -> Option<Platform> {
        Platform::from_u16((self.0 >> 48) as u16)
    }

    /// returns the title id's category segment
    #[inline]
    pub fn category(self) -> Option<Category> {
        Category::from_bits(((self.0 & CATEGORY_BITMASK) >> 32) as u16)
    }

    /// returns the title id's unique id segment
    #[inline]
    pub fn unique_id(self) -> UniqueId {
        UniqueId(u24::new(((self.0 & UNIQUE_ID_BITMASK) >> 8) as u32))
    }

    /// returns the title id's variation segment
    #[inline]
    pub const fn variation(self) -> Variation {
        Variation((self.0 & VARIATION_BITMASK) as u8)
    }

    /// returns the title id's high (platform & category) segment
    #[inline]
    pub const fn high(self) -> TitleIdHigh {
        TitleIdHigh((self.0 >> 32) as u32)
    }

    /// returns the title id's low (unique & variation) segment
    #[inline]
    pub const fn low(self) -> TitleIdLow {
        TitleIdLow((self.0 & LOW_BITMASK) as u32)
    }
}

/// a bitmask for extracting the platform
/// section of the high segment of a
/// title id
pub const TIDHIGH_PLATFORM_BITMASK: u32 = 0b1111_1111_1111_1111_0000_0000_0000_0000;

/// a bitmask for extracting the category
/// section of the high segment of a
/// title id
pub const TIDHIGH_CATEGORY_BITMASK: u32 = 0b0000_0000_0000_0000_1111_1111_1111_1111;

/// a newtype containing the high segment of a
/// title id
#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TitleIdHigh(pub u32);

impl TitleIdHigh {
    /// constructs a high title id from a platform
    /// segment and a category segment
    #[inline]
    pub const fn from_platform_and_category(platform: Platform, category: Category) -> Self {
        Self((platform as u32) << 16 | (category.bits() as u32))
    }

    /// returns the high title id's platform segment
    #[inline]
    pub fn platform(self) -> Option<Platform> {
        Platform::from_u16((self.0 >> 16) as u16)
    }

    /// returns the high title id's category segment
    #[inline]
    pub fn category(self) -> Option<Category> {
        Category::from_bits((self.0 & TIDHIGH_CATEGORY_BITMASK) as u16)
    }
}

/// a bitmask for extracting the unique id
/// section of the low segment of a titleid
pub const TIDLOW_UNIQUE_ID_BITMASK: u32 = 0b1111_1111_1111_1111_1111_1111_0000_0000;

/// a bitmask for extracting the title id
/// variation section of the low segment of
/// a title id
pub const TIDLOW_VARIATION_BITMASK: u32 = 0b0000_0000_0000_0000_0000_0000_1111_1111;

/// a newtype containing the low segment of a
/// title id
#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TitleIdLow(pub u32);

impl TitleIdLow {
    /// constructs a low title id from a unique id
    /// segment and a variation segment
    #[inline]
    pub fn from_unique_id_and_variation(unique_id: UniqueId, variation: Variation) -> Self {
        Self(u32::from(unique_id.0) << 8 | u32::from(variation.0))
    }

    /// returns the low title id's unique id segment
    #[inline]
    pub fn unique_id(self) -> UniqueId {
        UniqueId(u24::new(self.0 >> 8))
    }

    /// returns the title id's variation segment
    #[inline]
    pub const fn variation(self) -> Variation {
        Variation((self.0 & TIDLOW_VARIATION_BITMASK) as u8)
    }
}

/// an enum representing the possible platforms
/// and their corresponding values in the titleid
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Platform {
    NintendoWiiU = 5,
    Nintendo3ds = 4,
}

bitflags! {
    /// a newtype that defines various operations on
    /// a title id's category section
    pub struct Category: u16 {
        const DLPCHILD = 0b0000_0000_0000_0001;
        const DEMO = 0b0000_0000_0000_0010;
        const CONTENTS = 0b0000_0000_0000_0011;
        const ADDONCONTENTS = 0b0000_0000_0000_0100;
        const PATCH = 0b0000_0000_0000_0110;
        const CANNOTEXECUTION = 0b0000_0000_0000_1000;
        const SYSTEM = 0b0000_0000_0001_0000;
        const REQUIREBATCHUPDATE = 0b0000_0000_0010_0000;
        const NOTREQUIREUSERAPPROVAL = 0b0000_0000_0100_0000;
        const NOTREQUIRERIGHTFORMOUNT = 0b0000_0000_1000_0000;
        const CANSKIPCONVERTJUMPID = 0b0000_0001_0000_0000;
        const TWL = 0b1000_0000_0000_0000;
    }
}

/// a newtype that defines various operations on
/// a title id's unique id section
#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct UniqueId(pub u24);

impl UniqueId {
    /// returns the unique id group that the
    /// unique id is a part of, if it has
    /// one
    #[inline]
    pub fn group(self) -> Option<UniqueIdGroup> {
        match u24::new(0b0000_1111_1111_1111_1111_1111) & self.0 {
            id if id < u24::new(0x300) => Some(UniqueIdGroup::System),
            id if id < u24::new(0xF8000) => Some(UniqueIdGroup::Application),
            id if id < u24::new(0xFF000) => Some(UniqueIdGroup::Evaluation),
            id if id < u24::new(0xFF400) => Some(UniqueIdGroup::Prototype),
            id if id < u24::new(0xFF800) => Some(UniqueIdGroup::Developer),
            _ => None,
        }
    }

    /// tests if the unique id indicates that
    /// the corresponding title is new3ds only
    #[inline]
    pub fn is_new3ds_only(self) -> bool {
        (u24::new(0b1111_0000_0000_0000_0000_0000) & self.0) >> 20 == u24::new(2)
    }
}

/// the group that a unique id belongs to
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum UniqueIdGroup {
    System,
    Application,
    Evaluation,
    Prototype,
    Developer,
}

/// a newtype that contains a title id's
/// variation segment
#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Variation(pub u8);
