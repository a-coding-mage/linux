/* SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause */

// Translated from a C header. The C source uses the non-standard `__arena`
// address-space qualifier on pointers; Rust has no direct file-local equivalent,
// so those pointers are represented as raw pointers.

#[repr(C)]
pub struct spmc_arr {
    pub data: *mut u64,
    pub order: u64,
}

pub const SPMC_ARR_BASESZ: usize = 128;
pub const SPMC_ARR_ORDERS: usize = 10;

#[repr(C)]
pub struct spmc {
    pub cur: *mut spmc_arr,
    pub top: u64,
    pub bottom: u64,
    pub arr: [spmc_arr; SPMC_ARR_ORDERS],
}

unsafe extern "C" {
    pub fn spmc_owned_add(spmc: *mut spmc, val: u64) -> ::std::os::raw::c_int;
    pub fn spmc_owned_remove(spmc: *mut spmc, val: *mut u64) -> ::std::os::raw::c_int;
    pub fn spmc_steal(spmc: *mut spmc, val: *mut u64) -> ::std::os::raw::c_int;

    pub fn spmc_create() -> *mut spmc;
    pub fn spmc_destroy(spmc: *mut spmc) -> ::std::os::raw::c_int;
}
