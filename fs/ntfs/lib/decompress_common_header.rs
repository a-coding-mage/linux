/* SPDX-License-Identifier: MIT */
/*
 * decompress_common.h - Code shared by the XPRESS and LZX decompressors
 *
 * This is a port of the upstream wimlib "decompress_common.h" which uses a
 * subtable-based Huffman decode table format, as opposed to the older
 * binary-tree-based format previously used in this library.
 *
 * Copyright (C) 2022 Eric Biggers
 */

use core::mem::size_of;
use core::ptr;

/* "Force inline" macro (not required, but helpful for performance). */
#[inline(always)]

/* Size of a machine word. */
pub const WORDBYTES: usize = size_of::<usize>();
pub const WORDBITS: usize = 8 * WORDBYTES;

/* Build-time configuration equivalent to CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS. */
pub const UNALIGNED_ACCESS_IS_FAST: bool = false;
/* Deprecated name kept for compatibility with the upstream source. */
pub const FAST_UNALIGNED_ACCESS: bool = UNALIGNED_ACCESS_IS_FAST;

#[inline(always)]
pub unsafe fn load_word_unaligned(p: *const core::ffi::c_void) -> usize {
    ptr::read_unaligned(p as *const usize)
}

#[inline(always)]
pub unsafe fn store_word_unaligned(v: usize, p: *mut core::ffi::c_void) {
    ptr::write_unaligned(p as *mut usize, v);
}

#[inline(always)]
pub unsafe fn copy_word_unaligned(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void) {
    store_word_unaligned(load_word_unaligned(src), dst);
}

#[inline(always)]
pub fn repeat_u16(b: u16) -> usize {
    let mut v = b as usize;
    debug_assert!(WORDBITS == 32 || WORDBITS == 64);
    v |= v << 16;
    v |= v << if WORDBITS == 64 { 32 } else { 0 };
    v
}

#[inline(always)]
pub fn repeat_byte(b: u8) -> usize { repeat_u16(((b as u16) << 8) | b as u16) }

#[repr(C)]
pub struct input_bitstream {
    /* Bits that have been read from the input buffer; the next bit is bit 31. */
    pub bitbuf: u32,
    /* Number of bits currently held in bitbuf. */
    pub bitsleft: u32,
    /* Pointer to the next byte to be retrieved from the input buffer. */
    pub next: *const u8,
    /* Pointer past the end of the input buffer. */
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
    if (*is).bitsleft >= num_bits { return; }
    if (*is).end.offset_from((*is).next) < 2 {
        (*is).bitsleft = 32;
        return;
    }
    let v = ptr::read_unaligned((*is).next as *const u16).to_le();
    (*is).bitbuf |= (v as u32) << (16 - (*is).bitsleft);
    (*is).next = (*is).next.add(2);
    (*is).bitsleft += 16;
}

#[inline(always)]
pub unsafe fn bitstream_peek_bits(is: *const input_bitstream, num_bits: u32) -> u32 {
    ((*is).bitbuf >> 1) >> (size_of::<u32>() as u32 * 8 - num_bits - 1)
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
    let v = *(*is).next;
    (*is).next = (*is).next.add(1);
    v
}

#[inline(always)]
pub unsafe fn bitstream_read_u16(is: *mut input_bitstream) -> u16 {
    if (*is).end.offset_from((*is).next) < 2 { return 0; }
    let v = ptr::read_unaligned((*is).next as *const u16).to_le();
    (*is).next = (*is).next.add(2);
    v
}

#[inline(always)]
pub unsafe fn bitstream_read_u32(is: *mut input_bitstream) -> u32 {
    if (*is).end.offset_from((*is).next) < 4 { return 0; }
    let v = ptr::read_unaligned((*is).next as *const u32).to_le();
    (*is).next = (*is).next.add(4);
    v
}

#[inline(always)]
pub unsafe fn bitstream_read_bytes(is: *mut input_bitstream, dst_buffer: *mut core::ffi::c_void, count: usize) -> i32 {
    if (*is).end.offset_from((*is).next) < count as isize { return -1; }
    ptr::copy_nonoverlapping((*is).next, dst_buffer as *mut u8, count);
    (*is).next = (*is).next.add(count);
    0
}

#[inline(always)]
pub unsafe fn bitstream_align(is: *mut input_bitstream) { (*is).bitsleft = 0; (*is).bitbuf = 0; }

