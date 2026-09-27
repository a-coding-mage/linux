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
    pub refcnt: ::kernel::ffi::c_uint,
    pub lc_index: ::kernel::ffi::c_uint,
    pub lc_number: ::kernel::ffi::c_uint,
    pub lc_new_number: ::kernel::ffi::c_uint,
}

pub const LC_FREE: ::kernel::ffi::c_uint = !0u32;

#[repr(C)]
pub struct lru_cache {
    pub lru: list_head,
    pub free: list_head,
    pub in_use: list_head,
    pub to_be_changed: list_head,
    pub lc_cache: *mut kmem_cache,
    pub element_size: usize,
    pub element_off: usize,
    pub nr_elements: ::kernel::ffi::c_uint,
    pub max_pending_changes: ::kernel::ffi::c_uint,
    pub pending_changes: ::kernel::ffi::c_uint,
    pub used: ::kernel::ffi::c_uint,
    pub hits: ::kernel::ffi::c_ulong,
    pub misses: ::kernel::ffi::c_ulong,
    pub starving: ::kernel::ffi::c_ulong,
    pub locked: ::kernel::ffi::c_ulong,
    pub changed: ::kernel::ffi::c_ulong,
    pub flags: ::kernel::ffi::c_ulong,
    pub name: *const ::kernel::ffi::c_char,
    pub lc_slot: *mut hlist_head,
    pub lc_element: *mut *mut lc_element,
}

pub const LC_MAX_ACTIVE: ::kernel::ffi::c_uint = 1 << 24;

pub const __LC_PARANOIA: u32 = 0;
pub const __LC_DIRTY: u32 = 1;
pub const __LC_LOCKED: u32 = 2;
pub const __LC_STARVING: u32 = 3;

pub const LC_PARANOIA: ::kernel::ffi::c_ulong = 1 << __LC_PARANOIA;
pub const LC_DIRTY: ::kernel::ffi::c_ulong = 1 << __LC_DIRTY;
pub const LC_LOCKED: ::kernel::ffi::c_ulong = 1 << __LC_LOCKED;
pub const LC_STARVING: ::kernel::ffi::c_ulong = 1 << __LC_STARVING;

extern "C" {
    pub fn lc_create(
        name: *const ::kernel::ffi::c_char,
        cache: *mut kmem_cache,
        max_pending_changes: ::kernel::ffi::c_uint,
        e_count: ::kernel::ffi::c_uint,
        e_size: usize,
        e_off: usize,
    ) -> *mut lru_cache;
    pub fn lc_reset(lc: *mut lru_cache);
    pub fn lc_destroy(lc: *mut lru_cache);
    pub fn lc_del(lc: *mut lru_cache, element: *mut lc_element);
    pub fn lc_get_cumulative(lc: *mut lru_cache, enr: ::kernel::ffi::c_uint) -> *mut lc_element;
    pub fn lc_try_get(lc: *mut lru_cache, enr: ::kernel::ffi::c_uint) -> *mut lc_element;
    pub fn lc_find(lc: *mut lru_cache, enr: ::kernel::ffi::c_uint) -> *mut lc_element;
    pub fn lc_get(lc: *mut lru_cache, enr: ::kernel::ffi::c_uint) -> *mut lc_element;
    pub fn lc_put(lc: *mut lru_cache, e: *mut lc_element) -> ::kernel::ffi::c_uint;
    pub fn lc_committed(lc: *mut lru_cache);
    pub fn lc_seq_printf_stats(seq: *mut seq_file, lc: *mut lru_cache);
    pub fn lc_seq_dump_details(
        seq: *mut seq_file,
        lc: *mut lru_cache,
        utext: *mut ::kernel::ffi::c_char,
        detail: Option<unsafe extern "C" fn(*mut seq_file, *mut lc_element)>,
    );
    pub fn lc_try_lock(lc: *mut lru_cache) -> ::kernel::ffi::c_int;
    pub fn lc_element_by_index(lc: *mut lru_cache, i: ::kernel::ffi::c_uint) -> *mut lc_element;
    pub fn test_and_set_bit(nr: u32, addr: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn clear_bit(nr: u32, addr: *mut ::kernel::ffi::c_ulong);
    pub fn clear_bit_unlock(nr: u32, addr: *mut ::kernel::ffi::c_ulong);
}

#[inline]
pub unsafe fn lc_try_lock_for_transaction(lc: *mut lru_cache) -> ::kernel::ffi::c_int {
    (!test_and_set_bit(__LC_LOCKED, &mut (*lc).flags) != 0) as ::kernel::ffi::c_int
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
