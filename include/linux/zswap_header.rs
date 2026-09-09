/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the Linux type headers are intentionally not
// redefined here: linux/types.h and linux/mm_types.h.

#[repr(C)]
pub struct lruvec {
    _private: [u8; 0],
}

extern "C" {
    pub static mut zswap_stored_pages: atomic_long_t;
}

// The following items are selected by the C build-time CONFIG_ZSWAP condition.
#[cfg(feature = "CONFIG_ZSWAP")]
#[repr(C)]
pub struct zswap_lruvec_state {
    /*
     * Number of swapped in pages from disk, i.e not found in the zswap pool.
     *
     * This is consumed and subtracted from the lru size in
     * zswap_shrinker_count() to penalize past overshrinking that led to disk
     * swapins. The idea is that had we considered this many more pages in the
     * LRU active/protected and not written them back, we would not have had to
     * swapped them in.
     */
    pub nr_disk_swapins: atomic_long_t,
}

#[cfg(feature = "CONFIG_ZSWAP")]
extern "C" {
    pub fn zswap_total_pages() -> c_ulong;
    pub fn zswap_store(folio: *mut folio) -> bool;
    pub fn zswap_load(folio: *mut folio) -> c_int;
    pub fn zswap_invalidate(swp: swp_entry_t);
    pub fn zswap_swapon(type_: c_int, nr_pages: c_ulong) -> c_int;
    pub fn zswap_swapoff(type_: c_int);
    pub fn zswap_memcg_offline_cleanup(memcg: *mut mem_cgroup);
    pub fn zswap_lruvec_state_init(lruvec: *mut lruvec);
    pub fn zswap_folio_swapin(folio: *mut folio);
    pub fn zswap_is_enabled() -> bool;
    pub fn zswap_never_enabled() -> bool;
}

#[cfg(not(feature = "CONFIG_ZSWAP"))]
#[repr(C)]
pub struct zswap_lruvec_state {}

#[cfg(not(feature = "CONFIG_ZSWAP"))]
#[inline]
pub unsafe fn zswap_store(_folio: *mut folio) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_ZSWAP"))]
#[inline]
pub unsafe fn zswap_load(_folio: *mut folio) -> c_int {
    // -ENOENT; errno is supplied by the external Linux dependency.
    -ENOENT
}

#[cfg(not(feature = "CONFIG_ZSWAP"))]
#[inline]
pub unsafe fn zswap_invalidate(_swp: swp_entry_t) {}

#[cfg(not(feature = "CONFIG_ZSWAP"))]
#[inline]
pub unsafe fn zswap_swapon(_type: c_int, _nr_pages: c_ulong) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_ZSWAP"))]
#[inline]
pub unsafe fn zswap_swapoff(_type: c_int) {}

#[cfg(not(feature = "CONFIG_ZSWAP"))]
#[inline]
pub unsafe fn zswap_memcg_offline_cleanup(_memcg: *mut mem_cgroup) {}

#[cfg(not(feature = "CONFIG_ZSWAP"))]
#[inline]
pub unsafe fn zswap_lruvec_state_init(_lruvec: *mut lruvec) {}

#[cfg(not(feature = "CONFIG_ZSWAP"))]
#[inline]
pub unsafe fn zswap_folio_swapin(_folio: *mut folio) {}

#[cfg(not(feature = "CONFIG_ZSWAP"))]
#[inline]
pub unsafe fn zswap_is_enabled() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_ZSWAP"))]
#[inline]
pub unsafe fn zswap_never_enabled() -> bool {
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
