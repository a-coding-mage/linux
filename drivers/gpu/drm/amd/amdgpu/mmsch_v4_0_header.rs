/*
 * Copyright 2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency supplied by amdgpu_vcn.h in the original header.

pub const MMSCH_VERSION_MAJOR: u32 = 4;
pub const MMSCH_VERSION_MINOR: u32 = 0;
pub const MMSCH_VERSION: u32 = (MMSCH_VERSION_MAJOR << 16) | MMSCH_VERSION_MINOR;

pub const RB_ENABLED: u32 = 1 << 0;
pub const RB4_ENABLED: u32 = 1 << 1;

pub const MMSCH_VF_ENGINE_STATUS__PASS: u32 = 0x1;

pub const MMSCH_VF_MAILBOX_RESP__OK: u32 = 0x1;
pub const MMSCH_VF_MAILBOX_RESP__INCOMPLETE: u32 = 0x2;
pub const MMSCH_VF_MAILBOX_RESP__FAILED: u32 = 0x3;
pub const MMSCH_VF_MAILBOX_RESP__FAILED_SMALL_CTX_SIZE: u32 = 0x4;
pub const MMSCH_VF_MAILBOX_RESP__UNKNOWN_CMD: u32 = 0x5;

pub const MMSCH_V4_0_VCN_INSTANCES: usize = 0x2;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MmschV4_0CommandType {
    MmschCommandDirectRegWrite = 0,
    MmschCommandDirectRegPolling = 2,
    MmschCommandDirectRegReadModifyWrite = 3,
    MmschCommandIndirectRegWrite = 8,
    MmschCommandEnd = 0xf,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MmschV4_0TableInfo {
    pub init_status: u32,
    pub table_offset: u32,
    pub table_size: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MmschV4_0InitHeader {
    pub version: u32,
    pub total_size: u32,
    pub inst: [MmschV4_0TableInfo; MMSCH_V4_0_VCN_INSTANCES],
    pub jpegdec: MmschV4_0TableInfo,
}

// C bit-fields are represented by their packed 32-bit storage word.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MmschV4_0CmdDirectRegHeader {
    pub packed: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MmschV4_0CmdIndirectRegHeader {
    pub packed: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MmschV4_0CmdDirectWrite {
    pub cmd_header: MmschV4_0CmdDirectRegHeader,
    pub reg_value: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MmschV4_0CmdDirectReadModifyWrite {
    pub cmd_header: MmschV4_0CmdDirectRegHeader,
    pub write_data: u32,
    pub mask_value: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MmschV4_0CmdDirectPolling {
    pub cmd_header: MmschV4_0CmdDirectRegHeader,
    pub mask_value: u32,
    pub wait_value: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MmschV4_0CmdEnd {
    pub cmd_header: MmschV4_0CmdDirectRegHeader,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MmschV4_0CmdIndirectWrite {
    pub cmd_header: MmschV4_0CmdIndirectRegHeader,
    pub reg_value: u32,
}

// The following macros preserve the original header's caller-provided state
// (size, size_dw, table_loc, table_size, and command temporaries).
#[macro_export]
macro_rules! MMSCH_V4_0_INSERT_DIRECT_RD_MOD_WT {
    ($reg:expr, $mask:expr, $data:expr) => {{
        size = core::mem::size_of::<MmschV4_0CmdDirectReadModifyWrite>();
        size_dw = size / 4;
        direct_rd_mod_wt.cmd_header.packed = (($reg as u32) & 0x0fff_ffff) | (3 << 28);
        direct_rd_mod_wt.mask_value = $mask;
        direct_rd_mod_wt.write_data = $data;
        core::ptr::copy_nonoverlapping(&direct_rd_mod_wt as *const _ as *const u8, table_loc as *mut u8, size);
        table_loc = table_loc.add(size_dw);
        table_size += size_dw;
    }};
}

#[macro_export]
macro_rules! MMSCH_V4_0_INSERT_DIRECT_WT {
    ($reg:expr, $value:expr) => {{
        size = core::mem::size_of::<MmschV4_0CmdDirectWrite>();
        size_dw = size / 4;
        direct_wt.cmd_header.packed = (($reg as u32) & 0x0fff_ffff) | (0 << 28);
        direct_wt.reg_value = $value;
        core::ptr::copy_nonoverlapping(&direct_wt as *const _ as *const u8, table_loc as *mut u8, size);
        table_loc = table_loc.add(size_dw);
        table_size += size_dw;
    }};
}

#[macro_export]
macro_rules! MMSCH_V4_0_INSERT_DIRECT_POLL {
    ($reg:expr, $mask:expr, $wait:expr) => {{
        size = core::mem::size_of::<MmschV4_0CmdDirectPolling>();
        size_dw = size / 4;
        direct_poll.cmd_header.packed = (($reg as u32) & 0x0fff_ffff) | (2 << 28);
        direct_poll.mask_value = $mask;
        direct_poll.wait_value = $wait;
        core::ptr::copy_nonoverlapping(&direct_poll as *const _ as *const u8, table_loc as *mut u8, size);
        table_loc = table_loc.add(size_dw);
        table_size += size_dw;
    }};
}

#[macro_export]
macro_rules! MMSCH_V4_0_INSERT_END {
    () => {{
        size = core::mem::size_of::<MmschV4_0CmdEnd>();
        size_dw = size / 4;
        core::ptr::copy_nonoverlapping(&end as *const _ as *const u8, table_loc as *mut u8, size);
        table_loc = table_loc.add(size_dw);
        table_size += size_dw;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
