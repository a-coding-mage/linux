/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Platform Firmware Runtime Update header
 *
 * Copyright(c) 2021 Intel Corporation. All rights reserved.
 */

// Linux kernel type equivalents.
pub type __u8 = u8;
pub type __u32 = u32;
pub type __u64 = u64;

pub const PFRUT_IOCTL_MAGIC: u32 = 0xEE;

const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT) | (ty << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}

const fn iow(nr: u32, size: u32) -> u32 { ioc(IOC_WRITE, PFRUT_IOCTL_MAGIC, nr, size) }
const fn ior(nr: u32, size: u32) -> u32 { ioc(IOC_READ, PFRUT_IOCTL_MAGIC, nr, size) }

/// Set the Revision ID for Platform Firmware Runtime Update.
pub const PFRU_IOC_SET_REV: u32 = iow(0x01, core::mem::size_of::<u32>() as u32);
/// Stage a capsule image from communication buffer and perform authentication.
pub const PFRU_IOC_STAGE: u32 = iow(0x02, core::mem::size_of::<u32>() as u32);
/// Activate a previously staged capsule image.
pub const PFRU_IOC_ACTIVATE: u32 = iow(0x03, core::mem::size_of::<u32>() as u32);
/// Perform both stage and activation action.
pub const PFRU_IOC_STAGE_ACTIVATE: u32 = iow(0x04, core::mem::size_of::<u32>() as u32);
/// Retrieve information on the Platform Firmware Runtime Update capability.
pub const PFRU_IOC_QUERY_CAP: u32 = ior(0x05, core::mem::size_of::<pfru_update_cap_info>() as u32);

/// Capsule file payload header.
#[repr(C)]
pub struct pfru_payload_hdr {
    pub sig: __u32,
    pub hdr_version: __u32,
    pub hdr_size: __u32,
    pub hw_ver: __u32,
    pub rt_ver: __u32,
    pub platform_id: [__u8; 16],
    pub svn_ver: __u32,
}

#[repr(C)]
#[derive Copy, Clone, Debug, PartialEq, Eq)]
pub enum pfru_dsm_status {
    DSM_SUCCEED = 0,
    DSM_FUNC_NOT_SUPPORT = 1,
    DSM_INVAL_INPUT = 2,
    DSM_HARDWARE_ERR = 3,
    DSM_RETRY_SUGGESTED = 4,
    DSM_UNKNOWN = 5,
    DSM_FUNC_SPEC_ERR = 6,
}

/// Runtime update capability information.
#[repr(C)]
pub struct pfru_update_cap_info {
    pub status: __u32,
    pub update_cap: __u32,
    pub code_type: [__u8; 16],
    pub fw_version: __u32,
    pub code_rt_version: __u32,
    pub drv_type: [__u8; 16],
    pub drv_rt_version: __u32,
    pub drv_svn: __u32,
    pub platform_id: [__u8; 16],
    pub oem_id: [__u8; 16],
    pub oem_info_len: __u32,
}

/// Communication buffer information.
#[repr(C)]
pub struct pfru_com_buf_info {
    pub status: __u32,
    pub ext_status: __u32,
    pub addr_lo: __u64,
    pub addr_hi: __u64,
    pub buf_size: __u32,
}

/// Platform firmware runtime update result information.
#[repr(C)]
pub struct pfru_updated_result {
    pub status: __u32,
    pub ext_status: __u32,
    pub low_auth_time: __u64,
    pub high_auth_time: __u64,
    pub low_exec_time: __u64,
    pub high_exec_time: __u64,
}

/// Log Data from telemetry service.
#[repr(C)]
pub struct pfrt_log_data_info {
    pub status: __u32,
    pub ext_status: __u32,
    pub chunk1_addr_lo: __u64,
    pub chunk1_addr_hi: __u64,
    pub chunk2_addr_lo: __u64,
    pub chunk2_addr_hi: __u64,
    pub max_data_size: __u32,
    pub chunk1_size: __u32,
    pub chunk2_size: __u32,
    pub rollover_cnt: __u32,
    pub reset_cnt: __u32,
}

/// Telemetry log information.
#[repr(C)]
pub struct pfrt_log_info {
    pub log_level: __u32,
    pub log_type: __u32,
    pub log_revid: __u32,
}

/// Set the PFRT log level and log type.
pub const PFRT_LOG_IOC_SET_INFO: u32 = iow(0x06, core::mem::size_of::<pfrt_log_info>() as u32);
/// Retrieve log level and log type of the telemetry.
pub const PFRT_LOG_IOC_GET_INFO: u32 = ior(0x07, core::mem::size_of::<pfrt_log_info>() as u32);
/// Retrieve data information about the telemetry.
pub const PFRT_LOG_IOC_GET_DATA_INFO: u32 = ior(0x08, core::mem::size_of::<pfrt_log_data_info>() as u32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
