/* SPDX-License-Identifier: MIT */
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
 *
 */

// __GNUC__ conditional typedef intent is represented by the unconditional
// alias below; the header also declares uint unconditionally.
pub type uint = u32;

pub type int8 = i8;
pub type pint8 = *mut int8;
pub type int16 = i16;
pub type pint16 = *mut int16;
pub type int32 = i32;
pub type pint32 = *mut int32;
pub type int64 = i64;
pub type pint64 = *mut int64;

pub type uint8 = u8;
pub type puint8 = *mut uint8;
pub type uint16 = u16;
pub type puint16 = *mut uint16;
pub type uint32 = u32;
pub type puint32 = *mut uint32;
pub type uint64 = u64;
pub type puint64 = *mut uint64;

pub type ulong = u64;
pub type uchar = u8;

pub type pvoid = *mut core::ffi::c_void;
pub type pchar = *mut core::ffi::c_char;
pub type const_pvoid = *const core::ffi::c_void;
pub type const_pchar = *const core::ffi::c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rgba_struct {
    pub a: uint8,
    pub r: uint8,
    pub g: uint8,
    pub b: uint8,
}

pub type rgba_t = rgba_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gen_color_t {
    pub blue: uint8,
    pub green: uint8,
    pub red: uint8,
    pub alpha: uint8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union gen_color_u {
    pub val: uint32,
    pub f: gen_color_t,
}

//
// Types to make it easy to get or set the bits of a float/double.
// Avoids automatic casting from int to float and back.
//
// The original declarations are disabled by #if 0 and therefore emit no
// Rust items.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
