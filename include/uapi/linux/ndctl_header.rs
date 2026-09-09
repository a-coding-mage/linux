/*
 * Copyright (c) 2014-2016, Intel Corporation.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms and conditions of the GNU Lesser General Public License,
 * version 2.1, as published by the Free Software Foundation.
 */

use core::ffi::c_char;

#[repr(C, packed)]
pub struct nd_cmd_dimm_flags { pub status: u32, pub flags: u32 }

#[repr(C, packed)]
pub struct nd_cmd_get_config_size { pub status: u32, pub config_size: u32, pub max_xfer: u32 }

#[repr(C, packed)]
pub struct nd_cmd_get_config_data_hdr { pub in_offset: u32, pub in_length: u32, pub status: u32, pub out_buf: [u8; 0] }

#[repr(C, packed)]
pub struct nd_cmd_set_config_hdr { pub in_offset: u32, pub in_length: u32, pub in_buf: [u8; 0] }

#[repr(C, packed)]
pub struct nd_cmd_vendor_hdr { pub opcode: u32, pub in_length: u32, pub in_buf: [u8; 0] }

#[repr(C, packed)]
pub struct nd_cmd_vendor_tail { pub status: u32, pub out_length: u32, pub out_buf: [u8; 0] }

#[repr(C, packed)]
pub struct nd_cmd_ars_cap {
    pub address: u64, pub length: u64, pub status: u32, pub max_ars_out: u32,
    pub clear_err_unit: u32, pub flags: u16, pub reserved: u16,
}

#[repr(C, packed)]
pub struct nd_cmd_ars_start {
    pub address: u64, pub length: u64, pub type_: u16, pub flags: u8,
    pub reserved: [u8; 5], pub status: u32, pub scrub_time: u32,
}

#[repr(C, packed)]
pub struct nd_ars_record {
    pub handle: u32, pub reserved: u32, pub err_address: u64, pub length: u64,
}

#[repr(C, packed)]
pub struct nd_cmd_ars_status {
    pub status: u32, pub out_length: u32, pub address: u64, pub length: u64,
    pub restart_address: u64, pub restart_length: u64, pub type_: u16,
    pub flags: u16, pub num_records: u32, pub records: [nd_ars_record; 0],
}

#[repr(C, packed)]
pub struct nd_cmd_clear_error {
    pub address: u64, pub length: u64, pub status: u32, pub reserved: [u8; 4],
    pub cleared: u64,
}

pub const ND_CMD_IMPLEMENTED: u32 = 0;
pub const ND_CMD_ARS_CAP: u32 = 1;
pub const ND_CMD_ARS_START: u32 = 2;
pub const ND_CMD_ARS_STATUS: u32 = 3;
pub const ND_CMD_CLEAR_ERROR: u32 = 4;
pub const ND_CMD_SMART: u32 = 1;
pub const ND_CMD_SMART_THRESHOLD: u32 = 2;
pub const ND_CMD_DIMM_FLAGS: u32 = 3;
pub const ND_CMD_GET_CONFIG_SIZE: u32 = 4;
pub const ND_CMD_GET_CONFIG_DATA: u32 = 5;
pub const ND_CMD_SET_CONFIG_DATA: u32 = 6;
pub const ND_CMD_VENDOR_EFFECT_LOG_SIZE: u32 = 7;
pub const ND_CMD_VENDOR_EFFECT_LOG: u32 = 8;
pub const ND_CMD_VENDOR: u32 = 9;
pub const ND_CMD_CALL: u32 = 10;

pub const ND_ARS_VOLATILE: u32 = 1;
pub const ND_ARS_PERSISTENT: u32 = 2;
pub const ND_ARS_RETURN_PREV_DATA: u32 = 1 << 1;
pub const ND_CONFIG_LOCKED: u32 = 1;

#[inline]
pub fn nvdimm_bus_cmd_name(cmd: u32) -> *const c_char {
    match cmd {
        ND_CMD_ARS_CAP => b"ars_cap\0".as_ptr() as *const c_char,
        ND_CMD_ARS_START => b"ars_start\0".as_ptr() as *const c_char,
        ND_CMD_ARS_STATUS => b"ars_status\0".as_ptr() as *const c_char,
        ND_CMD_CLEAR_ERROR => b"clear_error\0".as_ptr() as *const c_char,
        ND_CMD_CALL => b"cmd_call\0".as_ptr() as *const c_char,
        _ => b"unknown\0".as_ptr() as *const c_char,
    }
}

#[inline]
pub fn nvdimm_cmd_name(cmd: u32) -> *const c_char {
    match cmd {
        ND_CMD_SMART => b"smart\0".as_ptr() as *const c_char,
        ND_CMD_SMART_THRESHOLD => b"smart_thresh\0".as_ptr() as *const c_char,
        ND_CMD_DIMM_FLAGS => b"flags\0".as_ptr() as *const c_char,
        ND_CMD_GET_CONFIG_SIZE => b"get_size\0".as_ptr() as *const c_char,
        ND_CMD_GET_CONFIG_DATA => b"get_data\0".as_ptr() as *const c_char,
        ND_CMD_SET_CONFIG_DATA => b"set_data\0".as_ptr() as *const c_char,
        ND_CMD_VENDOR_EFFECT_LOG_SIZE => b"effect_size\0".as_ptr() as *const c_char,
        ND_CMD_VENDOR_EFFECT_LOG => b"effect_log\0".as_ptr() as *const c_char,
        ND_CMD_VENDOR => b"vendor\0".as_ptr() as *const c_char,
        ND_CMD_CALL => b"cmd_call\0".as_ptr() as *const c_char,
        _ => b"unknown\0".as_ptr() as *const c_char,
    }
}

