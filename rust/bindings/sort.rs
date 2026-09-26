// SPDX-License-Identifier: GPL-2.0
//! Nullable adapters specific to the four sort export signatures.
// Included as a module of bindings; generic generated Option aliases stay intact.
use super::{cmp_func_t, cmp_r_func_t, swap_func_t, swap_r_func_t};

macro_rules! adapter {
    ($name:ident, $alias:ident, $encoding:literal) => {
        #[doc = "Nullable callback in its exact enclosing sort signature context."]
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        #[cfi_encoding = $encoding]
        pub struct $name($alias);
        const _: () = {
            assert!(core::mem::size_of::<$name>() == core::mem::size_of::<$alias>());
            assert!(core::mem::align_of::<$name>() == core::mem::align_of::<$alias>());
        };
        impl $name {
            /// Preserves the genuine bindgen Option representation.
            pub const fn from_option(value: $alias) -> Self {
                Self(value)
            }
            /// Returns the original nullable callback.
            pub const fn into_option(self) -> $alias {
                self.0
            }
        }
    };
}
adapter!(SortCmp, cmp_func_t, "PFu3i32PKvS3_E");
adapter!(SortSwap, swap_func_t, "PFvS_S_S1_E");
adapter!(SortRCmp, cmp_r_func_t, "PFu3i32PKvS3_S3_E");
adapter!(SortRSwap, swap_r_func_t, "PFvS_S_S1_S3_E");

/// Context pointer in the exact sort_r/sort_r_nonatomic signature.
///
/// Clang encodes this final const-void pointer as a substitution for the one
/// introduced inside the comparator type. Custom callback encodings do not
/// populate rustc's substitution table, so retain that original spelling here.
#[repr(transparent)]
#[derive(Clone, Copy)]
#[cfi_encoding = "S3_"]
pub struct SortPriv(*const ffi::c_void);

impl SortPriv {
    /// Preserve the caller's original context pointer, including null.
    pub const fn from_ptr(value: *const ffi::c_void) -> Self {
        Self(value)
    }
    /// Recover the original context pointer without accessing it.
    pub const fn as_ptr(self) -> *const ffi::c_void {
        self.0
    }
}

unsafe extern "C" {
    /// Original sort interface with signature-specific nullable parameters.
    pub fn sort(base: *mut ffi::c_void, num: usize, size: usize, cmp: SortCmp, swap: SortSwap);
    /// Context-aware original sort interface.
    pub fn sort_r(
        base: *mut ffi::c_void,
        num: usize,
        size: usize,
        cmp: SortRCmp,
        swap: SortRSwap,
        priv_: SortPriv,
    );
    /// Original sort interface with periodic cond_resched.
    pub fn sort_nonatomic(
        base: *mut ffi::c_void,
        num: usize,
        size: usize,
        cmp: SortCmp,
        swap: SortSwap,
    );
    /// Context-aware original sort interface with periodic cond_resched.
    pub fn sort_r_nonatomic(
        base: *mut ffi::c_void,
        num: usize,
        size: usize,
        cmp: SortRCmp,
        swap: SortRSwap,
        priv_: SortPriv,
    );
}
