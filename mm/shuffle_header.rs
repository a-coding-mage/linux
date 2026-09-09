// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2018 Intel Corporation. All rights reserved.

// C dependencies: <linux/jump_label.h>, <linux/mmzone.h>

pub const SHUFFLE_ORDER: ::core::ffi::c_int = MAX_PAGE_ORDER;

// The following opaque types and declarations are supplied by other translated
// kernel headers / translation units.
pub enum pg_data_t {}
pub enum zone {}
pub enum StaticKeyFalse {}

extern "C" {
    pub static page_alloc_shuffle_key: StaticKeyFalse;
    pub fn __shuffle_free_memory(pgdat: *mut pg_data_t);
    pub fn shuffle_pick_tail() -> bool;
    pub fn __shuffle_zone(z: *mut zone);
    pub fn static_branch_unlikely(key: *const StaticKeyFalse) -> bool;
}

// CONFIG_SHUFFLE_PAGE_ALLOCATOR selects the implementation below.  The
// configuration condition is preserved here as a Rust cfg condition.
#[cfg(shuffle_page_allocator)]
#[inline]
pub unsafe fn shuffle_free_memory(pgdat: *mut pg_data_t) {
    if !static_branch_unlikely(&page_alloc_shuffle_key) {
        return;
    }
    __shuffle_free_memory(pgdat);
}

#[cfg(shuffle_page_allocator)]
#[inline]
pub unsafe fn shuffle_zone(z: *mut zone) {
    if !static_branch_unlikely(&page_alloc_shuffle_key) {
        return;
    }
    __shuffle_zone(z);
}

#[cfg(shuffle_page_allocator)]
#[inline]
pub unsafe fn is_shuffle_order(order: ::core::ffi::c_int) -> bool {
    if !static_branch_unlikely(&page_alloc_shuffle_key) {
        return false;
    }
    order >= SHUFFLE_ORDER
}

#[cfg(not(shuffle_page_allocator))]
#[inline]
pub unsafe fn shuffle_pick_tail() -> bool {
    false
}

#[cfg(not(shuffle_page_allocator))]
#[inline]
pub unsafe fn shuffle_free_memory(_pgdat: *mut pg_data_t) {}

#[cfg(not(shuffle_page_allocator))]
#[inline]
pub unsafe fn shuffle_zone(_z: *mut zone) {}

#[cfg(not(shuffle_page_allocator))]
#[inline]
pub unsafe fn is_shuffle_order(_order: ::core::ffi::c_int) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
