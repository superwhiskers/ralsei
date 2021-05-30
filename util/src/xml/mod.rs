//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use std::lazy::SyncLazy;

pub mod errors;
pub mod framework;
pub mod helpers;

use framework::{BufferPool, BufferPoolManager};

/// Shared global [`Vec<u8>`] pool for parsing purposes
///
/// Has a hardcoded capacity of 1000, as this really should not be used anywhere outside of quick
/// examples or really hacked together, single-use programs
pub static GLOBAL_BUFFER_POOL: SyncLazy<BufferPool> =
    SyncLazy::new(|| BufferPool::new(BufferPoolManager {}, 1000));
