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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependency supplied by the original ipp.h include.

#[macro_export]
macro_rules! TO_DCN10_IPP {
    ($ipp:expr) => { container_of!($ipp, dcn10_ipp, base) };
}

// These register-list macros retain the original token-level interface.
#[macro_export]
macro_rules! IPP_REG_LIST_DCN {
    ($id:expr) => {
        SRI!(FORMAT_CONTROL, CNVC_CFG, $id),
        SRI!(CNVC_SURFACE_PIXEL_FORMAT, CNVC_CFG, $id),
        SRI!(CURSOR0_CONTROL, CNVC_CUR, $id),
        SRI!(CURSOR0_COLOR0, CNVC_CUR, $id),
        SRI!(CURSOR0_COLOR1, CNVC_CUR, $id)
    };
}

#[macro_export]
macro_rules! IPP_REG_LIST_DCN10 {
    ($id:expr) => {
        IPP_REG_LIST_DCN!($id), SRI!(CURSOR_SETTINS, HUBPREQ, $id),
        SRI!(CURSOR_SURFACE_ADDRESS_HIGH, CURSOR, $id),
        SRI!(CURSOR_SURFACE_ADDRESS, CURSOR, $id), SRI!(CURSOR_SIZE, CURSOR, $id),
        SRI!(CURSOR_CONTROL, CURSOR, $id), SRI!(CURSOR_POSITION, CURSOR, $id),
        SRI!(CURSOR_HOT_SPOT, CURSOR, $id), SRI!(CURSOR_DST_OFFSET, CURSOR, $id)
    };
}

#[macro_export]
macro_rules! IPP_REG_LIST_DCN20 {
    ($id:expr) => {
        IPP_REG_LIST_DCN!($id), SRI!(CURSOR_SETTINGS, HUBPREQ, $id),
        SRI!(CURSOR_SURFACE_ADDRESS_HIGH, CURSOR0_, $id),
        SRI!(CURSOR_SURFACE_ADDRESS, CURSOR0_, $id), SRI!(CURSOR_SIZE, CURSOR0_, $id),
        SRI!(CURSOR_CONTROL, CURSOR0_, $id), SRI!(CURSOR_POSITION, CURSOR0_, $id),
        SRI!(CURSOR_HOT_SPOT, CURSOR0_, $id), SRI!(CURSOR_DST_OFFSET, CURSOR0_, $id)
    };
}

#[macro_export]
macro_rules! IPP_REG_LIST_DCN201 {
    ($id:expr) => {
        IPP_REG_LIST_DCN!($id), SRI!(CURSOR_SURFACE_ADDRESS_HIGH, CURSOR0_, $id),
        SRI!(CURSOR_SURFACE_ADDRESS, CURSOR0_, $id), SRI!(CURSOR_SIZE, CURSOR0_, $id),
        SRI!(CURSOR_CONTROL, CURSOR0_, $id), SRI!(CURSOR_POSITION, CURSOR0_, $id),
        SRI!(CURSOR_HOT_SPOT, CURSOR0_, $id), SRI!(CURSOR_DST_OFFSET, CURSOR0_, $id)
    };
}

pub const CURSOR0_CURSOR_CONTROL_CURSOR_2X_MAGNIFY_SHIFT: u32 = 0x4;
pub const CURSOR0_CURSOR_CONTROL_CURSOR_2X_MAGNIFY_MASK: u32 = 0x00000010;

// Field-list macros from the C header are retained as token-oriented Rust
// macros; the register/field identifiers are resolved by the including code.
#[macro_export]
macro_rules! IPP_SF { ($reg:ident, $field:ident, $post:ident) => { ($reg, $field, $post) }; }
#[macro_export]
macro_rules! IPP_MASK_SH_LIST_DCN { ($mask_sh:ident) => {
    IPP_SF!(CNVC_CFG0_CNVC_SURFACE_PIXEL_FORMAT, CNVC_SURFACE_PIXEL_FORMAT, $mask_sh),
    IPP_SF!(CNVC_CFG0_FORMAT_CONTROL, CNVC_BYPASS, $mask_sh),
    IPP_SF!(CNVC_CFG0_FORMAT_CONTROL, ALPHA_EN, $mask_sh),
    IPP_SF!(CNVC_CFG0_FORMAT_CONTROL, FORMAT_EXPANSION_MODE, $mask_sh),
    IPP_SF!(CNVC_CUR0_CURSOR0_CONTROL, CUR0_MODE, $mask_sh),
    IPP_SF!(CNVC_CUR0_CURSOR0_COLOR0, CUR0_COLOR0, $mask_sh),
    IPP_SF!(CNVC_CUR0_CURSOR0_COLOR1, CUR0_COLOR1, $mask_sh),
    IPP_SF!(CNVC_CUR0_CURSOR0_CONTROL, CUR0_EXPANSION_MODE, $mask_sh),
    IPP_SF!(CNVC_CUR0_CURSOR0_CONTROL, CUR0_ENABLE, $mask_sh)
} }