pub const ND_IOCTL: u8 = b'N';
// _IOWR is supplied by the target ioctl dependency; these preserve the source macros.
pub const ND_IOCTL_DIMM_FLAGS: usize = _IOWR!(ND_IOCTL, ND_CMD_DIMM_FLAGS, nd_cmd_dimm_flags);
pub const ND_IOCTL_GET_CONFIG_SIZE: usize = _IOWR!(ND_IOCTL, ND_CMD_GET_CONFIG_SIZE, nd_cmd_get_config_size);
pub const ND_IOCTL_GET_CONFIG_DATA: usize = _IOWR!(ND_IOCTL, ND_CMD_GET_CONFIG_DATA, nd_cmd_get_config_data_hdr);
pub const ND_IOCTL_SET_CONFIG_DATA: usize = _IOWR!(ND_IOCTL, ND_CMD_SET_CONFIG_DATA, nd_cmd_set_config_hdr);
pub const ND_IOCTL_VENDOR: usize = _IOWR!(ND_IOCTL, ND_CMD_VENDOR, nd_cmd_vendor_hdr);
pub const ND_IOCTL_ARS_CAP: usize = _IOWR!(ND_IOCTL, ND_CMD_ARS_CAP, nd_cmd_ars_cap);
pub const ND_IOCTL_ARS_START: usize = _IOWR!(ND_IOCTL, ND_CMD_ARS_START, nd_cmd_ars_start);
pub const ND_IOCTL_ARS_STATUS: usize = _IOWR!(ND_IOCTL, ND_CMD_ARS_STATUS, nd_cmd_ars_status);
pub const ND_IOCTL_CLEAR_ERROR: usize = _IOWR!(ND_IOCTL, ND_CMD_CLEAR_ERROR, nd_cmd_clear_error);

pub const ND_DEVICE_DIMM: u32 = 1; // nd_dimm: container for "config data"
pub const ND_DEVICE_REGION_PMEM: u32 = 2; // nd_region: (parent of PMEM namespaces)
pub const ND_DEVICE_REGION_BLK: u32 = 3; // nd_region: (parent of BLK namespaces)
pub const ND_DEVICE_NAMESPACE_IO: u32 = 4; // legacy persistent memory
pub const ND_DEVICE_NAMESPACE_PMEM: u32 = 5; // PMEM namespace (may alias with BLK)
pub const ND_DEVICE_DAX_PMEM: u32 = 7; // Device DAX interface to pmem

pub const ND_DRIVER_DIMM: u32 = 1 << ND_DEVICE_DIMM;
pub const ND_DRIVER_REGION_PMEM: u32 = 1 << ND_DEVICE_REGION_PMEM;
pub const ND_DRIVER_REGION_BLK: u32 = 1 << ND_DEVICE_REGION_BLK;
pub const ND_DRIVER_NAMESPACE_IO: u32 = 1 << ND_DEVICE_NAMESPACE_IO;
pub const ND_DRIVER_NAMESPACE_PMEM: u32 = 1 << ND_DEVICE_NAMESPACE_PMEM;
pub const ND_DRIVER_DAX_PMEM: u32 = 1 << ND_DEVICE_DAX_PMEM;
pub const ARS_STATUS_MASK: u32 = 0x0000FFFF;
pub const ARS_EXT_STATUS_SHIFT: u32 = 16;

#[repr(C)]
pub struct nd_cmd_pkg {
    pub nd_family: u64, pub nd_command: u64, pub nd_size_in: u32, pub nd_size_out: u32,
    pub nd_reserved2: [u32; 9], pub nd_fw_size: u32, pub nd_payload: [u8; 0],
}

pub const NVDIMM_FAMILY_INTEL: u32 = 0;
pub const NVDIMM_FAMILY_HPE1: u32 = 1;
pub const NVDIMM_FAMILY_HPE2: u32 = 2;
pub const NVDIMM_FAMILY_MSFT: u32 = 3;
pub const NVDIMM_FAMILY_HYPERV: u32 = 4;
pub const NVDIMM_FAMILY_PAPR: u32 = 5;
pub const NVDIMM_FAMILY_MAX: u32 = NVDIMM_FAMILY_PAPR;
pub const NVDIMM_BUS_FAMILY_NFIT: u32 = 0;
pub const NVDIMM_BUS_FAMILY_INTEL: u32 = 1;
pub const NVDIMM_BUS_FAMILY_MAX: u32 = NVDIMM_BUS_FAMILY_INTEL;
pub const ND_IOCTL_CALL: usize = _IOWR!(ND_IOCTL, ND_CMD_CALL, nd_cmd_pkg);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
