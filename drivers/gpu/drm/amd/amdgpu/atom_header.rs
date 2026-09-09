/*
 * Copyright 2008 Advanced Micro Devices, Inc.
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
 *
 * Author: Stanislaw Skowronek
 */

// Dependencies supplied by the surrounding translation unit: linux types,
// struct drm_device, struct mutex, atom-types.h, atombios.h, and ObjectID.h.

#[repr(C)]
pub struct drm_device;

pub const ATOM_BIOS_MAGIC: u32 = 0xAA55;
pub const ATOM_ATI_MAGIC_PTR: u32 = 0x30;
pub const ATOM_ATI_MAGIC: &str = " 761295520";
pub const ATOM_ROM_TABLE_PTR: u32 = 0x48;
pub const ATOM_ROM_MAGIC: &str = "ATOM";
pub const ATOM_ROM_MAGIC_PTR: u32 = 4;
pub const ATOM_ROM_CFG_PTR: u32 = 0xC;
pub const ATOM_ROM_MSG_PTR: u32 = 0x10;
pub const ATOM_ROM_CMD_PTR: u32 = 0x1E;
pub const ATOM_ROM_DATA_PTR: u32 = 0x20;
pub const ATOM_CMD_INIT: u32 = 0;
pub const ATOM_CMD_SETSCLK: u32 = 0x0A;
pub const ATOM_CMD_SETMCLK: u32 = 0x0B;
pub const ATOM_CMD_SETPCLK: u32 = 0x0C;
pub const ATOM_CMD_SPDFANCNTL: u32 = 0x39;
pub const ATOM_DATA_FWI_PTR: u32 = 0xC;
pub const ATOM_DATA_IIO_PTR: u32 = 0x32;
pub const ATOM_FWI_DEFSCLK_PTR: u32 = 8;
pub const ATOM_FWI_DEFMCLK_PTR: u32 = 0xC;
pub const ATOM_FWI_MAXSCLK_PTR: u32 = 0x24;
pub const ATOM_FWI_MAXMCLK_PTR: u32 = 0x28;
pub const ATOM_CT_SIZE_PTR: u32 = 0;
pub const ATOM_CT_WS_PTR: u32 = 4;
pub const ATOM_CT_PS_PTR: u32 = 5;
pub const ATOM_CT_PS_MASK: u32 = 0x7F;
pub const ATOM_CT_CODE_PTR: u32 = 6;
pub const ATOM_OP_CNT: u32 = 127;
pub const ATOM_OP_EOT: u32 = 91;
pub const ATOM_CASE_MAGIC: u32 = 0x63;
pub const ATOM_CASE_END: u32 = 0x5A5A;

pub const ATOM_ARG_REG: u32 = 0;
pub const ATOM_ARG_PS: u32 = 1;
pub const ATOM_ARG_WS: u32 = 2;
pub const ATOM_ARG_FB: u32 = 3;
pub const ATOM_ARG_ID: u32 = 4;
pub const ATOM_ARG_IMM: u32 = 5;
pub const ATOM_ARG_PLL: u32 = 6;
pub const ATOM_ARG_MC: u32 = 7;

pub const ATOM_SRC_DWORD: u32 = 0;
pub const ATOM_SRC_WORD0: u32 = 1;
pub const ATOM_SRC_WORD8: u32 = 2;
pub const ATOM_SRC_WORD16: u32 = 3;
pub const ATOM_SRC_BYTE0: u32 = 4;
pub const ATOM_SRC_BYTE8: u32 = 5;
pub const ATOM_SRC_BYTE16: u32 = 6;
pub const ATOM_SRC_BYTE24: u32 = 7;

pub const ATOM_WS_QUOTIENT: u32 = 0x40;
pub const ATOM_WS_REMAINDER: u32 = 0x41;
pub const ATOM_WS_DATAPTR: u32 = 0x42;
pub const ATOM_WS_SHIFT: u32 = 0x43;
pub const ATOM_WS_OR_MASK: u32 = 0x44;
pub const ATOM_WS_AND_MASK: u32 = 0x45;
pub const ATOM_WS_FB_WINDOW: u32 = 0x46;
pub const ATOM_WS_ATTRIBUTES: u32 = 0x47;
pub const ATOM_WS_REGPTR: u32 = 0x48;

