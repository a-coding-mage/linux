/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit:
// dc_bios_types.h, bios_parser_helper.h
// use atomfirmware_bringup.h only. Not atombios.h anymore

#[repr(C)]
pub struct atom_data_revision {
    pub major: u32,
    pub minor: u32,
}

#[repr(C)]
pub union object_info_table_v {
    pub v1_4: *mut display_object_info_table_v1_4,
    pub v1_5: *mut display_object_info_table_v1_5,
}

#[repr(C)]
pub struct object_info_table {
    pub revision: atom_data_revision,
    pub __bindgen_anon_1: object_info_table_v,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum spread_spectrum_id {
    SS_ID_UNKNOWN = 0,
    SS_ID_DP1 = 0xf1,
    SS_ID_DP2 = 0xf2,
    SS_ID_LVLINK_2700MHZ = 0xf3,
    SS_ID_LVLINK_1620MHZ = 0xf4,
}

#[repr(C)]
pub struct bios_parser {
    pub base: dc_bios,
    pub object_info_tbl: object_info_table,
    pub object_info_tbl_offset: u32,
    pub master_data_tbl: *mut atom_master_data_table_v2_1,
    pub bios_helper: *const bios_parser_helper,
    pub cmd_helper: *const command_table_helper,
    pub cmd_tbl: cmd_tbl,
    pub remap_device_tags: bool,
}

// C macro equivalent: container_of(dc_bios, struct bios_parser, base).
#[inline]
pub unsafe fn BP_FROM_DCB(dc_bios: *mut dc_bios) -> *mut bios_parser {
    dc_bios as *mut bios_parser
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
