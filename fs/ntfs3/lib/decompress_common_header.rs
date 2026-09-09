/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * decompress_common.h - Code shared by the XPRESS and LZX decompressors
 *
 * Copyright (C) 2015 Eric Biggers
 */

// C header dependencies: linux/string.h, linux/compiler.h, linux/types.h,
// linux/slab.h, and linux/unaligned.h.

#[inline(always)]
pub unsafe fn copy_unaligned_word(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void) {
    let value = (src as *const usize).read_unaligned();
    (dst as *mut usize).write_unaligned(value);
}

#[inline(always)]
pub fn repeat_byte(b: u8) -> usize {
    let mut v = b as usize;
    v |= v << 8;
    v |= v << 16;
    v |= v << if core::mem::size_of::<usize>() == 8 { 32 } else { 0 };
    v
}

#[repr(C)]
pub struct input_bitstream {
    pub bitbuf: u32,
    pub bitsleft: u32,
    pub next: *const u8,
    pub end: *const u8,
}

#[inline(always)]
pub unsafe fn init_input_bitstream(is: *mut input_bitstream, buffer: *const core::ffi::c_void, size: u32) {
    (*is).bitbuf = 0;
    (*is).bitsleft = 0;
    (*is).next = buffer as *const u8;
    (*is).end = (*is).next.add(size as usize);
}

#[inline(always)]
pub unsafe fn bitstream_ensure_bits(is: *mut input_bitstream, num_bits: u32) {
    if (*is).bitsleft < num_bits {
        if (*is).end.offset_from((*is).next) >= 2 {
            let value = u16::from_le(((*is).next as *const u16).read_unaligned());
            (*is).bitbuf |= (value as u32) << (16 - (*is).bitsleft);
            (*is).next = (*is).next.add(2);
        }
        (*is).bitsleft += 16;
    }
}

#[inline(always)]
pub unsafe fn bitstream_peek_bits(is: *const input_bitstream, num_bits: u32) -> u32 {
    ((*is).bitbuf >> 1) >> (core::mem::size_of::<u32>() as u32 * 8 - num_bits - 1)
}

#[inline(always)]
pub unsafe fn bitstream_remove_bits(is: *mut input_bitstream, num_bits: u32) {
    (*is).bitbuf <<= num_bits;
    (*is).bitsleft -= num_bits;
}

#[inline(always)]
pub unsafe fn bitstream_pop_bits(is: *mut input_bitstream, num_bits: u32) -> u32 {
    let bits = bitstream_peek_bits(is, num_bits);
    bitstream_remove_bits(is, num_bits);
    bits
}

#[inline(always)]
pub unsafe fn bitstream_read_bits(is: *mut input_bitstream, num_bits: u32) -> u32 {
    bitstream_ensure_bits(is, num_bits);
    bitstream_pop_bits(is, num_bits)
}

#[inline(always)]
pub unsafe fn bitstream_read_byte(is: *mut input_bitstream) -> u8 {
    if (*is).end == (*is).next { return 0; }
    let value = *(*is).next;
    (*is).next = (*is).next.add(1);
    value
}

#[inline(always)]
pub unsafe fn bitstream_read_u16(is: *mut input_bitstream) -> u16 {
    if (*is).end.offset_from((*is).next) < 2 { return 0; }
    let value = u16::from_le(((*is).next as *const u16).read_unaligned());
    (*is).next = (*is).next.add(2);
    value
}

#[inline(always)]
pub unsafe fn bitstream_read_u32(is: *mut input_bitstream) -> u32 {
    if (*is).end.offset_from((*is).next) < 4 { return 0; }
    let value = u32::from_le(((*is).next as *const u32).read_unaligned());
    (*is).next = (*is).next.add(4);
    value
}

#[inline(always)]
pub unsafe fn bitstream_read_bytes(is: *mut input_bitstream, dst_buffer: *mut core::ffi::c_void, count: usize) -> *mut core::ffi::c_void {
    if (*is).end.offset_from((*is).next) as usize < count { return core::ptr::null_mut(); }
    core::ptr::copy_nonoverlapping((*is).next, dst_buffer as *mut u8, count);
    (*is).next = (*is).next.add(count);
    (dst_buffer as *mut u8).add(count) as *mut core::ffi::c_void
}

#[inline(always)]
pub unsafe fn bitstream_align(is: *mut input_bitstream) {
    (*is).bitsleft = 0;
    (*is).bitbuf = 0;
}

extern "C" {
    pub fn make_huffman_decode_table(decode_table: *mut u16, num_syms: u32, num_bits: u32,
                                     lens: *const u8, max_codeword_len: u32,
                                     working_space: *mut u16) -> i32;
}

#[inline(always)]
pub unsafe fn read_huffsym(istream: *mut input_bitstream, decode_table: *const u16,
                           table_bits: u32, max_codeword_len: u32) -> u32 {
    bitstream_ensure_bits(istream, max_codeword_len);
    let mut entry = *decode_table.add(bitstream_peek_bits(istream, table_bits) as usize);
    if entry < 0xC000 {
        bitstream_remove_bits(istream, (entry >> 11) as u32);
        return (entry & 0x7FF) as u32;
    }
    bitstream_remove_bits(istream, table_bits);
    loop {
        let key_bits = (entry & 0x3FFF) as u32 + bitstream_pop_bits(istream, 1);
        entry = *decode_table.add(key_bits as usize);
        if entry < 0xC000 { return entry as u32; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
