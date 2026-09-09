/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum baco_cmd_type {
    CMD_WRITE = 0,
    CMD_READMODIFYWRITE,
    CMD_WAITFOR,
    CMD_DELAY_MS,
    CMD_DELAY_US,
}

#[repr(C)]
pub struct baco_cmd_entry {
    pub cmd: baco_cmd_type,
    pub reg_offset: u32,
    pub mask: u32,
    pub shift: u32,
    pub timeout: u32,
    pub val: u32,
}

#[repr(C)]
pub struct soc15_baco_cmd_entry {
    pub cmd: baco_cmd_type,
    pub hwip: u32,
    pub inst: u32,
    pub seg: u32,
    pub reg_offset: u32,
    pub mask: u32,
    pub shift: u32,
    pub timeout: u32,
    pub val: u32,
}

// Supplied by the hwmgr dependency.
#[repr(C)]
pub struct pp_hwmgr {
    _private: [u8; 0],
}

extern "C" {
    pub fn baco_program_registers(
        hwmgr: *mut pp_hwmgr,
        entry: *const baco_cmd_entry,
        array_size: u32,
    ) -> bool;

    pub fn soc15_baco_program_registers(
        hwmgr: *mut pp_hwmgr,
        entry: *const soc15_baco_cmd_entry,
        array_size: u32,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
