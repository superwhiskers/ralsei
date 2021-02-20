//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

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
