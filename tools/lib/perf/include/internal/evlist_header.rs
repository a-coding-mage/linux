/* SPDX-License-Identifier: GPL-2.0 */

/* Bindings translated from internal/evlist.h.
 *
 * C include dependencies removed from executable Rust:
 * - <linux/list.h>
 * - <api/fd/array.h>
 * - <internal/cpumap.h>
 * - <internal/evsel.h>
 */

use core::ffi::c_void;
use core::mem::MaybeUninit;
use std::os::raw::{c_int, c_short};

pub const PERF_EVLIST__HLIST_BITS: usize = 8;
pub const PERF_EVLIST__HLIST_SIZE: usize = 1usize << PERF_EVLIST__HLIST_BITS;

/* Forward declarations / dependency types supplied by other translated files. */
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hlist_head {
    pub first: *mut c_void,
}

#[repr(C)]
pub struct fdarray {
    _data: MaybeUninit<[u8; 0]>,
}

#[repr(C)]
pub struct perf_cpu_map {
    _data: MaybeUninit<[u8; 0]>,
}

#[repr(C)]
pub struct perf_thread_map {
    _data: MaybeUninit<[u8; 0]>,
}

#[repr(C)]
pub struct perf_mmap_param {
    _data: MaybeUninit<[u8; 0]>,
}

#[repr(C)]
pub struct perf_evsel {
    _data: MaybeUninit<[u8; 0]>,
}

#[repr(C)]
pub struct perf_mmap {
    _data: MaybeUninit<[u8; 0]>,
}

#[repr(C)]
pub struct perf_cpu {
    _data: MaybeUninit<[u8; 0]>,
}

/* enum fdarray_flags is supplied by <api/fd/array.h>. */
pub type fdarray_flags = c_int;

pub type size_t = usize;
pub type u64 = u64;

#[repr(C)]
pub struct perf_evlist {
    pub entries: list_head,
    pub nr_entries: c_int,
    pub has_user_cpus: bool,
    pub needs_map_propagation: bool,
    /**
     * The cpus passed from the command line or all online CPUs by
     * default.
     */
    pub user_requested_cpus: *mut perf_cpu_map,
    /** The union of all evsel cpu maps. */
    pub all_cpus: *mut perf_cpu_map,
    pub threads: *mut perf_thread_map,
    pub nr_mmaps: c_int,
    pub mmap_len: size_t,
    pub pollfd: fdarray,
    pub heads: [hlist_head; PERF_EVLIST__HLIST_SIZE],
    pub mmap: *mut perf_mmap,
    pub mmap_ovw: *mut perf_mmap,
    pub mmap_first: *mut perf_mmap,
    pub mmap_ovw_first: *mut perf_mmap,
}

pub type perf_evlist_mmap__cb_idx_t = Option<
    unsafe extern "C" fn(
        *mut perf_evlist,
        *mut perf_evsel,
        *mut perf_mmap_param,
        c_int,
    ),
>;

pub type perf_evlist_mmap__cb_get_t =
    Option<unsafe extern "C" fn(*mut perf_evlist, bool, c_int) -> *mut perf_mmap>;

pub type perf_evlist_mmap__cb_mmap_t = Option<
    unsafe extern "C" fn(*mut perf_mmap, *mut perf_mmap_param, c_int, perf_cpu) -> c_int,
>;

#[repr(C)]
pub struct perf_evlist_mmap_ops {
    pub idx: perf_evlist_mmap__cb_idx_t,
    pub get: perf_evlist_mmap__cb_get_t,
    pub mmap: perf_evlist_mmap__cb_mmap_t,
}

extern "C" {
    pub fn perf_evlist__alloc_pollfd(evlist: *mut perf_evlist) -> c_int;
    pub fn perf_evlist__add_pollfd(
        evlist: *mut perf_evlist,
        fd: c_int,
        ptr: *mut c_void,
        revent: c_short,
        flags: fdarray_flags,
    ) -> c_int;

    pub fn perf_evlist__mmap_ops(
        evlist: *mut perf_evlist,
        ops: *mut perf_evlist_mmap_ops,
        mp: *mut perf_mmap_param,
    ) -> c_int;

    pub fn perf_evlist__init(evlist: *mut perf_evlist);
    pub fn perf_evlist__exit(evlist: *mut perf_evlist);
}

