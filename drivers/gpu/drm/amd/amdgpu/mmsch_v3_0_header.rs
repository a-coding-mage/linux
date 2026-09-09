/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// C dependency: amdgpu_vcn.h

pub const MMSCH_VERSION_MAJOR: u32 = 3;
pub const MMSCH_VERSION_MINOR: u32 = 0;
pub const MMSCH_VERSION: u32 = (MMSCH_VERSION_MAJOR << 16) | MMSCH_VERSION_MINOR;

pub const MMSCH_V3_0_VCN_INSTANCES: usize = 0x2;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mmsch_v3_0_command_type {
    MMSCH_COMMAND__DIRECT_REG_WRITE = 0,
    MMSCH_COMMAND__DIRECT_REG_POLLING = 2,
    MMSCH_COMMAND__DIRECT_REG_READ_MODIFY_WRITE = 3,
    MMSCH_COMMAND__INDIRECT_REG_WRITE = 8,
    MMSCH_COMMAND__END = 0xf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v3_0_table_info {
    pub init_status: u32,
    pub table_offset: u32,
    pub table_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v3_0_init_header {
    pub version: u32,
    pub total_size: u32,
    pub inst: [mmsch_v3_0_table_info; MMSCH_V3_0_VCN_INSTANCES],
}

// C bit-fields occupy one 32-bit word. The raw field preserves their layout:
// reg_offset:28, command_type:4.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v3_0_cmd_direct_reg_header {
    pub bits: u32,
}

// C bit-fields occupy one 32-bit word. The raw field preserves their layout:
// reg_offset:20, reg_idx_space:8, command_type:4.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v3_0_cmd_indirect_reg_header {
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v3_0_cmd_direct_write {
    pub cmd_header: mmsch_v3_0_cmd_direct_reg_header,
    pub reg_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v3_0_cmd_direct_read_modify_write {
    pub cmd_header: mmsch_v3_0_cmd_direct_reg_header,
    pub write_data: u32,
    pub mask_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v3_0_cmd_direct_polling {
    pub cmd_header: mmsch_v3_0_cmd_direct_reg_header,
    pub mask_value: u32,
    pub wait_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v3_0_cmd_end {
    pub cmd_header: mmsch_v3_0_cmd_direct_reg_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v3_0_cmd_indirect_write {
    pub cmd_header: mmsch_v3_0_cmd_indirect_reg_header,
    pub reg_value: u32,
}

#[macro_export]
macro_rules! MMSCH_V3_0_INSERT_DIRECT_RD_MOD_WT {
    ($reg:expr, $mask:expr, $data:expr) => {{
        size = core::mem::size_of::<mmsch_v3_0_cmd_direct_read_modify_write>();
        size_dw = size / 4;
        direct_rd_mod_wt.cmd_header.bits = (($reg as u32) & 0x0fffffff) |
            ((MMSCH_COMMAND__DIRECT_REG_READ_MODIFY_WRITE as u32) << 28);
        direct_rd_mod_wt.mask_value = $mask;
        direct_rd_mod_wt.write_data = $data;
        unsafe {
            core::ptr::copy_nonoverlapping(
                &direct_rd_mod_wt as *const _ as *const u8,
                table_loc as *mut u8,
                size,
            );
        }
        table_loc += size_dw;
        table_size += size_dw;
    }};
}

#[macro_export]
macro_rules! MMSCH_V3_0_INSERT_DIRECT_WT {
    ($reg:expr, $value:expr) => {{
        size = core::mem::size_of::<mmsch_v3_0_cmd_direct_write>();
        size_dw = size / 4;
        direct_wt.cmd_header.bits = (($reg as u32) & 0x0fffffff) |
            ((MMSCH_COMMAND__DIRECT_REG_WRITE as u32) << 28);
        direct_wt.reg_value = $value;
        unsafe { core::ptr::copy_nonoverlapping(&direct_wt as *const _ as *const u8, table_loc as *mut u8, size); }
        table_loc += size_dw;
        table_size += size_dw;
    }};
}

#[macro_export]
macro_rules! MMSCH_V3_0_INSERT_DIRECT_POLL {
    ($reg:expr, $mask:expr, $wait:expr) => {{
        size = core::mem::size_of::<mmsch_v3_0_cmd_direct_polling>();
        size_dw = size / 4;
        direct_poll.cmd_header.bits = (($reg as u32) & 0x0fffffff) |
            ((MMSCH_COMMAND__DIRECT_REG_POLLING as u32) << 28);
        direct_poll.mask_value = $mask;
        direct_poll.wait_value = $wait;
        unsafe { core::ptr::copy_nonoverlapping(&direct_poll as *const _ as *const u8, table_loc as *mut u8, size); }
        table_loc += size_dw;
        table_size += size_dw;
    }};
}

#[macro_export]
macro_rules! MMSCH_V3_0_INSERT_END {
    () => {{
        size = core::mem::size_of::<mmsch_v3_0_cmd_end>();
        size_dw = size / 4;
        unsafe { core::ptr::copy_nonoverlapping(&end as *const _ as *const u8, table_loc as *mut u8, size); }
        table_loc += size_dw;
        table_size += size_dw;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
