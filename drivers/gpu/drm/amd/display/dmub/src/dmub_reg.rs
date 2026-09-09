/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

use core::ffi::{c_void, VaList};

// Definitions supplied by dmub_reg.h and ../dmub_srv.h.
use crate::dmub_srv;

#[repr(C)]
struct dmub_reg_value_masks {
    value: u32,
    mask: u32,
}

#[inline]
unsafe fn set_reg_field_value_masks(
    field_value_mask: *mut dmub_reg_value_masks,
    value: u32,
    mask: u32,
    shift: u8,
) {
    (*field_value_mask).value =
        ((*field_value_mask).value & !mask) | (mask & (value << shift));
    (*field_value_mask).mask |= mask;
}

unsafe fn set_reg_field_values(
    field_value_mask: *mut dmub_reg_value_masks,
    addr: u32,
    n: i32,
    shift1: u8,
    mask1: u32,
    field_value1: u32,
    mut ap: VaList<'_>,
) {
    let _ = addr;
    let mut shift: u32;
    let mut mask: u32;
    let mut field_value: u32;
    let mut i: i32 = 1;

    // gather all bits value/mask getting updated in this register
    set_reg_field_value_masks(field_value_mask, field_value1, mask1, shift1);

    while i < n {
        shift = ap.arg::<u32>();
        mask = ap.arg::<u32>();
        field_value = ap.arg::<u32>();

        debug_assert!(shift <= 0xFF);
        set_reg_field_value_masks(field_value_mask, field_value, mask, shift as u8);
        i += 1;
    }
}

#[inline]
fn get_reg_field_value_ex(reg_value: u32, mask: u32, shift: u8) -> u32 {
    (mask & reg_value) >> shift
}

pub unsafe extern "C" fn dmub_reg_update(
    srv: *mut dmub_srv,
    addr: u32,
    n: i32,
    shift1: u8,
    mask1: u32,
    field_value1: u32,
    mut args: ...,
) {
    let mut field_value_mask = dmub_reg_value_masks { value: 0, mask: 0 };

    set_reg_field_values(
        &mut field_value_mask,
        addr,
        n,
        shift1,
        mask1,
        field_value1,
        args,
    );

    let mut reg_val = (*srv).funcs.reg_read((*srv).user_ctx, addr);
    reg_val = (reg_val & !field_value_mask.mask) | field_value_mask.value;
    (*srv).funcs.reg_write((*srv).user_ctx, addr, reg_val);
}

pub unsafe extern "C" fn dmub_reg_set(
    srv: *mut dmub_srv,
    addr: u32,
    mut reg_val: u32,
    n: i32,
    shift1: u8,
    mask1: u32,
    field_value1: u32,
    mut args: ...,
) {
    let mut field_value_mask = dmub_reg_value_masks { value: 0, mask: 0 };

    set_reg_field_values(
        &mut field_value_mask,
        addr,
        n,
        shift1,
        mask1,
        field_value1,
        args,
    );

    reg_val = (reg_val & !field_value_mask.mask) | field_value_mask.value;
    (*srv).funcs.reg_write((*srv).user_ctx, addr, reg_val);
}

pub unsafe extern "C" fn dmub_reg_get(
    srv: *mut dmub_srv,
    addr: u32,
    shift: u8,
    mask: u32,
    field_value: *mut u32,
) {
    let reg_val = (*srv).funcs.reg_read((*srv).user_ctx, addr);
    *field_value = get_reg_field_value_ex(reg_val, mask, shift);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
