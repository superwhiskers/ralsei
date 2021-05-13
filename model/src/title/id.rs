//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Type definitions and operations for the title id construct that many Nintendo consoles use as a
//! means of distinguishing titles from one another
//!
//! Specifically, this module contains the [`TitleId`] tuple struct, which has multiple methods
//! defined on it for pulling more specific sections out of it as well as constructing it from
//! existing sections.
//!
//! This specific title id implementation is designed to be used for title ids as implemented on
//! the 3DS and Wii U. While it may work for title ids from other Nintendo consoles, it was not
//! designed to do so. Do not be surprised if it doesn't handle such title ids properly.
//!
//! # Basic usage
//!
//! ```rust
//! # use ralsei_model::title::id::{TitleId, Platform, Variation};
//! // the title id of the 3ds' system settings application
//! let mset_title_id = TitleId(0x0004001000021000);
//!
//! println!("mset's unique id is {:?}!!", mset_title_id.unique_id());
//!
//! assert_eq!(mset_title_id.platform().unwrap(), Platform::Nintendo3ds);
//! assert_eq!(mset_title_id.variation(), Variation(0x0));
//! ```
//!
//! [`TitleId`]: ./struct.TitleId.html

use bitflags::bitflags;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::cast::FromPrimitive;
use unin::u24;

/// A bitmask representing the [`Platform`] portion of a [`TitleId`]
///
/// [`Platform`]: ./enum.Platform.html
/// [`TitleId`]: ./struct.TitleId.html
pub const PLATFORM_BITMASK: u64 =
    0b1111_1111_1111_1111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

/// A bitmask representing the [`Category`] portion of a [`TitleId`]
///
/// [`Category`]: ./struct.Category.html
/// [`TitleId`]: ./struct.TitleId.html
pub const CATEGORY_BITMASK: u64 =
    0b0000_0000_0000_0000_1111_1111_1111_1111_0000_0000_0000_0000_0000_0000_0000_0000;

/// A bitmask representing the [`UniqueId`] portion of a [`TitleId`]
///
/// [`UniqueId`]: ./struct.UniqueId.html
/// [`TitleId`]: ./struct.TitleId.html
pub const UNIQUE_ID_BITMASK: u64 =
    0b0000_0000_0000_0000_0000_0000_0000_0000_1111_1111_1111_1111_1111_1111_0000_0000;

/// A bitmask representing the [`Variation`] portion of a [`TitleId`]
///
/// [`Variation`]: ./struct.Variation.html
/// [`TitleId`]: ./struct.TitleId.html
pub const VARIATION_BITMASK: u64 =
    0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111_1111;

/// A bitmask representing the [`TitleIdHigh`] portion of a [`TitleId`]
///
/// [`TitleIdHigh`]: ./struct.TitleIdHigh.html
/// [`TitleId`]: ./struct.TitleId.html
pub const HIGH_BITMASK: u64 =
    0b1111_1111_1111_1111_1111_1111_1111_1111_0000_0000_0000_0000_0000_0000_0000_0000;

/// A bitmask representing the [`TitleIdLow`] portion of a [`TitleId`]
///
/// [`TitleIdLow`]: ./struct.TitleIdLow.html
/// [`TitleId`]: ./struct.TitleId.html
pub const LOW_BITMASK: u64 =
    0b0000_0000_0000_0000_0000_0000_0000_0000_1111_1111_1111_1111_1111_1111_1111_1111;

/// An integer that corresponds to a title on a Nintendo console
///
/// Contained within are multiple different tidbits of information, such as the title's
/// [`Platform`] and its [`Category`], among other things.
#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TitleId(pub u64);

impl TitleId {
    /// Create a [`TitleId`] from a [`TitleIdHigh`] and a [`TitleIdLow`]
    ///
    /// [`TitleId`]: ./struct.TitleId.html
    /// [`TitleIdHigh`]: ./struct.TitleIdHigh.html
    /// [`TitleIdLow`]: ./struct.TitleIdLow.html
    #[inline]
    pub const fn from_high_and_low(high: TitleIdHigh, low: TitleIdLow) -> Self {
        Self(((high.0 as u64) << 32) | low.0 as u64)
    }

    /// Create a [`TitleId`] from its four components: [`Platform`], [`Category`], [`UniqueId`],
    /// and [`Variation`]
    ///
    /// [`TitleId`]: ./struct.TitleId.html
    /// [`Platform`]: ./enum.Platform.html
    /// [`Category`]: ./struct.Category.html
    /// [`UniqueId`]: ./struct.UniqueId.html
    /// [`Variation`]: ./stuct.Variation.html
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

