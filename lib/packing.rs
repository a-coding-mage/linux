// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/* Copyright 2016-2018 NXP
 * Copyright (c) 2018-2019, Vladimir Oltean <olteanv@gmail.com>
 */

// Types, constants, structures, and external symbols supplied by linux/packing.h
// and the other included headers are intentionally referenced here as dependencies.

unsafe fn calculate_box_addr(box_: usize, len: usize, quirks: u8) -> usize {
    let this_group = box_ / 4;
    let offset_of_group = if quirks & QUIRK_LSW32_IS_FIRST != 0 {
        this_group * 4
    } else {
        len - (this_group + 1) * 4
    };
    let group_size = core::cmp::min(4, len - offset_of_group);
    let offset_in_group = if quirks & QUIRK_LITTLE_ENDIAN != 0 {
        box_ - this_group * 4
    } else {
        group_size - (box_ - this_group * 4) - 1
    };
    offset_of_group + offset_in_group
}

unsafe fn __pack(pbuf: *mut core::ffi::c_void, uval: u64, startbit: usize,
                 endbit: usize, pbuflen: usize, quirks: u8) {
    let plogical_first_u8 = (startbit / BITS_PER_BYTE) as isize;
    let plogical_last_u8 = (endbit / BITS_PER_BYTE) as isize;
    let value_width = startbit - endbit + 1;
    if value_width < 64 && uval >= (1u64 << value_width) {
        WARN!("Cannot store 0x{:x} inside bits {}-{} - will truncate\n", uval, startbit, endbit);
    }
    let mut box_ = plogical_first_u8;
    while box_ >= plogical_last_u8 {
        let box_start_bit = if box_ == plogical_first_u8 { startbit % BITS_PER_BYTE } else { 7 };
        let box_end_bit = if box_ == plogical_last_u8 { endbit % BITS_PER_BYTE } else { 0 };
        let proj_start_bit = ((box_ as usize * BITS_PER_BYTE) + box_start_bit) - endbit;
        let proj_end_bit = ((box_ as usize * BITS_PER_BYTE) + box_end_bit) - endbit;
        let proj_mask = genmask_ull(proj_start_bit, proj_end_bit);
        let mut box_mask = genmask(box_start_bit, box_end_bit);
        let box_addr = calculate_box_addr(box_ as usize, pbuflen, quirks);
        let mut pval = (uval & proj_mask) >> proj_end_bit << box_end_bit;
        if quirks & QUIRK_MSB_ON_THE_RIGHT != 0 {
            pval = bitrev8(pval);
            box_mask = bitrev8(box_mask as u64) as u8;
        }
        let p = (pbuf as *mut u8).add(box_addr);
        *p &= !box_mask;
        *p |= pval as u8;
        box_ -= 1;
    }
}

pub unsafe fn pack(pbuf: *mut core::ffi::c_void, uval: u64, startbit: usize,
                   endbit: usize, pbuflen: usize, quirks: u8) -> i32 {
    if startbit < endbit || startbit >= BITS_PER_BYTE * pbuflen { return -EINVAL; }
    if startbit - endbit >= 64 { return -ERANGE; }
    __pack(pbuf, uval, startbit, endbit, pbuflen, quirks);
    0
}