/**
 * __perf_evlist__for_each_entry - iterate thru all the evsels
 * @list: list_head instance to iterate
 * @evsel: struct perf_evsel iterator
 */
/* C macro:
 * #define __perf_evlist__for_each_entry(list, evsel) \
 *      list_for_each_entry(evsel, list, node)
 */

/**
 * evlist__for_each_entry - iterate thru all the evsels
 * @evlist: perf_evlist instance to iterate
 * @evsel: struct perf_evsel iterator
 */
/* C macro:
 * #define perf_evlist__for_each_entry(evlist, evsel) \
 *      __perf_evlist__for_each_entry(&(evlist)->entries, evsel)
 */

/**
 * __perf_evlist__for_each_entry_reverse - iterate thru all the evsels in reverse order
 * @list: list_head instance to iterate
 * @evsel: struct evsel iterator
 */
/* C macro:
 * #define __perf_evlist__for_each_entry_reverse(list, evsel) \
 *      list_for_each_entry_reverse(evsel, list, node)
 */

/**
 * perf_evlist__for_each_entry_reverse - iterate thru all the evsels in reverse order
 * @evlist: evlist instance to iterate
 * @evsel: struct evsel iterator
 */
/* C macro:
 * #define perf_evlist__for_each_entry_reverse(evlist, evsel) \
 *      __perf_evlist__for_each_entry_reverse(&(evlist)->entries, evsel)
 */

/**
 * __perf_evlist__for_each_entry_safe - safely iterate thru all the evsels
 * @list: list_head instance to iterate
 * @tmp: struct evsel temp iterator
 * @evsel: struct evsel iterator
 */
/* C macro:
 * #define __perf_evlist__for_each_entry_safe(list, tmp, evsel) \
 *      list_for_each_entry_safe(evsel, tmp, list, node)
 */

/**
 * perf_evlist__for_each_entry_safe - safely iterate thru all the evsels
 * @evlist: evlist instance to iterate
 * @evsel: struct evsel iterator
 * @tmp: struct evsel temp iterator
 */
/* C macro:
 * #define perf_evlist__for_each_entry_safe(evlist, tmp, evsel) \
 *      __perf_evlist__for_each_entry_safe(&(evlist)->entries, tmp, evsel)
 */

extern "C" {
    /* list_entry is supplied by <linux/list.h>; these preserve the inline helper semantics. */
    fn list_entry_perf_evsel_node(ptr: *mut list_head) -> *mut perf_evsel;
}

pub unsafe fn perf_evlist__first(evlist: *mut perf_evlist) -> *mut perf_evsel {
    unsafe { list_entry_perf_evsel_node((*evlist).entries.next) }
}

pub unsafe fn perf_evlist__last(evlist: *mut perf_evlist) -> *mut perf_evsel {
    unsafe { list_entry_perf_evsel_node((*evlist).entries.prev) }
}

extern "C" {
    pub fn perf_evlist__read_format(evlist: *mut perf_evlist) -> u64;

    pub fn perf_evlist__id_add(
        evlist: *mut perf_evlist,
        evsel: *mut perf_evsel,
        cpu_map_idx: c_int,
        thread: c_int,
        id: u64,
    );

    pub fn perf_evlist__id_add_fd(
        evlist: *mut perf_evlist,
        evsel: *mut perf_evsel,
        cpu_map_idx: c_int,
        thread: c_int,
        fd: c_int,
    ) -> c_int;

    pub fn perf_evlist__reset_id_hash(evlist: *mut perf_evlist);

    pub fn __perf_evlist__set_leader(list: *mut list_head, leader: *mut perf_evsel);

    pub fn perf_evlist__go_system_wide(evlist: *mut perf_evlist, evsel: *mut perf_evsel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