    /// Extract the [`Platform`] segment from the [`TitleId`]
    ///
    /// [`Platform`]: ./enum.Platform.html
    /// [`TitleId`]: ./struct.TitleId.html
    #[inline]
    pub fn platform(self) -> Option<Platform> {
        Platform::from_u16((self.0 >> 48) as u16)
    }

    /// Extract the [`Category`] segment from the [`TitleId`]
    ///
    /// [`Category`]: ./struct.Category.html
    /// [`TitleId`]: ./struct.TitleId.html
    #[inline]
    pub fn category(self) -> Option<Category> {
        Category::from_bits(((self.0 & CATEGORY_BITMASK) >> 32) as u16)
    }

    /// Extract the [`UniqueId`] segment from the [`TitleId`]
    ///
    /// [`UniqueId`]: ./struct.UniqueId.html
    /// [`TitleId`]: ./stuct.TitleId.html
    #[inline]
    pub fn unique_id(self) -> UniqueId {
        UniqueId(u24::new(((self.0 & UNIQUE_ID_BITMASK) >> 8) as u32))
    }

    /// Extract the [`Variation`] segment from the [`TitleId`]
    ///
    /// [`Variation`]: ./struct.Variation.html
    /// [`TitleId`]: ./struct.TitleId.html
    #[inline]
    pub const fn variation(self) -> Variation {
        Variation((self.0 & VARIATION_BITMASK) as u8)
    }

    /// Extract the [`TitleIdHigh`] segment from the [`TitleId`]
    ///
    /// [`TitleIdHigh`]: ./struct.TitleIdHigh.html
    /// [`TitleId`]: ./struct.TitleId.html
    #[inline]
    pub const fn high(self) -> TitleIdHigh {
        TitleIdHigh((self.0 >> 32) as u32)
    }

    /// Extract the [`TitleIdLow`] segment from the [`TitleId`]
    ///
    /// [`TitleIdLow`]: ./struct.TitleIdLow.html
    /// [`TitleId`]: ./struct.TitleId.html
    #[inline]
    pub const fn low(self) -> TitleIdLow {
        TitleIdLow((self.0 & LOW_BITMASK) as u32)
    }
}

/// A bitmask representing the [`Platform`] portion of a [`TitleIdHigh`]
///
/// [`Platform`]: ./enum.Platform.html
/// [`TitleIdHigh`]: ./struct.TitleIdHigh.html
pub const TIDHIGH_PLATFORM_BITMASK: u32 = 0b1111_1111_1111_1111_0000_0000_0000_0000;

/// A bitmask representing the [`Category`] portion of a [`TitleIdHigh`]
///
/// [`Category`]: ./struct.Category.html
/// [`TitleIdHigh`]: ./struct.TitleIdHigh.html
pub const TIDHIGH_CATEGORY_BITMASK: u32 = 0b0000_0000_0000_0000_1111_1111_1111_1111;

/// The higher portion of a [`TitleId`]
///
/// It is composed of both a [`Platform`] and [`Category`] segment.
///
/// [`TitleId`]: ./struct.TitleId.html
/// [`Platform`]: ./enum.Platform.html
/// [`Category`]: ./struct.Category.html
#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TitleIdHigh(pub u32);

impl TitleIdHigh {
    /// Construct a [`TitleIdHigh`] from a [`Platform`] and a [`Category`]
    ///
    /// [`TitleId`]: ./struct.TitleId.html
    /// [`Platform`]: ./enum.Platform.html
    /// [`Category`]: ./struct.Category.html
    #[inline]
    pub const fn from_platform_and_category(platform: Platform, category: Category) -> Self {
        Self((platform as u32) << 16 | (category.bits() as u32))
    }

    /// Extract the [`Platform`] segment from the [`TitleIdHigh`]
    ///
    /// [`Platform`]: ./enum.Platform.html
    /// [`TitleIdHigh`]: ./struct.TitleIdHigh.html
    #[inline]
    pub fn platform(self) -> Option<Platform> {
        Platform::from_u16((self.0 >> 16) as u16)
    }

    /// Extract the [`Category`] segment from the [`TitleIdHigh`]
    ///
    /// [`Category`]: ./struct.Category.html
    /// [`TitleIdHigh`]: ./struct.TitleIdHigh.html
    #[inline]
    pub fn category(self) -> Option<Category> {
        Category::from_bits((self.0 & TIDHIGH_CATEGORY_BITMASK) as u16)
    }
}