#[repr(C)]
pub struct dcn10_ipp_shift {
    pub CNVC_SURFACE_PIXEL_FORMAT: u8, pub CNVC_BYPASS: u8, pub ALPHA_EN: u8,
    pub FORMAT_EXPANSION_MODE: u8, pub CURSOR0_DST_Y_OFFSET: u8,
    pub CURSOR0_CHUNK_HDL_ADJUST: u8, pub CUR0_MODE: u8, pub CUR0_COLOR0: u8,
    pub CUR0_COLOR1: u8, pub CUR0_EXPANSION_MODE: u8,
    pub CURSOR_SURFACE_ADDRESS_HIGH: u8, pub CURSOR_SURFACE_ADDRESS: u8,
    pub CURSOR_WIDTH: u8, pub CURSOR_HEIGHT: u8, pub CURSOR_MODE: u8,
    pub CURSOR_2X_MAGNIFY: u8, pub CURSOR_PITCH: u8, pub CURSOR_LINES_PER_CHUNK: u8,
    pub CURSOR_ENABLE: u8, pub CUR0_ENABLE: u8, pub CURSOR_X_POSITION: u8,
    pub CURSOR_Y_POSITION: u8, pub CURSOR_HOT_SPOT_X: u8, pub CURSOR_HOT_SPOT_Y: u8,
    pub CURSOR_DST_X_OFFSET: u8, pub OUTPUT_FP: u8,
}

#[repr(C)]
pub struct dcn10_ipp_mask {
    pub CNVC_SURFACE_PIXEL_FORMAT: u32, pub CNVC_BYPASS: u32, pub ALPHA_EN: u32,
    pub FORMAT_EXPANSION_MODE: u32, pub CURSOR0_DST_Y_OFFSET: u32,
    pub CURSOR0_CHUNK_HDL_ADJUST: u32, pub CUR0_MODE: u32, pub CUR0_COLOR0: u32,
    pub CUR0_COLOR1: u32, pub CUR0_EXPANSION_MODE: u32,
    pub CURSOR_SURFACE_ADDRESS_HIGH: u32, pub CURSOR_SURFACE_ADDRESS: u32,
    pub CURSOR_WIDTH: u32, pub CURSOR_HEIGHT: u32, pub CURSOR_MODE: u32,
    pub CURSOR_2X_MAGNIFY: u32, pub CURSOR_PITCH: u32, pub CURSOR_LINES_PER_CHUNK: u32,
    pub CURSOR_ENABLE: u32, pub CUR0_ENABLE: u32, pub CURSOR_X_POSITION: u32,
    pub CURSOR_Y_POSITION: u32, pub CURSOR_HOT_SPOT_X: u32, pub CURSOR_HOT_SPOT_Y: u32,
    pub CURSOR_DST_X_OFFSET: u32, pub OUTPUT_FP: u32,
}

#[repr(C)]
pub struct dcn10_ipp_registers {
    pub CURSOR_SETTINS: u32, pub CURSOR_SETTINGS: u32,
    pub CNVC_SURFACE_PIXEL_FORMAT: u32, pub CURSOR0_CONTROL: u32,
    pub CURSOR0_COLOR0: u32, pub CURSOR0_COLOR1: u32, pub FORMAT_CONTROL: u32,
    pub CURSOR_SURFACE_ADDRESS_HIGH: u32, pub CURSOR_SURFACE_ADDRESS: u32,
    pub CURSOR_SIZE: u32, pub CURSOR_CONTROL: u32, pub CURSOR_POSITION: u32,
    pub CURSOR_HOT_SPOT: u32, pub CURSOR_DST_OFFSET: u32,
}

#[repr(C)]
pub struct dcn10_ipp {
    pub base: input_pixel_processor,
    pub regs: *const dcn10_ipp_registers,
    pub ipp_shift: *const dcn10_ipp_shift,
    pub ipp_mask: *const dcn10_ipp_mask,
    pub curs_attr: dc_cursor_attributes,
}

extern "C" {
    pub fn dcn10_ipp_construct(ippn10: *mut dcn10_ipp, ctx: *mut dc_context, inst: i32,
        regs: *const dcn10_ipp_registers, ipp_shift: *const dcn10_ipp_shift,
        ipp_mask: *const dcn10_ipp_mask);
    pub fn dcn20_ipp_construct(ippn10: *mut dcn10_ipp, ctx: *mut dc_context, inst: i32,
        regs: *const dcn10_ipp_registers, ipp_shift: *const dcn10_ipp_shift,
        ipp_mask: *const dcn10_ipp_mask);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
