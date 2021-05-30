//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

//TODO(superwhiskers): finish documentation
//! This module provides

use std::borrow::Cow;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// An enumeration over the different kinds of Nintendo Network servers, each of which contain the
/// host address of the server
#[non_exhaustive]
#[derive(
    IntoStaticStr, AsRefStr, EnumString, Display, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord,
)]
pub enum Kind<'a> {
    Account(Cow<'a, str>),
}
