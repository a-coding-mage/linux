// SPDX-License-Identifier: GPL-2.0

//! Nullable rotation arguments for the two augmented-rbtree C exports.

use super::bindings_raw::{rb_node, rb_root};

/// The nullable rotation argument of `__rb_insert_augmented` and `__rb_erase_color`.
///
/// Both functions have the C type `void (struct rb_node *, struct rb_root *,
/// void (*)(struct rb_node *, struct rb_node *))`. Their final argument may be
/// null on paths that do not rotate. Keeping the actual `Option` representation
/// preserves that domain, unlike substituting a non-null Rust function pointer.
///
/// Rust 1.85 encodes `Option<fn>` nominally for outer KCFI. The annotation is the
/// callback fragment of the original compiler type encoding
/// `_ZTSFvP7rb_nodeP7rb_rootPFvS0_S0_EE.normalized`. Its substitutions refer to
/// this exact enclosing signature. This is not a general callback typedef and
/// must not be reused in an unrelated C function's argument position.
#[repr(transparent)]
#[derive(Clone, Copy)]
#[cfi_encoding = "PFvS0_S0_E"]
pub struct RbAugmentRotate(Option<unsafe extern "C" fn(*mut rb_node, *mut rb_node)>);

impl RbAugmentRotate {
    /// Wraps the original nullable callback without changing its value.
    #[inline(always)]
    pub const fn from_option(
        callback: Option<unsafe extern "C" fn(*mut rb_node, *mut rb_node)>,
    ) -> Self {
        Self(callback)
    }

    /// Recovers the original callback, to be called only when a rotation occurs.
    #[inline(always)]
    pub const fn into_option(self) -> Option<unsafe extern "C" fn(*mut rb_node, *mut rb_node)> {
        self.0
    }
}

unsafe extern "C" {
    /// Repairs a tree after insertion, calling the callback only upon rotation.
    ///
    /// # Safety
    /// Pointers, links, node colors and reached callbacks must satisfy the
    /// original `__rb_insert_augmented` C contract.
    pub fn __rb_insert_augmented(node: *mut rb_node, root: *mut rb_root, rotate: RbAugmentRotate);

    /// Repairs a tree after removal, calling the callback only upon rotation.
    ///
    /// # Safety
    /// Pointers, links, node colors and reached callbacks must satisfy the
    /// original `__rb_erase_color` C contract.
    pub fn __rb_erase_color(parent: *mut rb_node, root: *mut rb_root, rotate: RbAugmentRotate);
}