pub const DECODE_TABLE_ALIGNMENT: usize = 16;
pub const DECODE_TABLE_SYMBOL_SHIFT: u32 = 4;
pub const DECODE_TABLE_MAX_SYMBOL: u16 = (1 << (16 - DECODE_TABLE_SYMBOL_SHIFT)) - 1;
pub const DECODE_TABLE_MAX_LENGTH: u16 = (1 << DECODE_TABLE_SYMBOL_SHIFT) - 1;
pub const DECODE_TABLE_LENGTH_MASK: u16 = DECODE_TABLE_MAX_LENGTH;

#[inline(always)]
pub const fn make_decode_table_entry(symbol: u16, length: u16) -> u16 { (symbol << 4) | length }

#[inline(always)]
pub unsafe fn read_huffsym(is: *mut input_bitstream, decode_table: *const u16, table_bits: u32, max_codeword_len: u32) -> u32 {
    bitstream_ensure_bits(is, max_codeword_len);
    let mut entry = *decode_table.add(bitstream_peek_bits(is, table_bits) as usize);
    let mut symbol = (entry >> 4) as u32;
    let mut length = (entry & DECODE_TABLE_LENGTH_MASK) as u32;
    if max_codeword_len > table_bits && entry as u32 >= (1u32 << (table_bits + 4)) {
        bitstream_remove_bits(is, table_bits);
        entry = *decode_table.add(symbol as usize + bitstream_peek_bits(is, length) as usize);
        symbol = (entry >> 4) as u32;
        length = (entry & DECODE_TABLE_LENGTH_MASK) as u32;
    }
    bitstream_remove_bits(is, length);
    symbol
}

/* DECODE_TABLE_ENOUGH is a compile-time mapping; unsupported combinations yield -1. */
#[macro_export]
macro_rules! decode_table_enough {
    (8,5,7)=>{36}; (8,6,7)=>{66}; (8,7,7)=>{128};
    (20,5,15)=>{1062}; (20,6,15)=>{582}; (20,7,15)=>{390};
    (54,9,15)=>{618}; (54,10,15)=>{1098};
    (249,9,16)=>{878}; (249,10,16)=>{1326}; (249,11,16)=>{2318};
    (496,11,16)=>{2566}; (256,9,15)=>{822}; (256,10,15)=>{1302}; (256,11,15)=>{2310};
    (512,10,15)=>{1558}; (512,11,15)=>{2566}; (512,12,15)=>{4606};
    (656,10,16)=>{1734}; (656,11,16)=>{2726}; (656,12,16)=>{4758};
    (799,9,15)=>{1366}; (799,10,15)=>{1846}; (799,11,15)=>{2854};
}

extern "C" {
    pub fn make_huffman_decode_table(decode_table: *mut u16, num_syms: u32, table_bits: u32,
        lens: *const u8, max_codeword_len: u32, working_space: *mut u16, decode_table_size: u32) -> i32;
}

#[inline(always)]
pub unsafe fn lz_copy(length: u32, offset: u32, out_begin: *mut u8, mut out_next: *mut u8,
                      out_end: *mut u8, min_length: u32) -> i32 {
    if offset > out_next.offset_from(out_begin) as u32 { return -1; }
    let mut src = out_next.sub(offset as usize);
    if UNALIGNED_ACCESS_IS_FAST && length <= 3 * WORDBYTES as u32 && offset >= WORDBYTES as u32 && out_end.offset_from(out_next) >= 3 * WORDBYTES as isize {
        for i in 0..3 { copy_word_unaligned(src.add(i * WORDBYTES) as *const _, out_next.add(i * WORDBYTES) as *mut _); }
        return 0;
    }
    if length > out_end.offset_from(out_next) as u32 { return -1; }
    let end = out_next.add(length as usize);
    if UNALIGNED_ACCESS_IS_FAST && out_end.offset_from(end) >= WORDBYTES as isize - 1 {
        if offset >= WORDBYTES as u32 {
            while out_next < end { copy_word_unaligned(src as *const _, out_next as *mut _); src = src.add(WORDBYTES); out_next = out_next.add(WORDBYTES); }
            return 0;
        } else if offset == 1 {
            let v = repeat_byte(*out_next.sub(1));
            while out_next < end { store_word_unaligned(v, out_next as *mut _); out_next = out_next.add(WORDBYTES); }
            return 0;
        }
    }
    if min_length >= 2 { *out_next = *src; out_next = out_next.add(1); src = src.add(1); }
    if min_length >= 3 { *out_next = *src; out_next = out_next.add(1); src = src.add(1); }
    while out_next != end { *out_next = *src; out_next = out_next.add(1); src = src.add(1); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
