/* SPDX-License-Identifier: GPL-2.0 */

// Translated from blk-ioprio.h.
// The Linux kconfig include supplies CONFIG_BLK_CGROUP_IOPRIO at build time.

#[repr(C)]
pub struct request_queue {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bio {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_BLK_CGROUP_IOPRIO")]
unsafe extern "C" {
    pub fn blkcg_set_ioprio(bio: *mut bio);
}

#[cfg(not(feature = "CONFIG_BLK_CGROUP_IOPRIO"))]
#[inline]
pub unsafe fn blkcg_set_ioprio(_bio: *mut bio) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
