//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use std::borrow::Cow;

/// A Nintendo Network Id
pub struct Nnid<'a>(pub Cow<'a, str>);

/// A PID associated with a Nintendo Network Id
pub struct Pid(pub u32);
