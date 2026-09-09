/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 */

// Dependencies supplied by the corresponding SOF header and stream modules.

/*
 * Firmware boot and version
 */

pub const SOF_IPC_MAX_ELEMS: u32 = 16;

/*
 * Firmware boot info flag bits (64-bit)
 */
pub const SOF_IPC_INFO_BUILD: u64 = 1u64 << 0;
pub const SOF_IPC_INFO_LOCKS: u64 = 1u64 << 1;
pub const SOF_IPC_INFO_LOCKSV: u64 = 1u64 << 2;
pub const SOF_IPC_INFO_GDB: u64 = 1u64 << 3;
pub const SOF_IPC_INFO_D3_PERSISTENT: u64 = 1u64 << 4;

/* extended data types that can be appended onto end of sof_ipc_fw_ready */
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum sof_ipc_ext_data {
    SOF_IPC_EXT_UNUSED = 0,
    SOF_IPC_EXT_WINDOW = 1,
    SOF_IPC_EXT_CC_INFO = 2,
    SOF_IPC_EXT_PROBE_INFO = 3,
    SOF_IPC_EXT_USER_ABI_INFO = 4,
}

/* Build u32 number in format MMmmmppp */
#[inline]
pub const fn SOF_FW_VER(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 24) | (minor << 12) | patch
}

/* FW version - SOF_IPC_GLB_VERSION */
#[repr(C, packed)]
pub struct sof_ipc_fw_version {
    pub hdr: sof_ipc_hdr,
    pub major: u16,
    pub minor: u16,
    pub micro: u16,
    pub build: u16,
    pub date: [u8; 12],
    pub time: [u8; 10],
    pub tag: [u8; 6],
    pub abi_version: u32,
    /* used to check FW and ldc file compatibility, reproducible value */
    pub src_hash: u32,

    /* reserved for future use */
    pub reserved: [u32; 3],
}

/* FW ready Message - sent by firmware when boot has completed */
#[repr(C, packed)]
pub struct sof_ipc_fw_ready {
    pub hdr: sof_ipc_cmd_hdr,
    pub dspbox_offset: u32,  /* dsp initiated IPC mailbox */
    pub hostbox_offset: u32, /* host initiated IPC mailbox */
    pub dspbox_size: u32,
    pub hostbox_size: u32,
    pub version: sof_ipc_fw_version,

    /* Miscellaneous flags */
    pub flags: u64,

    /* reserved for future use */
    pub reserved: [u32; 4],
}

/*
 * Extended Firmware data. All optional, depends on platform/arch.
 */
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum sof_ipc_region {
    SOF_IPC_REGION_DOWNBOX = 0,
    SOF_IPC_REGION_UPBOX,
    SOF_IPC_REGION_TRACE,
    SOF_IPC_REGION_DEBUG,
    SOF_IPC_REGION_STREAM,
    SOF_IPC_REGION_REGS,
    SOF_IPC_REGION_EXCEPTION,
}

#[repr(C, packed)]
pub struct sof_ipc_ext_data_hdr {
    pub hdr: sof_ipc_cmd_hdr,
    pub r#type: u32, /*< SOF_IPC_EXT_ */
}

#[repr(C, packed)]
pub struct sof_ipc_window_elem {
    pub hdr: sof_ipc_hdr,
    pub r#type: u32, /*< SOF_IPC_REGION_ */
    pub id: u32, /*< platform specific - used to map to host memory */
    pub flags: u32, /*< R, W, RW, etc - to define */
    pub size: u32, /*< size of region in bytes */
    /* offset in window region as windows can be partitioned */
    pub offset: u32,
}

/* extended data memory windows for IPC, trace and debug */
#[repr(C, packed)]
pub struct sof_ipc_window {
    pub ext_hdr: sof_ipc_ext_data_hdr,
    pub num_windows: u32,
    pub window: [sof_ipc_window_elem; SOF_IPC_MAX_ELEMS as usize],
}

#[repr(C, packed)]
pub struct sof_ipc_cc_version {
    pub ext_hdr: sof_ipc_ext_data_hdr,
    pub major: u32,
    pub minor: u32,
    pub micro: u32,

    /* reserved for future use */
    pub reserved: [u32; 4],

    pub name: [u8; 16], /* null terminated compiler name */
    pub optim: [u8; 4], /* null terminated compiler -O flag value */
    pub desc: [u8; 32], /* null terminated compiler description */
}

/* extended data: Probe setup */
#[repr(C, packed)]
pub struct sof_ipc_probe_support {
    pub ext_hdr: sof_ipc_ext_data_hdr,

    pub probe_points_max: u32,
    pub injection_dmas_max: u32,

    /* reserved for future use */
    pub reserved: [u32; 2],
}

/* extended data: user abi version(s) */
#[repr(C, packed)]
pub struct sof_ipc_user_abi_version {
    pub ext_hdr: sof_ipc_ext_data_hdr,

    pub abi_dbg_version: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
