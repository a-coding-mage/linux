/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  dibs loopback (aka loopback-ism) device structure definitions.
 *
 *  Copyright (c) 2024, Alibaba Inc.
 *
 *  Author: Wen Gu <guwen@linux.alibaba.com>
 *          Tony Lu <tonylu@linux.alibaba.com>
 *
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/dibs.h, linux/hashtable.h, linux/spinlock.h, linux/types.h,
// and linux/wait.h.

#[cfg(feature = "CONFIG_DIBS_LO")]
pub const DIBS_LO_DMBS_HASH_BITS: usize = 12;
#[cfg(feature = "CONFIG_DIBS_LO")]
pub const DIBS_LO_MAX_DMBS: usize = 5000;

#[cfg(feature = "CONFIG_DIBS_LO")]
#[repr(C)]
pub struct dibs_lo_dmb_node {
    pub list: hlist_node,
    pub token: u64,
    pub len: u32,
    pub sba_idx: u32,
    pub cpu_addr: *mut core::ffi::c_void,
    pub dma_addr: dma_addr_t,
    pub refcnt: refcount_t,
}

#[cfg(feature = "CONFIG_DIBS_LO")]
#[repr(C)]
pub struct dibs_lo_dev {
    pub dibs: *mut dibs_dev,
    pub dmb_cnt: atomic_t,
    pub dmb_ht_lock: rwlock_t,
    // DECLARE_BITMAP(sba_idx_mask, DIBS_LO_MAX_DMBS)
    pub sba_idx_mask: [usize; (DIBS_LO_MAX_DMBS + usize::BITS as usize - 1)
        / usize::BITS as usize],
    // DECLARE_HASHTABLE(dmb_ht, DIBS_LO_DMBS_HASH_BITS)
    pub dmb_ht: [hlist_head; 1usize << DIBS_LO_DMBS_HASH_BITS],
    pub ldev_release: wait_queue_head_t,
}

#[cfg(feature = "CONFIG_DIBS_LO")]
unsafe extern "C" {
    pub fn dibs_loopback_init() -> core::ffi::c_int;
    pub fn dibs_loopback_exit();
}

#[cfg(not(feature = "CONFIG_DIBS_LO"))]
#[inline]
pub const fn dibs_loopback_init() -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_DIBS_LO"))]
#[inline]
pub const fn dibs_loopback_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
