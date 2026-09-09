/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * PCC (Platform Communications Channel) methods
 */

// The Linux mailbox types below are supplied by the corresponding dependency.

#[repr(C)]
pub struct pcc_mbox_chan {
    pub mchan: *mut mbox_chan,
    pub shmem_base_addr: u64,
    pub shmem: *mut core::ffi::c_void,
    pub shmem_size: u64,
    pub latency: u32,
    pub max_access_rate: u32,
    pub min_turnaround_time: u16,
}

/* Generic Communications Channel Shared Memory Region */
pub const PCC_SIGNATURE: u32 = 0x5043_4300;
/* Generic Communications Channel Command Field */
pub const PCC_CMD_GENERATE_DB_INTR: u32 = 1u32 << 15;
/* Generic Communications Channel Status Field */
pub const PCC_STATUS_CMD_COMPLETE: u32 = 1u32 << 0;
pub const PCC_STATUS_SCI_DOORBELL: u32 = 1u32 << 1;
pub const PCC_STATUS_ERROR: u32 = 1u32 << 2;
pub const PCC_STATUS_PLATFORM_NOTIFY: u32 = 1u32 << 3;
/* Initiator Responder Communications Channel Flags */
pub const PCC_CMD_COMPLETION_NOTIFY: u32 = 1u32 << 0;

pub const MAX_PCC_SUBSPACES: usize = 256;

// CONFIG_PCC selects the external implementation; the fallback preserves the
// Linux ERR_PTR(-ENODEV) behavior when PCC support is unavailable.
#[cfg(feature = "CONFIG_PCC")]
unsafe extern "C" {
    pub fn pcc_mbox_request_channel(
        cl: *mut mbox_client,
        subspace_id: core::ffi::c_int,
    ) -> *mut pcc_mbox_chan;
    pub fn pcc_mbox_free_channel(chan: *mut pcc_mbox_chan);
}

#[cfg(not(feature = "CONFIG_PCC"))]
#[inline]
pub unsafe fn pcc_mbox_request_channel(
    _cl: *mut mbox_client,
    _subspace_id: core::ffi::c_int,
) -> *mut pcc_mbox_chan {
    // ERR_PTR(-ENODEV), with ENODEV == 19.
    core::mem::transmute::<isize, *mut pcc_mbox_chan>(-19)
}

#[cfg(not(feature = "CONFIG_PCC"))]
#[inline]
pub unsafe fn pcc_mbox_free_channel(_chan: *mut pcc_mbox_chan) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
