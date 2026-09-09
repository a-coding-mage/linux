/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

pub const MMSCH_VERSION: u32 = 0x1;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mmsch_v1_0_command_type {
    MMSCH_COMMAND__DIRECT_REG_WRITE = 0,
    MMSCH_COMMAND__DIRECT_REG_POLLING = 2,
    MMSCH_COMMAND__DIRECT_REG_READ_MODIFY_WRITE = 3,
    MMSCH_COMMAND__INDIRECT_REG_WRITE = 8,
    MMSCH_COMMAND__END = 0xf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v1_0_init_header {
    pub version: u32,
    pub header_size: u32,
    pub vce_init_status: u32,
    pub uvd_init_status: u32,
    pub vce_table_offset: u32,
    pub vce_table_size: u32,
    pub uvd_table_offset: u32,
    pub uvd_table_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_vf_eng_init_header {
    pub init_status: u32,
    pub table_offset: u32,
    pub table_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v1_1_init_header {
    pub version: u32,
    pub total_size: u32,
    pub eng: [mmsch_vf_eng_init_header; 2],
}

/* C bitfields are represented by their containing u32; accessors preserve the bit layout. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v1_0_cmd_direct_reg_header {
    pub bits: u32,
}

impl mmsch_v1_0_cmd_direct_reg_header {
    pub fn reg_offset(&self) -> u32 { self.bits & 0x0fffffff }
    pub fn set_reg_offset(&mut self, value: u32) { self.bits = (self.bits & 0xf0000000) | (value & 0x0fffffff); }
    pub fn command_type(&self) -> u32 { self.bits >> 28 }
    pub fn set_command_type(&mut self, value: u32) { self.bits = (self.bits & 0x0fffffff) | ((value & 0xf) << 28); }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v1_0_cmd_indirect_reg_header {
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v1_0_cmd_direct_write {
    pub cmd_header: mmsch_v1_0_cmd_direct_reg_header,
    pub reg_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v1_0_cmd_direct_read_modify_write {
    pub cmd_header: mmsch_v1_0_cmd_direct_reg_header,
    pub write_data: u32,
    pub mask_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v1_0_cmd_direct_polling {
    pub cmd_header: mmsch_v1_0_cmd_direct_reg_header,
    pub mask_value: u32,
    pub wait_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v1_0_cmd_end {
    pub cmd_header: mmsch_v1_0_cmd_direct_reg_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v1_0_cmd_indirect_write {
    pub cmd_header: mmsch_v1_0_cmd_indirect_reg_header,
    pub reg_value: u32,
}

pub unsafe fn mmsch_v1_0_insert_direct_wt(
    direct_wt: *mut mmsch_v1_0_cmd_direct_write,
    init_table: *mut u32,
    reg_offset: u32,
    value: u32,
) {
    (*direct_wt).cmd_header.set_reg_offset(reg_offset);
    (*direct_wt).reg_value = value;
    core::ptr::copy_nonoverlapping(direct_wt as *const u8, init_table as *mut u8, core::mem::size_of::<mmsch_v1_0_cmd_direct_write>());
}

pub unsafe fn mmsch_v1_0_insert_direct_rd_mod_wt(
    direct_rd_mod_wt: *mut mmsch_v1_0_cmd_direct_read_modify_write,
    init_table: *mut u32,
    reg_offset: u32,
    mask: u32,
    data: u32,
) {
    (*direct_rd_mod_wt).cmd_header.set_reg_offset(reg_offset);
    (*direct_rd_mod_wt).mask_value = mask;
    (*direct_rd_mod_wt).write_data = data;
    core::ptr::copy_nonoverlapping(direct_rd_mod_wt as *const u8, init_table as *mut u8, core::mem::size_of::<mmsch_v1_0_cmd_direct_read_modify_write>());
}

pub unsafe fn mmsch_v1_0_insert_direct_poll(
    direct_poll: *mut mmsch_v1_0_cmd_direct_polling,
    init_table: *mut u32,
    reg_offset: u32,
    mask: u32,
    wait: u32,
) {
    (*direct_poll).cmd_header.set_reg_offset(reg_offset);
    (*direct_poll).mask_value = mask;
    (*direct_poll).wait_value = wait;
    core::ptr::copy_nonoverlapping(direct_poll as *const u8, init_table as *mut u8, core::mem::size_of::<mmsch_v1_0_cmd_direct_polling>());
}

#[macro_export]
macro_rules! MMSCH_V1_0_INSERT_DIRECT_RD_MOD_WT {
    ($reg:expr, $mask:expr, $data:expr) => {{
        unsafe { $crate::mmsch_v1_0_insert_direct_rd_mod_wt(&mut direct_rd_mod_wt, init_table, $reg, $mask, $data); }
        init_table = unsafe { init_table.add(core::mem::size_of::<$crate::mmsch_v1_0_cmd_direct_read_modify_write>() / 4) };
        table_size += core::mem::size_of::<$crate::mmsch_v1_0_cmd_direct_read_modify_write>() / 4;
    }};
}

#[macro_export]
macro_rules! MMSCH_V1_0_INSERT_DIRECT_WT {
    ($reg:expr, $value:expr) => {{
        unsafe { $crate::mmsch_v1_0_insert_direct_wt(&mut direct_wt, init_table, $reg, $value); }
        init_table = unsafe { init_table.add(core::mem::size_of::<$crate::mmsch_v1_0_cmd_direct_write>() / 4) };
        table_size += core::mem::size_of::<$crate::mmsch_v1_0_cmd_direct_write>() / 4;
    }};
}

#[macro_export]
macro_rules! MMSCH_V1_0_INSERT_DIRECT_POLL {
    ($reg:expr, $mask:expr, $wait:expr) => {{
        unsafe { $crate::mmsch_v1_0_insert_direct_poll(&mut direct_poll, init_table, $reg, $mask, $wait); }
        init_table = unsafe { init_table.add(core::mem::size_of::<$crate::mmsch_v1_0_cmd_direct_polling>() / 4) };
        table_size += core::mem::size_of::<$crate::mmsch_v1_0_cmd_direct_polling>() / 4;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
