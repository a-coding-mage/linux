/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::{c_char, c_int};

/* Dependency intent from C header: #include "probe-event.h" */

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strfilter {
    _private: [u8; 0],
}

/* External dependency types supplied by included headers in the original C. */
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_probe_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct probe_trace_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

/* Cache of probe definitions */
#[repr(C)]
pub struct probe_cache_entry {
    pub node: list_head,
    pub sdt: bool,
    pub pev: perf_probe_event,
    pub spev: *mut c_char,
    pub tevlist: *mut strlist,
}

#[repr(C)]
pub struct probe_cache {
    pub fd: c_int,
    pub entries: list_head,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum probe_type {
    PROBE_TYPE_U = 0,
    PROBE_TYPE_S,
    PROBE_TYPE_X,
    PROBE_TYPE_STRING,
    PROBE_TYPE_BITFIELD,
    PROBE_TYPE_END,
}

pub const PF_FL_UPROBE: c_int = 1;
pub const PF_FL_RW: c_int = 2;

/*
 * C macro:
 * for_each_probe_cache_entry(entry, pcache) \
 *     list_for_each_entry(entry, &pcache->entries, node)
 *
 * This depends on the kernel-style list_for_each_entry macro and is preserved
 * as dependency intent because Rust cannot reproduce the C loop header locally
 * without the list implementation.
 */

/* probe-file.c depends on libelf */
/* Original C condition: #ifdef HAVE_LIBELF_SUPPORT */
unsafe extern "C" {
    pub fn open_trace_file(trace_file: *const c_char, readwrite: bool) -> c_int;
    pub fn probe_file__open(flag: c_int) -> c_int;
    pub fn probe_file__open_both(kfd: *mut c_int, ufd: *mut c_int, flag: c_int) -> c_int;
    pub fn probe_file__get_namelist(fd: c_int) -> *mut strlist;
    pub fn probe_file__get_rawlist(fd: c_int) -> *mut strlist;
    pub fn probe_file__add_event(fd: c_int, tev: *mut probe_trace_event) -> c_int;

    pub fn probe_file__get_events(
        fd: c_int,
        filter: *mut strfilter,
        plist: *mut strlist,
    ) -> c_int;
    pub fn probe_file__del_strlist(fd: c_int, namelist: *mut strlist) -> c_int;

    pub fn probe_cache_entry__get_event(
        entry: *mut probe_cache_entry,
        tevs: *mut *mut probe_trace_event,
    ) -> c_int;

    pub fn probe_cache__new(target: *const c_char, nsi: *mut nsinfo) -> *mut probe_cache;
    pub fn probe_cache__add_entry(
        pcache: *mut probe_cache,
        pev: *mut perf_probe_event,
        tevs: *mut probe_trace_event,
        ntevs: c_int,
    ) -> c_int;
    pub fn probe_cache__scan_sdt(pcache: *mut probe_cache, pathname: *const c_char) -> c_int;
    pub fn probe_cache__commit(pcache: *mut probe_cache) -> c_int;
    pub fn probe_cache__purge(pcache: *mut probe_cache);
    pub fn probe_cache__delete(pcache: *mut probe_cache);
    pub fn probe_cache__filter_purge(pcache: *mut probe_cache, filter: *mut strfilter) -> c_int;
    pub fn probe_cache__find(
        pcache: *mut probe_cache,
        pev: *mut perf_probe_event,
    ) -> *mut probe_cache_entry;
    pub fn probe_cache__find_by_name(
        pcache: *mut probe_cache,
        group: *const c_char,
        event: *const c_char,
    ) -> *mut probe_cache_entry;
    pub fn probe_cache__show_all_caches(filter: *mut strfilter) -> c_int;
    pub fn probe_type_is_available(type_: probe_type) -> bool;
    pub fn kretprobe_offset_is_supported() -> bool;
    pub fn uprobe_ref_ctr_is_supported() -> bool;
    pub fn user_access_is_supported() -> bool;
    pub fn multiprobe_event_is_supported() -> bool;
    pub fn immediate_value_is_supported() -> bool;
}

/*
 * Original C fallback under: #else / !HAVE_LIBELF_SUPPORT
 *
 * static inline struct probe_cache *probe_cache__new(
 *     const char *tgt __maybe_unused,
 *     struct nsinfo *nsi __maybe_unused)
 * {
 *     return NULL;
 * }
 *
 * #define probe_cache__delete(pcache) do {} while (0)
 */
#[cfg(not(HAVE_LIBELF_SUPPORT))]
#[allow(non_snake_case)]
pub unsafe fn probe_cache__new(
    _tgt: *const c_char,
    _nsi: *mut nsinfo,
) -> *mut probe_cache {
    std::ptr::null_mut()
}

#[cfg(not(HAVE_LIBELF_SUPPORT))]
#[allow(non_snake_case)]
pub unsafe fn probe_cache__delete(_pcache: *mut probe_cache) {}
