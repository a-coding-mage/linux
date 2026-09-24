/* SPDX-License-Identifier: GPL-2.0 */

//! C declarations for list sorting, using the kernel's actual list layout.
use core::ffi::c_void;

pub use kernel::bindings::list_head;

/// Nonnull comparator required by the original C list_sort declaration.
#[allow(non_camel_case_types)]
pub type list_cmp_func_t = unsafe extern "C" fn(
    priv_: *mut c_void,
    a: *const list_head,
    b: *const list_head,
) -> i32;

// The C declaration carries __attribute__((nonnull(2,3))).
unsafe extern "C" {
    /// Sorts a valid exclusively accessed circular list with a nonnull comparator.
    pub fn list_sort(
        priv_: *mut c_void,
        head: *mut list_head,
        cmp: list_cmp_func_t,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
