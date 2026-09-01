/* SPDX-License-Identifier: GPL-2.0 */

// Header guard omitted in Rust.

// C macro: #define module_param(name, type, perm)
macro_rules! module_param {
    ($name:expr, $type:expr, $perm:expr) => {};
}

#[inline]
pub unsafe fn __is_module_percpu_address(
    addr: ::core::ffi::c_ulong,
    can_addr: *mut ::core::ffi::c_ulong,
) -> bool {
    let _ = addr;
    let _ = can_addr;
    false
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
