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

// Dependencies are supplied by the surrounding DCN10 implementation.

#[repr(C)]
pub struct input_pixel_processor {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct dc_context {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct dcn10_ipp_registers {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct dcn10_ipp_shift {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct dcn10_ipp_mask {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct ipp_funcs {
    pub ipp_destroy: Option<unsafe extern "C" fn(*mut *mut input_pixel_processor)>,
}

#[repr(C)]
pub struct dcn10_ipp_base {
    pub ctx: *mut dc_context,
    pub inst: i32,
    pub funcs: *const ipp_funcs,
}

#[repr(C)]
pub struct dcn10_ipp {
    pub base: dcn10_ipp_base,
    pub regs: *const dcn10_ipp_registers,
    pub ipp_shift: *const dcn10_ipp_shift,
    pub ipp_mask: *const dcn10_ipp_mask,
}

extern "C" {
    fn kfree(ptr: *mut core::ffi::c_void);
}

unsafe extern "C" fn dcn10_ipp_destroy(ipp: *mut *mut input_pixel_processor) {
    // TO_DCN10_IPP(*ipp) is the container-of conversion supplied by dcn10_ipp.h.
    kfree(*ipp as *mut core::ffi::c_void);
    *ipp = core::ptr::null_mut();
}

static dcn10_ipp_funcs: ipp_funcs = ipp_funcs {
    ipp_destroy: Some(dcn10_ipp_destroy),
};

static dcn20_ipp_funcs: ipp_funcs = ipp_funcs {
    ipp_destroy: Some(dcn10_ipp_destroy),
};

pub unsafe extern "C" fn dcn10_ipp_construct(
    ippn10: *mut dcn10_ipp,
    ctx: *mut dc_context,
    inst: i32,
    regs: *const dcn10_ipp_registers,
    ipp_shift: *const dcn10_ipp_shift,
    ipp_mask: *const dcn10_ipp_mask,
) {
    (*ippn10).base.ctx = ctx;
    (*ippn10).base.inst = inst;
    (*ippn10).base.funcs = &dcn10_ipp_funcs;

    (*ippn10).regs = regs;
    (*ippn10).ipp_shift = ipp_shift;
    (*ippn10).ipp_mask = ipp_mask;
}

pub unsafe extern "C" fn dcn20_ipp_construct(
    ippn10: *mut dcn10_ipp,
    ctx: *mut dc_context,
    inst: i32,
    regs: *const dcn10_ipp_registers,
    ipp_shift: *const dcn10_ipp_shift,
    ipp_mask: *const dcn10_ipp_mask,
) {
    (*ippn10).base.ctx = ctx;
    (*ippn10).base.inst = inst;
    (*ippn10).base.funcs = &dcn20_ipp_funcs;

    (*ippn10).regs = regs;
    (*ippn10).ipp_shift = ipp_shift;
    (*ippn10).ipp_mask = ipp_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
