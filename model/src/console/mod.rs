//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

//! A collection of modules providing type definitions for consoles as well as information that is
//! strictly needed to impersonate them
//!
//! Type definitions for data that is not only necessary for impersonating consoles (such as the
//! device certificate, which can be useful for other things, or title information, which is
//! necessary to understand applications that run on consoles), belongs in separate modules outside
//! of this one.

pub mod common;
pub mod n3ds;
pub mod wiiu;
