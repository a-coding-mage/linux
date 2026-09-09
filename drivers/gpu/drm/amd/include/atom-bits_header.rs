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

#[inline]
pub unsafe fn get_u8(bios: *mut core::ffi::c_void, ptr: i32) -> u8 {
    *((bios as *const u8).offset(ptr as isize))
}

macro_rules! U8 {
    ($ptr:expr) => {{
        unsafe { get_u8(ctx.ctx.bios, $ptr) }
    }};
}

macro_rules! CU8 {
    ($ptr:expr) => {{
        unsafe { get_u8(ctx.bios, $ptr) }
    }};
}

#[inline]
pub unsafe fn get_u16(bios: *mut core::ffi::c_void, ptr: i32) -> u16 {
    get_u8(bios, ptr) as u16
        | ((get_u8(bios, ptr.wrapping_add(1)) as u16) << 8)
}

macro_rules! U16 {
    ($ptr:expr) => {{
        unsafe { get_u16(ctx.ctx.bios, $ptr) }
    }};
}

macro_rules! CU16 {
    ($ptr:expr) => {{
        unsafe { get_u16(ctx.bios, $ptr) }
    }};
}

#[inline]
pub unsafe fn get_u32(bios: *mut core::ffi::c_void, ptr: i32) -> u32 {
    get_u16(bios, ptr) as u32
        | ((get_u16(bios, ptr.wrapping_add(2)) as u32) << 16)
}

macro_rules! U32 {
    ($ptr:expr) => {{
        unsafe { get_u32(ctx.ctx.bios, $ptr) }
    }};
}

macro_rules! CU32 {
    ($ptr:expr) => {{
        unsafe { get_u32(ctx.bios, $ptr) }
    }};
}

macro_rules! CSTR {
    ($ptr:expr) => {{
        (ctx.bios as *mut i8).offset($ptr as isize)
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