/// A bitmask representing the [`UniqueId`] portion of a [`TitleIdLow`]
///
/// [`UniqueId`]: ./struct.UniqueId.html
/// [`TitleIdLow`]: ./struct.TitleIdLow.html
pub const TIDLOW_UNIQUE_ID_BITMASK: u32 = 0b1111_1111_1111_1111_1111_1111_0000_0000;

/// A bitmask representing the [`Variation`] portion of a [`TitleIdLow`]
///
/// [`Variation`]: ./struct.Variation.html
/// [`TitleIdLow`]: ./struct.TitleIdLow.html
pub const TIDLOW_VARIATION_BITMASK: u32 = 0b0000_0000_0000_0000_0000_0000_1111_1111;

/// The lower portion of a [`TitleId`]
///
/// It is composed of both a [`UniqueId`] and [`Variation`] segment.
///
/// [`TitleId`]: ./struct.TitleId.html
/// [`UniqueId`]: ./struct.UniqueId.html
/// [`Variation`]: ./struct.Variation.html
#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TitleIdLow(pub u32);

impl TitleIdLow {
    /// Constructs a [`TitleIdLow`] from a [`UniqueId`] and a [`Variation`]
    ///
    /// [`TitleIdLow`]: ./struct.TitleIdLow.html
    /// [`UniqueId`]: ./struct.UniqueId.html
    /// [`Variation`]: ./struct.Variation.html
    #[inline]
    pub fn from_unique_id_and_variation(unique_id: UniqueId, variation: Variation) -> Self {
        Self(u32::from(unique_id.0) << 8 | u32::from(variation.0))
    }

    /// Extract the [`UniqueId`] segment from the [`TitleIdLow`]
    ///
    /// [`UniqueId`]: ./struct.UniqueId.html
    /// [`TitleIdLow`]: ./struct.TitleIdLow.html
    #[inline]
    pub fn unique_id(self) -> UniqueId {
        UniqueId(u24::new(self.0 >> 8))
    }

    /// Extract the [`Variation`] segment from the [`TitleIdLow`]
    ///
    /// [`Variation`]: ./struct.Variation.html
    /// [`TitleIdLow`]: ./struct.TitleIdLow.html
    #[inline]
    pub const fn variation(self) -> Variation {
        Variation((self.0 & TIDLOW_VARIATION_BITMASK) as u8)
    }
}

/// An enumeration over the possible platforms of a [`TitleId`]. Not all are listed as only some of
/// the possible platforms are supported
///
/// [`TitleId`]: ./struct.TitleId.html
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Platform {
    NintendoWiiU = 5,
    Nintendo3ds = 4,
}

