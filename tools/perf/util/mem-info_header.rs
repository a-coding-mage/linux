/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/mem-info.h. */
/* C includes omitted:
 * - <linux/refcount.h>
 * - <linux/perf_event.h>
 * - <internal/rc_check.h>
 * - "map_symbol.h"
 */

use crate::{addr_map_symbol, perf_mem_data_src, refcount_t};

/* DECLARE_RC_STRUCT(mem_info) */
#[repr(C)]
pub struct mem_info {
    pub iaddr: addr_map_symbol,
    pub daddr: addr_map_symbol,
    pub data_src: perf_mem_data_src,
    pub refcnt: refcount_t,
}

extern "C" {
    pub fn mem_info__new() -> *mut mem_info;
    pub fn mem_info__clone(mi: *mut mem_info) -> *mut mem_info;
    pub fn mem_info__get(mi: *mut mem_info) -> *mut mem_info;
    pub fn mem_info__put(mi: *mut mem_info);
}

#[inline]
pub unsafe fn __mem_info__zput(mi: *mut *mut mem_info) {
    mem_info__put(*mi);
    *mi = core::ptr::null_mut();
}

/* C macro: #define mem_info__zput(mi) __mem_info__zput(&mi) */
#[inline]
pub unsafe fn mem_info__zput(mi: *mut *mut mem_info) {
    __mem_info__zput(mi);
}

#[inline]
pub unsafe fn mem_info__iaddr(mi: *mut mem_info) -> *mut addr_map_symbol {
    /* Original C uses RC_CHK_ACCESS(mi)->iaddr. */
    &mut (*mi).iaddr
}

#[inline]
pub unsafe fn mem_info__daddr(mi: *mut mem_info) -> *mut addr_map_symbol {
    /* Original C uses RC_CHK_ACCESS(mi)->daddr. */
    &mut (*mi).daddr
}

#[inline]
pub unsafe fn mem_info__data_src(mi: *mut mem_info) -> *mut perf_mem_data_src {
    /* Original C uses RC_CHK_ACCESS(mi)->data_src. */
    &mut (*mi).data_src
}

#[inline]
pub unsafe fn mem_info__const_data_src(mi: *const mem_info) -> *const perf_mem_data_src {
    /* Original C uses RC_CHK_ACCESS(mi)->data_src. */
    &(*mi).data_src
}

#[inline]
pub unsafe fn mem_info__refcnt(mi: *mut mem_info) -> *mut refcount_t {
    /* Original C uses RC_CHK_ACCESS(mi)->refcnt. */
    &mut (*mi).refcnt
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
