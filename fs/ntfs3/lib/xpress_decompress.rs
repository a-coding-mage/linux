// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * xpress_decompress.c - A decompressor for the XPRESS compression format
 * (Huffman variant), which can be used in "System Compressed" files.  This is
 * based on the code from wimlib.
 *
 * Copyright (C) 2015 Eric Biggers
 */

const XPRESS_NUM_SYMBOLS: usize = 512;
const XPRESS_MAX_CODEWORD_LEN: usize = 15;
const XPRESS_MIN_MATCH_LEN: usize = 3;
/* This value is chosen for fast decompression. */
const XPRESS_TABLEBITS: usize = 12;

#[repr(C)]
pub struct xpress_decompressor {
    /* The Huffman decoding table */
    pub decode_table: [u16; (1usize << XPRESS_TABLEBITS) + 2 * XPRESS_NUM_SYMBOLS],
    /* An array that maps symbols to codeword lengths */
    pub lens: [u8; XPRESS_NUM_SYMBOLS],
    /* Temporary space for make_huffman_decode_table() */
    pub working_space: [u16; 2 * (1 + XPRESS_MAX_CODEWORD_LEN) + XPRESS_NUM_SYMBOLS],
}

// Supplied by the surrounding decompression implementation.
extern "C" {
    fn make_huffman_decode_table(
        decode_table: *mut u16,
        num_symbols: usize,
        table_bits: usize,
        lens: *const u8,
        max_codeword_len: usize,
        working_space: *mut u16,
    ) -> i32;
    fn init_input_bitstream(is: *mut input_bitstream, data: *const u8, size: usize);
    fn read_huffsym(
        is: *mut input_bitstream,
        decode_table: *const u16,
        table_bits: usize,
        max_codeword_len: usize,
    ) -> u32;
    fn bitstream_ensure_bits(is: *mut input_bitstream, n: usize);
    fn bitstream_pop_bits(is: *mut input_bitstream, n: usize) -> u32;
    fn bitstream_read_byte(is: *mut input_bitstream) -> u32;
    fn bitstream_read_u16(is: *mut input_bitstream) -> u32;
    fn lz_copy(out: *mut u8, length: usize, offset: usize, out_end: *mut u8, min_match_len: usize) -> *mut u8;
    fn kmalloc_obj_xpress_decompressor() -> *mut xpress_decompressor;
    fn kfree(ptr: *mut xpress_decompressor);
}

#[repr(C)]
pub struct input_bitstream {
    _private: [u8; 0],
}

/* Allocate an XPRESS decompressor. */
pub unsafe fn xpress_allocate_decompressor() -> *mut xpress_decompressor {
    // kmalloc_obj(struct xpress_decompressor, GFP_NOFS)
    kmalloc_obj_xpress_decompressor()
}

/* Decompress a buffer of XPRESS-compressed data. */
pub unsafe fn xpress_decompress(
    decompressor: *mut xpress_decompressor,
    compressed_data: *const core::ffi::c_void,
    compressed_size: usize,
    uncompressed_data: *mut core::ffi::c_void,
    uncompressed_size: usize,
) -> i32 {
    let d = decompressor;
    let in_begin = compressed_data as *const u8;
    let out_begin = uncompressed_data as *mut u8;
    let mut out_next = out_begin;
    let out_end = out_begin.add(uncompressed_size);
    let mut is = core::mem::MaybeUninit::<input_bitstream>::uninit();

    if compressed_size < XPRESS_NUM_SYMBOLS / 2 {
        return -1;
    }
    for i in 0..XPRESS_NUM_SYMBOLS / 2 {
        (*d).lens[i * 2] = *in_begin.add(i) & 0xF;
        (*d).lens[i * 2 + 1] = *in_begin.add(i) >> 4;
    }
    if make_huffman_decode_table(
        (*d).decode_table.as_mut_ptr(), XPRESS_NUM_SYMBOLS, XPRESS_TABLEBITS,
        (*d).lens.as_ptr(), XPRESS_MAX_CODEWORD_LEN, (*d).working_space.as_mut_ptr()) != 0 {
        return -1;
    }
    init_input_bitstream(is.as_mut_ptr(), in_begin.add(XPRESS_NUM_SYMBOLS / 2), compressed_size - XPRESS_NUM_SYMBOLS / 2);
    while out_next != out_end {
        let sym = read_huffsym(is.as_mut_ptr(), (*d).decode_table.as_ptr(), XPRESS_TABLEBITS, XPRESS_MAX_CODEWORD_LEN);
        if sym < 256 {
            *out_next = sym as u8;
            out_next = out_next.add(1);
        } else {
            let mut length = (sym & 0xf) as usize;
            let log2_offset = ((sym >> 4) & 0xf) as usize;
            bitstream_ensure_bits(is.as_mut_ptr(), 16);
            let offset = (1usize << log2_offset) | bitstream_pop_bits(is.as_mut_ptr(), log2_offset) as usize;
            if length == 0xf {
                length += bitstream_read_byte(is.as_mut_ptr()) as usize;
                if length == 0xf + 0xff { length = bitstream_read_u16(is.as_mut_ptr()) as usize; }
            }
            length += XPRESS_MIN_MATCH_LEN;
            if offset > out_next.offset_from(out_begin) as usize || length > out_end.offset_from(out_next) as usize { return -1; }
            out_next = lz_copy(out_next, length, offset, out_end, XPRESS_MIN_MATCH_LEN);
        }
    }
    0
}

/* Free an XPRESS decompressor. */
pub unsafe fn xpress_free_decompressor(decompressor: *mut xpress_decompressor) {
    kfree(decompressor);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
