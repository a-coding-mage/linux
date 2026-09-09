/* SPDX-License-Identifier: GPL-2.0 */
/* TRACE_SYSTEM vmalloc */
/* The C tracepoint definitions and trace/define_trace.h are supplied externally. */

use core::ffi::{c_int, c_uint, c_ulong};

/**
 * alloc_vmap_area - called when a new vmap allocation occurs
 * @addr: an allocated address
 * @size: a requested size
 * @align: a requested alignment
 * @vstart: a requested start range
 * @vend: a requested end range
 * @failed: an allocation failed or not
 *
 * This event is used for a debug purpose, it can give an extra
 * information for a developer about how often it occurs and which
 * parameters are passed for further validation.
 */
#[repr(C)]
pub struct AllocVmapAreaEntry {
    pub addr: c_ulong,
    pub size: c_ulong,
    pub align: c_ulong,
    pub vstart: c_ulong,
    pub vend: c_ulong,
    pub failed: c_int,
}

pub const ALLOC_VMAP_AREA_PRINTK: &str =
    "va_start: %lu size=%lu align=%lu vstart=0x%lx vend=0x%lx failed=%d";

#[inline]
pub unsafe fn alloc_vmap_area_fast_assign(
    entry: *mut AllocVmapAreaEntry,
    addr: c_ulong,
    size: c_ulong,
    align: c_ulong,
    vstart: c_ulong,
    vend: c_ulong,
    failed: c_int,
) {
    (*entry).addr = addr;
    (*entry).size = size;
    (*entry).align = align;
    (*entry).vstart = vstart;
    (*entry).vend = vend;
    (*entry).failed = failed;
}

/**
 * purge_vmap_area_lazy - called when vmap areas were lazily freed
 * @start: purging start address
 * @end: purging end address
 * @npurged: numbed of purged vmap areas
 *
 * This event is used for a debug purpose. It gives some
 * indication about start:end range and how many objects
 * are released.
 */
#[repr(C)]
pub struct PurgeVmapAreaLazyEntry {
    pub start: c_ulong,
    pub end: c_ulong,
    pub npurged: c_uint,
}

pub const PURGE_VMAP_AREA_LAZY_PRINTK: &str = "start=0x%lx end=0x%lx num_purged=%u";

#[inline]
pub unsafe fn purge_vmap_area_lazy_fast_assign(
    entry: *mut PurgeVmapAreaLazyEntry,
    start: c_ulong,
    end: c_ulong,
    npurged: c_uint,
) {
    (*entry).start = start;
    (*entry).end = end;
    (*entry).npurged = npurged;
}

/**
 * free_vmap_area_noflush - called when a vmap area is freed
 * @va_start: a start address of VA
 * @nr_lazy: number of current lazy pages
 * @nr_lazy_max: number of maximum lazy pages
 *
 * This event is used for a debug purpose. It gives some
 * indication about a VA that is released, number of current
 * outstanding areas and a maximum allowed threshold before
 * dropping all of them.
 */
#[repr(C)]
pub struct FreeVmapAreaNoflushEntry {
    pub va_start: c_ulong,
    pub nr_lazy: c_ulong,
    pub nr_lazy_max: c_ulong,
}

pub const FREE_VMAP_AREA_NOFLUSH_PRINTK: &str = "va_start=0x%lx nr_lazy=%lu nr_lazy_max=%lu";

#[inline]
pub unsafe fn free_vmap_area_noflush_fast_assign(
    entry: *mut FreeVmapAreaNoflushEntry,
    va_start: c_ulong,
    nr_lazy: c_ulong,
    nr_lazy_max: c_ulong,
) {
    (*entry).va_start = va_start;
    (*entry).nr_lazy = nr_lazy;
    (*entry).nr_lazy_max = nr_lazy_max;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