pub const ATOM_IIO_NOP: u32 = 0;
pub const ATOM_IIO_START: u32 = 1;
pub const ATOM_IIO_READ: u32 = 2;
pub const ATOM_IIO_WRITE: u32 = 3;
pub const ATOM_IIO_CLEAR: u32 = 4;
pub const ATOM_IIO_SET: u32 = 5;
pub const ATOM_IIO_MOVE_INDEX: u32 = 6;
pub const ATOM_IIO_MOVE_ATTR: u32 = 7;
pub const ATOM_IIO_MOVE_DATA: u32 = 8;
pub const ATOM_IIO_END: u32 = 9;
pub const ATOM_IO_MM: u32 = 0;
pub const ATOM_IO_PCI: u32 = 1;
pub const ATOM_IO_SYSIO: u32 = 2;
pub const ATOM_IO_IIO: u32 = 0x80;
pub const STRLEN_NORMAL: usize = 32;
pub const STRLEN_LONG: usize = 64;
pub const STRLEN_VERYLONG: usize = 254;

#[repr(C)]
pub struct card_info {
    pub dev: *mut drm_device,
    pub reg_write: Option<unsafe extern "C" fn(*mut card_info, u32, u32)>,
    pub reg_read: Option<unsafe extern "C" fn(*mut card_info, u32) -> u32>,
    pub mc_write: Option<unsafe extern "C" fn(*mut card_info, u32, u32)>,
    pub mc_read: Option<unsafe extern "C" fn(*mut card_info, u32) -> u32>,
    pub pll_write: Option<unsafe extern "C" fn(*mut card_info, u32, u32)>,
    pub pll_read: Option<unsafe extern "C" fn(*mut card_info, u32) -> u32>,
}

#[repr(C)]
pub struct atom_context {
    pub card: *mut card_info,
    pub mutex: mutex,
    pub bios: *mut core::ffi::c_void,
    pub bios_size: u32,
    pub cmd_table: u32,
    pub data_table: u32,
    pub iio: *mut u16,
    pub data_block: u16,
    pub fb_base: u32,
    pub divmul: [u32; 2],
    pub io_attr: u16,
    pub reg_block: u16,
    pub shift: u8,
    pub cs_equal: i32,
    pub cs_above: i32,
    pub io_mode: i32,
    pub scratch: *mut u32,
    pub scratch_size_bytes: i32,
    pub name: [u8; STRLEN_LONG],
    pub vbios_pn: [u8; STRLEN_LONG],
    pub version: u32,
    pub vbios_ver_str: [u8; STRLEN_NORMAL],
    pub date: [u8; STRLEN_NORMAL],
    pub build_num: [u8; STRLEN_NORMAL],
    // Nesting depth for ATOM_OP_CALLTABLE
    pub execute_depth: u32,
}

extern "C" {
    pub static mut amdgpu_atom_debug: i32;
    pub fn amdgpu_atom_parse(card: *mut card_info, bios: *mut core::ffi::c_void, bios_size: u32) -> *mut atom_context;
    pub fn amdgpu_atom_execute_table(ctx: *mut atom_context, index: i32, params: *mut u32, params_size: i32) -> i32;
    pub fn amdgpu_atom_asic_init(ctx: *mut atom_context) -> i32;
    pub fn amdgpu_atom_destroy(ctx: *mut atom_context);
    pub fn amdgpu_atom_parse_data_header(ctx: *mut atom_context, index: i32, size: *mut u16, frev: *mut u8, crev: *mut u8, data_start: *mut u16) -> bool;
    pub fn amdgpu_atom_parse_cmd_header(ctx: *mut atom_context, index: i32, frev: *mut u8, crev: *mut u8) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