unsafe fn __unpack(pbuf: *const core::ffi::c_void, uval: *mut u64, startbit: usize,
                   endbit: usize, pbuflen: usize, quirks: u8) {
    let first = (startbit / BITS_PER_BYTE) as isize;
    let last = (endbit / BITS_PER_BYTE) as isize;
    *uval = 0;
    let mut box_ = first;
    while box_ >= last {
        let box_start_bit = if box_ == first { startbit % BITS_PER_BYTE } else { 7 };
        let box_end_bit = if box_ == last { endbit % BITS_PER_BYTE } else { 0 };
        let proj_start_bit = ((box_ as usize * BITS_PER_BYTE) + box_start_bit) - endbit;
        let proj_end_bit = ((box_ as usize * BITS_PER_BYTE) + box_end_bit) - endbit;
        let proj_mask = genmask_ull(proj_start_bit, proj_end_bit);
        let box_mask = genmask(box_start_bit, box_end_bit);
        let box_addr = calculate_box_addr(box_ as usize, pbuflen, quirks);
        let mut pval = *((pbuf as *const u8).add(box_addr)) as u64;
        if quirks & QUIRK_MSB_ON_THE_RIGHT != 0 { pval = bitrev8(pval); }
        pval = (pval & box_mask as u64) >> box_end_bit << proj_end_bit;
        *uval = (*uval & !proj_mask) | pval;
        box_ -= 1;
    }
}

pub unsafe fn unpack(pbuf: *const core::ffi::c_void, uval: *mut u64, startbit: usize,
                     endbit: usize, pbuflen: usize, quirks: u8) -> i32 {
    if startbit < endbit || startbit >= BITS_PER_BYTE * pbuflen { return -EINVAL; }
    if startbit - endbit + 1 > 64 { return -ERANGE; }
    __unpack(pbuf, uval, startbit, endbit, pbuflen, quirks);
    0
}

pub unsafe fn packing(pbuf: *mut core::ffi::c_void, uval: *mut u64, startbit: i32,
                      endbit: i32, pbuflen: usize, op: packing_op, quirks: u8) -> i32 {
    if op == PACK { pack(pbuf, *uval, startbit as usize, endbit as usize, pbuflen, quirks) }
    else { unpack(pbuf, uval, startbit as usize, endbit as usize, pbuflen, quirks) }
}

unsafe fn ustruct_field_to_u64(ustruct: *const core::ffi::c_void, offset: usize, size: usize) -> u64 {
    let p = (ustruct as *const u8).add(offset);
    match size { 1 => *(p as *const u8) as u64, 2 => *(p as *const u16) as u64,
        4 => *(p as *const u32) as u64, _ => *(p as *const u64) }
}

unsafe fn u64_to_ustruct_field(ustruct: *mut core::ffi::c_void, offset: usize, size: usize, v: u64) {
    let p = (ustruct as *mut u8).add(offset);
    match size { 1 => *(p as *mut u8) = v as u8, 2 => *(p as *mut u16) = v as u16,
        4 => *(p as *mut u32) = v as u32, _ => *(p as *mut u64) = v }
}

pub unsafe fn pack_fields_u8(p: *mut core::ffi::c_void, l: usize, s: *const core::ffi::c_void, f: *const packed_field_u8, n: usize, q: u8) { for i in 0..n { let x = &*f.add(i); __pack(p, ustruct_field_to_u64(s, x.offset, x.size), x.startbit, x.endbit, l, q); } }
pub unsafe fn pack_fields_u16(p: *mut core::ffi::c_void, l: usize, s: *const core::ffi::c_void, f: *const packed_field_u16, n: usize, q: u8) { for i in 0..n { let x = &*f.add(i); __pack(p, ustruct_field_to_u64(s, x.offset, x.size), x.startbit, x.endbit, l, q); } }
pub unsafe fn unpack_fields_u8(p: *const core::ffi::c_void, l: usize, s: *mut core::ffi::c_void, f: *const packed_field_u8, n: usize, q: u8) { for i in 0..n { let x = &*f.add(i); let mut v = 0; __unpack(p, &mut v, x.startbit, x.endbit, l, q); u64_to_ustruct_field(s, x.offset, x.size, v); } }
pub unsafe fn unpack_fields_u16(p: *const core::ffi::c_void, l: usize, s: *mut core::ffi::c_void, f: *const packed_field_u16, n: usize, q: u8) { for i in 0..n { let x = &*f.add(i); let mut v = 0; __unpack(p, &mut v, x.startbit, x.endbit, l, q); u64_to_ustruct_field(s, x.offset, x.size, v); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
