// SPDX-License-Identifier: GPL-2.0
//! Signature-specific nullable callback for the fifth bsearch argument.
//!
//! The encoding is derived from the original C declaration, not a universal
//! encoding for cmp_func_t. With CFI enabled, CONFIG_RUST selects normalized
//! CFI integers. Do not reuse this adapter in a different argument context.

/// Nullable comparator in the exact bsearch C signature.
#[repr(transparent)]
#[derive(Clone, Copy)]
#[cfi_encoding = "PFu3i32S1_S1_E"]
pub struct BsearchCmp(super::cmp_func_t);

const _: () = {
    assert!(core::mem::size_of::<BsearchCmp>() == core::mem::size_of::<super::cmp_func_t>());
    assert!(core::mem::align_of::<BsearchCmp>() == core::mem::align_of::<super::cmp_func_t>());
};

impl BsearchCmp {
    /// Preserve the generated C callback's Some/None representation.
    pub const fn from_option(value: super::cmp_func_t) -> Self {
        Self(value)
    }

    /// Recover the generated callback, including its null value.
    pub const fn into_option(self) -> super::cmp_func_t {
        self.0
    }
}

unsafe extern "C" {
    /// Search an ascending array; a zero count permits null arguments.
    pub fn bsearch(
        key: *const ffi::c_void,
        base: *const ffi::c_void,
        num: usize,
        size: usize,
        cmp: BsearchCmp,
    ) -> *mut ffi::c_void;
}
