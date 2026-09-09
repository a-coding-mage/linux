/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependency supplied by amdgpu_vcn.h in the original C header.

pub const MMSCH_VERSION_MAJOR: u32 = 5;
pub const MMSCH_VERSION_MINOR: u32 = 0;
pub const MMSCH_VERSION: u32 = (MMSCH_VERSION_MAJOR << 16) | MMSCH_VERSION_MINOR;

pub const RB_ENABLED: u32 = 1 << 0;
pub const RB4_ENABLED: u32 = 1 << 1;
pub const MMSCH_VF_ENGINE_STATUS_PASS: u32 = 0x1;
pub const MMSCH_VF_MAILBOX_RESP_OK: u32 = 0x1;
pub const MMSCH_VF_MAILBOX_RESP_INCOMPLETE: u32 = 0x2;
pub const MMSCH_VF_MAILBOX_RESP_FAILED: u32 = 0x3;
pub const MMSCH_VF_MAILBOX_RESP_FAILED_SMALL_CTX_SIZE: u32 = 0x4;
pub const MMSCH_VF_MAILBOX_RESP_UNKNOWN_CMD: u32 = 0x5;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MmschV5_0CommandType {
    MmschCommandDirectRegWrite = 0,
    MmschCommandDirectRegPolling = 2,
    MmschCommandDirectRegReadModifyWrite = 3,
    MmschCommandIndirectRegWrite = 8,
    MmschCommandEnd = 0xf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MmschV5_0TableInfo {
    pub init_status: u32,
    pub table_offset: u32,
    pub table_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MmschV5_0InitHeader {
    pub version: u32,
    pub total_size: u32,
    pub vcn0: MmschV5_0TableInfo,
    pub mjpegdec0: [MmschV5_0TableInfo; 5],
    pub mjpegdec1: [MmschV5_0TableInfo; 5],
}

// C bit-fields occupy one u32 word; the masks preserve their on-wire layout.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MmschV5_0CmdDirectRegHeader {
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MmschV5_0CmdIndirectRegHeader {
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MmschV5_0CmdDirectWrite {
    pub cmd_header: MmschV5_0CmdDirectRegHeader,
    pub reg_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MmschV5_0CmdDirectReadModifyWrite {
    pub cmd_header: MmschV5_0CmdDirectRegHeader,
    pub write_data: u32,
    pub mask_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MmschV5_0CmdDirectPolling {
    pub cmd_header: MmschV5_0CmdDirectRegHeader,
    pub mask_value: u32,
    pub wait_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MmschV5_0CmdEnd {
    pub cmd_header: MmschV5_0CmdDirectRegHeader,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MmschV5_0CmdIndirectWrite {
    pub cmd_header: MmschV5_0CmdIndirectRegHeader,
    pub reg_value: u32,
}

#[inline]
pub const fn mmsch_v5_0_direct_header(reg: u32, command_type: u32) -> MmschV5_0CmdDirectRegHeader {
    MmschV5_0CmdDirectRegHeader { raw: (reg & 0x0fffffff) | ((command_type & 0xf) << 28) }
}

#[inline]
pub const fn mmsch_v5_0_indirect_header(reg: u32, reg_idx_space: u32, command_type: u32) -> MmschV5_0CmdIndirectRegHeader {
    MmschV5_0CmdIndirectRegHeader { raw: (reg & 0x000fffff) | ((reg_idx_space & 0xff) << 20) | ((command_type & 0xf) << 28) }
}

#[macro_export]
macro_rules! MMSCH_V5_0_INSERT_DIRECT_RD_MOD_WT {
    ($reg:expr, $mask:expr, $data:expr) => {{
        size = core::mem::size_of::<$crate::MmschV5_0CmdDirectReadModifyWrite>();
        size_dw = size / 4;
        direct_rd_mod_wt.cmd_header = $crate::mmsch_v5_0_direct_header($reg as u32, 3);
        direct_rd_mod_wt.mask_value = $mask;
        direct_rd_mod_wt.write_data = $data;
        unsafe { core::ptr::copy_nonoverlapping(&direct_rd_mod_wt as *const _ as *const u8, table_loc as *mut u8, size); }
        table_loc += size_dw;
        table_size += size_dw;
    }};
}

#[macro_export]
macro_rules! MMSCH_V5_0_INSERT_DIRECT_WT {
    ($reg:expr, $value:expr) => {{
        size = core::mem::size_of::<$crate::MmschV5_0CmdDirectWrite>();
        size_dw = size / 4;
        direct_wt.cmd_header = $crate::mmsch_v5_0_direct_header($reg as u32, 0);
        direct_wt.reg_value = $value;
        unsafe { core::ptr::copy_nonoverlapping(&direct_wt as *const _ as *const u8, table_loc as *mut u8, size); }
        table_loc += size_dw;
        table_size += size_dw;
    }};
}

#[macro_export]
macro_rules! MMSCH_V5_0_INSERT_DIRECT_POLL {
    ($reg:expr, $mask:expr, $wait:expr) => {{
        size = core::mem::size_of::<$crate::MmschV5_0CmdDirectPolling>();
        size_dw = size / 4;
        direct_poll.cmd_header = $crate::mmsch_v5_0_direct_header($reg as u32, 2);
        direct_poll.mask_value = $mask;
        direct_poll.wait_value = $wait;
        unsafe { core::ptr::copy_nonoverlapping(&direct_poll as *const _ as *const u8, table_loc as *mut u8, size); }
        table_loc += size_dw;
        table_size += size_dw;
    }};
}

#[macro_export]
macro_rules! MMSCH_V5_0_INSERT_END {
    () => {{
        size = core::mem::size_of::<$crate::MmschV5_0CmdEnd>();
        size_dw = size / 4;
        unsafe { core::ptr::copy_nonoverlapping(&end as *const _ as *const u8, table_loc as *mut u8, size); }
        table_loc += size_dw;
        table_size += size_dw;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
