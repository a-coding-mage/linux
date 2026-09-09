/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
   lru_cache.c

   This file is part of DRBD by Philipp Reisner and Lars Ellenberg.

   Copyright (C) 2003-2008, LINBIT Information Technologies GmbH.
   Copyright (C) 2003-2008, Philipp Reisner <philipp.reisner@linbit.com>.
   Copyright (C) 2003-2008, Lars Ellenberg <lars.ellenberg@linbit.com>.
*/

/* Dependencies supplied by the Linux kernel headers are intentionally external. */

#[repr(C)]
pub struct lc_element {
    pub collision: hlist_node,
    pub list: list_head,
    pub refcnt: ::core::ffi::c_uint,
    pub lc_index: ::core::ffi::c_uint,
    pub lc_number: ::core::ffi::c_uint,
    pub lc_new_number: ::core::ffi::c_uint,
}

pub const LC_FREE: ::core::ffi::c_uint = !0u32;

#[repr(C)]
pub struct lru_cache {
    pub lru: list_head,
    pub free: list_head,
    pub in_use: list_head,
    pub to_be_changed: list_head,
    pub lc_cache: *mut kmem_cache,
    pub element_size: usize,
    pub element_off: usize,
    pub nr_elements: ::core::ffi::c_uint,
    pub max_pending_changes: ::core::ffi::c_uint,
    pub pending_changes: ::core::ffi::c_uint,
    pub used: ::core::ffi::c_uint,
    pub hits: ::core::ffi::c_ulong,
    pub misses: ::core::ffi::c_ulong,
    pub starving: ::core::ffi::c_ulong,
    pub locked: ::core::ffi::c_ulong,
    pub changed: ::core::ffi::c_ulong,
    pub flags: ::core::ffi::c_ulong,
    pub name: *const ::core::ffi::c_char,
    pub lc_slot: *mut hlist_head,
    pub lc_element: *mut *mut lc_element,
}

pub const LC_MAX_ACTIVE: ::core::ffi::c_uint = 1 << 24;

pub const __LC_PARANOIA: u32 = 0;
pub const __LC_DIRTY: u32 = 1;
pub const __LC_LOCKED: u32 = 2;
pub const __LC_STARVING: u32 = 3;

pub const LC_PARANOIA: ::core::ffi::c_ulong = 1 << __LC_PARANOIA;
pub const LC_DIRTY: ::core::ffi::c_ulong = 1 << __LC_DIRTY;
pub const LC_LOCKED: ::core::ffi::c_ulong = 1 << __LC_LOCKED;
pub const LC_STARVING: ::core::ffi::c_ulong = 1 << __LC_STARVING;

extern "C" {
    pub fn lc_create(
        name: *const ::core::ffi::c_char,
        cache: *mut kmem_cache,
        max_pending_changes: ::core::ffi::c_uint,
        e_count: ::core::ffi::c_uint,
        e_size: usize,
        e_off: usize,
    ) -> *mut lru_cache;
    pub fn lc_reset(lc: *mut lru_cache);
    pub fn lc_destroy(lc: *mut lru_cache);
    pub fn lc_del(lc: *mut lru_cache, element: *mut lc_element);
    pub fn lc_get_cumulative(lc: *mut lru_cache, enr: ::core::ffi::c_uint) -> *mut lc_element;
    pub fn lc_try_get(lc: *mut lru_cache, enr: ::core::ffi::c_uint) -> *mut lc_element;
    pub fn lc_find(lc: *mut lru_cache, enr: ::core::ffi::c_uint) -> *mut lc_element;
    pub fn lc_get(lc: *mut lru_cache, enr: ::core::ffi::c_uint) -> *mut lc_element;
    pub fn lc_put(lc: *mut lru_cache, e: *mut lc_element) -> ::core::ffi::c_uint;
    pub fn lc_committed(lc: *mut lru_cache);
    pub fn lc_seq_printf_stats(seq: *mut seq_file, lc: *mut lru_cache);
    pub fn lc_seq_dump_details(
        seq: *mut seq_file,
        lc: *mut lru_cache,
        utext: *mut ::core::ffi::c_char,
        detail: Option<unsafe extern "C" fn(*mut seq_file, *mut lc_element)>,
    );
    pub fn lc_try_lock(lc: *mut lru_cache) -> ::core::ffi::c_int;
    pub fn lc_element_by_index(lc: *mut lru_cache, i: ::core::ffi::c_uint) -> *mut lc_element;
    pub fn test_and_set_bit(nr: u32, addr: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn clear_bit(nr: u32, addr: *mut ::core::ffi::c_ulong);
    pub fn clear_bit_unlock(nr: u32, addr: *mut ::core::ffi::c_ulong);
}

#[inline]
pub unsafe fn lc_try_lock_for_transaction(lc: *mut lru_cache) -> ::core::ffi::c_int {
    (!test_and_set_bit(__LC_LOCKED, &mut (*lc).flags) != 0) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn lc_unlock(lc: *mut lru_cache) {
    clear_bit(__LC_DIRTY, &mut (*lc).flags);
    clear_bit_unlock(__LC_LOCKED, &mut (*lc).flags);
}

/* The following kernel types are provided by the included Linux headers. */
#[allow(non_camel_case_types)]
pub enum hlist_node {}
#[allow(non_camel_case_types)]
pub enum list_head {}
#[allow(non_camel_case_types)]
pub enum kmem_cache {}
#[allow(non_camel_case_types)]
pub enum hlist_head {}
#[allow(non_camel_case_types)]
pub enum seq_file {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