bitflags! {
    /// A data structure representing the category section of a [`TitleId`].
    ///
    /// Instead of implementing the "normal" flag (`0x0`) as another possible bitflag, it is
    /// implemented as the [`is_normal`] method due to it requiring extra logic
    ///
    /// [`TitleId`]: ./struct.TitleId.html
    /// [`is_normal`]: #method.is_normal
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

impl Category {
    /// Check if the enveloping [`TitleId`] represents a "normal" title
    ///
    /// [`TitleId`]: ./struct.TitleId.html
    #[inline]
    pub fn is_normal(self) -> bool {
        //TODO(superwhiskers): verify that this is how it actually works
        !self.contains(Self::CONTENTS)
    }
}

/// A bitmask representing the hardware portion of a [`UniqueId`]
///
/// [`UniqueId`]: ./struct.UniqueId.html
pub const UNIQUE_ID_HARDWARE_BITMASK: u32 = 0b1111_0000_0000_0000_0000_0000;

/// A bitmask representing the identifier portion of a [`UniqueId`]
///
/// This section of the [`UniqueId`] is represented in the library by the [`UniqueIdGroup`]
/// enumeration.
///
/// [`UniqueId`]: ./struct.UniqueId.html
/// [`UniqueIdGroup`]: ./enum.UniqueIdGroup.html
pub const UNIQUE_ID_IDENTIFIER_BITMASK: u32 = 0b0000_1111_1111_1111_1111_1111;

/// A data structure representing the unique id section of a [`TitleId`]
///
/// Aside from merely acting as a container, it provides a few operations that can be performed on
/// the contained unique id.
///
/// [`TitleId`]: ./struct.TitleId.html
#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct UniqueId(pub u24);

impl UniqueId {
    /// Determine the title group that the [`UniqueId`] fits within
    ///
    /// Note that this may not be correct, as titles don't appear to be required to conform to the
    /// correct region, and may not be in a group at all. However, this metric is usually reliable,
    /// and can be reasonably relied upon.
    ///
    /// See [3dbrew] for more information.
    ///
    /// [`UniqueId`]: ./struct.UniqueId.html
    /// [3dbrew]: https://www.3dbrew.org/wiki/Title
    #[inline]
    pub fn group(self) -> Option<UniqueIdGroup> {
        match u24::new(UNIQUE_ID_IDENTIFIER_BITMASK) & self.0 {
            id if id < u24::new(0x300) => Some(UniqueIdGroup::System),
            id if id < u24::new(0xF8000) => Some(UniqueIdGroup::Application),
            id if id < u24::new(0xFF000) => Some(UniqueIdGroup::Evaluation),
            id if id < u24::new(0xFF400) => Some(UniqueIdGroup::Prototype),
            id if id < u24::new(0xFF800) => Some(UniqueIdGroup::Developer),
            _ => None,
        }
    }

    /// Determine if the [`UniqueId`] represents a title that is New3ds-only
    ///
    /// [`UniqueId`]: ./struct.UniqueId.html
    #[inline]
    pub fn is_new3ds_only(self) -> bool {
        (u24::new(UNIQUE_ID_HARDWARE_BITMASK) & self.0) >> 20 == u24::new(2)
    }
}

/// An enumeration over the possible groups that a [`UniqueId`] can conform to
///
/// As stated in the documentation of [`UniqueId::group`], a title may not conform to one of these,
/// or may be in the improper group. However, this metric is usually reliable, and can be
/// reasonably relied upon.
///
/// See [3dbrew] for more information.
///
/// [`UniqueId`]: ./struct.UniqueId.html
/// [`UniqueId::group`]: ./struct.UniqueId.html#method.group
/// [3dbrew]: https://www.3dbrew.org/wiki/Title
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum UniqueIdGroup {
    System,
    Application,
    Evaluation,
    Prototype,
    Developer,
}

/// The variation segment of a [`TitleId`]
///
/// This portion of the [`TitleId`] generally allows for multiple titles with the same [`Category`]
/// and [`UniqueId`] te be installed.
///
/// One specific use of this is on the 3DS, where there are two copies of most system titles for
/// use on both `NATIVE_FIRM` and `SAFE_MODE_FIRM`, which is allowed by setting the variation
/// segment to the core version set by the firm it is designed to be used with. However, this is
/// merely a convention/idiom.
///
/// Another thing to note is that for all titles where [`Category`] contains [`Category::TWL`], the
/// variation segment is ignored as on the DSi/DS, it is used for region locking.
///
/// See [3dbrew] for more information
///
/// [`TitleId`]: ./struct.TitleId.html
/// [`Category`]: ./struct.Category.html
/// [`UniqueId`]: ./struct.UniqueId.html
/// [`Category::TWL`]: ./struct.Category.html#associatedconstant.TWL
/// [3dbrew]: https://www.3dbrew.org/wiki/Title
#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Variation(pub u8);

#[cfg(test)]
mod test {
    use std::lazy::Lazy;

    use super::*;

    macro generate_derivation_test($tname:ident, $left:expr, $fn:ident, $right:expr) {
        #[test]
        fn $tname() {
            assert_eq!($left.$fn(), $right);
        }
    }

    macro generate_fallible_derivation_test($tname:ident, $left:expr, $fn:ident, $right:expr) {
        #[test]
        fn $tname() {
            assert_eq!($left.$fn().unwrap(), $right);
        }
    }

    const MSET_TITLE_ID: TitleId = TitleId(0x0004001000020000);
    const MSET_TITLE_ID_HIGH: TitleIdHigh = TitleIdHigh(0x00040010);
    const MSET_TITLE_ID_LOW: TitleIdLow = TitleIdLow(0x00020000);
    const MSET_TITLE_ID_PLATFORM: Platform = Platform::Nintendo3ds;
    const MSET_TITLE_ID_CATEGORY: Category = Category::SYSTEM;
    const MSET_TITLE_ID_UNIQUE_ID: Lazy<UniqueId> =
        Lazy::new(|| UniqueId(u24::new(0x000200 as u32)));
    const MSET_TITLE_ID_VARIATION: Variation = Variation(0x0);
    const MSET_TITLE_ID_UNIQUE_ID_GROUP: UniqueIdGroup = UniqueIdGroup::System;

    #[test]
    fn titleid_from_high_and_low() {
        assert_eq!(
            TitleId::from_high_and_low(MSET_TITLE_ID_HIGH, MSET_TITLE_ID_LOW),
            MSET_TITLE_ID
        );
    }

    #[test]
    fn titleid_from_segments() {
        assert_eq!(
            TitleId::from_segments(
                MSET_TITLE_ID_PLATFORM,
                MSET_TITLE_ID_CATEGORY,
                *MSET_TITLE_ID_UNIQUE_ID,
                MSET_TITLE_ID_VARIATION,
            ),
            MSET_TITLE_ID
        );
    }

    generate_fallible_derivation_test!(
        titleid_platform,
        MSET_TITLE_ID,
        platform,
        MSET_TITLE_ID_PLATFORM
    );
    generate_fallible_derivation_test!(
        titleid_category,
        MSET_TITLE_ID,
        category,
        MSET_TITLE_ID_CATEGORY
    );
    generate_derivation_test!(
        titleid_unique_id,
        MSET_TITLE_ID,
        unique_id,
        *MSET_TITLE_ID_UNIQUE_ID
    );
    generate_derivation_test!(
        titleid_variation,
        MSET_TITLE_ID,
        variation,
        MSET_TITLE_ID_VARIATION
    );
    generate_derivation_test!(titleid_high, MSET_TITLE_ID, high, MSET_TITLE_ID_HIGH);
    generate_derivation_test!(titleid_low, MSET_TITLE_ID, low, MSET_TITLE_ID_LOW);

    #[test]
    fn titleid_high_from_platform_and_category() {
        assert_eq!(
            TitleIdHigh::from_platform_and_category(MSET_TITLE_ID_PLATFORM, MSET_TITLE_ID_CATEGORY),
            MSET_TITLE_ID_HIGH
        );
    }

    generate_fallible_derivation_test!(
        titleid_high_platform,
        MSET_TITLE_ID_HIGH,
        platform,
        MSET_TITLE_ID_PLATFORM
    );
    generate_fallible_derivation_test!(
        titleid_high_category,
        MSET_TITLE_ID_HIGH,
        category,
        MSET_TITLE_ID_CATEGORY
    );

    #[test]
    fn titleid_low_from_unique_id_and_variation() {
        assert_eq!(
            TitleIdLow::from_unique_id_and_variation(
                *MSET_TITLE_ID_UNIQUE_ID,
                MSET_TITLE_ID_VARIATION
            ),
            MSET_TITLE_ID_LOW
        );
    }

    generate_derivation_test!(
        titleid_low_unique_id,
        MSET_TITLE_ID_LOW,
        unique_id,
        *MSET_TITLE_ID_UNIQUE_ID
    );
    generate_derivation_test!(
        titleid_low_variation,
        MSET_TITLE_ID_LOW,
        variation,
        MSET_TITLE_ID_VARIATION
    );

    generate_derivation_test!(category_is_normal, MSET_TITLE_ID_CATEGORY, is_normal, true);

    generate_fallible_derivation_test!(
        unique_id_system_group,
        MSET_TITLE_ID_UNIQUE_ID,
        group,
        MSET_TITLE_ID_UNIQUE_ID_GROUP
    );
    generate_fallible_derivation_test!(
        unique_id_application_group,
        UniqueId(u24::new(0xC0F44)),
        group,
        UniqueIdGroup::Application
    );
    generate_fallible_derivation_test!(
        unique_id_evaluation_group,
        UniqueId(u24::new(0xF9426)),
        group,
        UniqueIdGroup::Evaluation
    );
    generate_fallible_derivation_test!(
        unique_id_prototype_group,
        UniqueId(u24::new(0xFF327)),
        group,
        UniqueIdGroup::Prototype
    );
    generate_fallible_derivation_test!(
        unique_id_developer_group,
        UniqueId(u24::new(0xFF496)),
        group,
        UniqueIdGroup::Developer
    );
    #[test]
    fn unique_id_unmatched() {
        assert_eq!(UniqueId(u24::new(0xFFFFF)).group(), None);
    }
    generate_derivation_test!(
        unique_id_is_new_3ds_only,
        MSET_TITLE_ID_UNIQUE_ID,
        is_new3ds_only,
        false
    );
}
