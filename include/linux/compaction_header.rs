/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from linux/compaction.h. C preprocessor configuration is
 * represented with Rust cfg feature names where applicable. */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum compact_priority {
    COMPACT_PRIO_SYNC_FULL = 0,
    MIN_COMPACT_PRIORITY = 0,
    COMPACT_PRIO_SYNC_LIGHT = 1,
    MIN_COMPACT_COSTLY_PRIORITY = 1,
    DEF_COMPACT_PRIORITY = 1,
    COMPACT_PRIO_ASYNC = 2,
    INIT_COMPACT_PRIORITY = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum compact_result {
    /* For more detailed tracepoint output - internal to compaction */
    COMPACT_NOT_SUITABLE_ZONE = 0,
    /* compaction didn't start as it was not possible or direct reclaim
     * was more suitable */
    COMPACT_SKIPPED,
    /* compaction didn't start as it was deferred due to past failures */
    COMPACT_DEFERRED,
    /* For more detailed tracepoint output - internal to compaction */
    COMPACT_NO_SUITABLE_PAGE,
    /* compaction should continue to another pageblock */
    COMPACT_CONTINUE,
    /* The full zone was compacted scanned but wasn't successful to compact
     * suitable pages. */
    COMPACT_COMPLETE,
    /* direct compaction has scanned part of the zone but wasn't successful
     * to compact suitable pages. */
    COMPACT_PARTIAL_SKIPPED,
    /* compaction terminated prematurely due to lock contentions */
    COMPACT_CONTENDED,
    /* direct compaction terminated after concluding that the allocation
     * should now succeed */
    COMPACT_SUCCESS,
}

/* in mm/internal.h */
#[repr(C)]
pub struct alloc_context {
    _private: [u8; 0],
}
/* in mm/internal.h */
#[repr(C)]
pub struct capture_control {
    _private: [u8; 0],
}

/* Number of free order-0 pages that should be available above given watermark
 * to make sure compaction has reasonable chance of not running out of free
 * pages that it needs to isolate as migration target during its work. */
#[inline]
pub fn compact_gap(order: core::ffi::c_uint) -> core::ffi::c_ulong {
    /* Although all the isolations for migration are temporary, compaction
     * free scanner may have up to 1 << order pages on its list and then
     * try to split an (order - 1) free page. At that point, a gap of
     * 1 << order might not be enough, so it's safer to require twice that
     * amount. Note that the number of pages on the list is also
     * effectively limited by COMPACT_CLUSTER_MAX, as that's the maximum
     * that the migrate scanner can have isolated on migrate list, and free
     * scanner is only invoked when the number of isolated free pages is
     * lower than that. */
    unsafe {
        core::cmp::min(2u64.wrapping_shl(order), COMPACT_CLUSTER_MAX as u64) as core::ffi::c_ulong
    }
}

#[inline]
pub unsafe fn current_is_kcompactd() -> core::ffi::c_int {
    (*current).flags & PF_KCOMPACTD
}

#[cfg(feature = "CONFIG_COMPACTION")]
extern "C" {
    pub fn extfrag_for_order(zone: *mut zone, order: core::ffi::c_uint) -> core::ffi::c_uint;
    pub fn fragmentation_index(zone: *mut zone, order: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn try_to_compact_pages(
        gfp_mask: gfp_t,
        order: core::ffi::c_uint,
        alloc_flags: core::ffi::c_uint,
        ac: *const alloc_context,
        prio: compact_priority,
        capc: *mut capture_control,
    ) -> compact_result;
    pub fn reset_isolation_suitable(pgdat: *mut pg_data_t);
    pub fn compaction_suitable(
        zone: *mut zone,
        order: core::ffi::c_int,
        watermark: core::ffi::c_ulong,
        highest_zoneidx: core::ffi::c_int,
    ) -> bool;
    pub fn compaction_defer_reset(
        zone: *mut zone,
        order: core::ffi::c_int,
        alloc_success: bool,
    );
    pub fn compaction_zonelist_suitable(
        ac: *mut alloc_context,
        order: core::ffi::c_int,
        alloc_flags: core::ffi::c_int,
        gfp_mask: gfp_t,
    ) -> bool;
    pub fn kcompactd_run(nid: core::ffi::c_int);
    pub fn kcompactd_stop(nid: core::ffi::c_int);
    pub fn wakeup_kcompactd(pgdat: *mut pg_data_t, order: core::ffi::c_int, highest_zoneidx: core::ffi::c_int);
}

#[cfg(not(feature = "CONFIG_COMPACTION"))]
#[inline]
pub unsafe fn reset_isolation_suitable(_pgdat: *mut pg_data_t) {}

#[cfg(not(feature = "CONFIG_COMPACTION"))]
#[inline]
pub unsafe fn compaction_suitable(_zone: *mut zone, _order: core::ffi::c_int, _watermark: core::ffi::c_ulong, _highest_zoneidx: core::ffi::c_int) -> bool { false }

#[cfg(not(feature = "CONFIG_COMPACTION"))]
#[inline]
pub unsafe fn kcompactd_run(_nid: core::ffi::c_int) {}

#[cfg(not(feature = "CONFIG_COMPACTION"))]
#[inline]
pub unsafe fn kcompactd_stop(_nid: core::ffi::c_int) {}

#[cfg(not(feature = "CONFIG_COMPACTION"))]
#[inline]
pub unsafe fn wakeup_kcompactd(_pgdat: *mut pg_data_t, _order: core::ffi::c_int, _highest_zoneidx: core::ffi::c_int) {}

#[repr(C)]
pub struct node {
    _private: [u8; 0],
}

#[cfg(all(feature = "CONFIG_COMPACTION", feature = "CONFIG_SYSFS", feature = "CONFIG_NUMA"))]
extern "C" {
    pub fn compaction_register_node(node: *mut node) -> core::ffi::c_int;
    pub fn compaction_unregister_node(node: *mut node);
}

#[cfg(not(all(feature = "CONFIG_COMPACTION", feature = "CONFIG_SYSFS", feature = "CONFIG_NUMA")))]
#[inline]
pub unsafe fn compaction_register_node(_node: *mut node) -> core::ffi::c_int { 0 }

#[cfg(not(all(feature = "CONFIG_COMPACTION", feature = "CONFIG_SYSFS", feature = "CONFIG_NUMA")))]
#[inline]
pub unsafe fn compaction_unregister_node(_node: *mut node) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
