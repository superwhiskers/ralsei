//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use std::lazy::SyncLazy;

use crate::model::xml::conversion::{BufferPool, BufferPoolManager};

/// Shared global [`Vec<u8>`] pool for parsing purposes
///
/// Has a hardcoded capacity of 100, as this really should not be used anywhere outside of quick
/// examples or really hacked together, single-use programs
///
/// [`Vec<u8>`]: https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html
pub(crate) static GLOBAL_BUFFER_POOL: SyncLazy<BufferPool> =
    SyncLazy::new(|| BufferPool::new(BufferPoolManager {}, 100));

/// Helper macro used to aid in generating methods for builder-like types
pub macro builder_set($field_name:literal, $container_field:ident, $field:ident, $type:ty) {
    #[doc = "Sets the `"]
    #[doc = $field_name]
    #[doc = "` field to the provided value"]
    pub fn $field(&mut self, $field: $type) -> &mut Self {
        self.$container_field.$field = Some($field);
        self
    }
}
