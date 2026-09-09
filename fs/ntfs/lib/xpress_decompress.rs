// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * xpress_decompress.c - A decompressor for the XPRESS compression format
 * (Huffman variant), which can be used in "System Compressed" (WOF) files.
 *
 * This is a port of the upstream wimlib "xpress_decompress.c" which uses a
 * subtable-based Huffman decode table format.  The decode table and the codeword-length
 * array share a union since the lengths are fully consumed before the table is written.
 *
 * Copyright (C) 2012-2016 Eric Biggers
 */

use core::ffi::c_void;

pub const XPRESS_NUM_CHARS: usize = 256;
pub const XPRESS_NUM_SYMBOLS: usize = 512;
pub const XPRESS_MAX_CODEWORD_LEN: usize = 15;
pub const XPRESS_MIN_MATCH_LEN: u32 = 3;
pub const XPRESS_TABLEBITS: usize = 11;

/* The concrete decode-table and input-bitstream layouts are supplied by the
 * surrounding decompression implementation. */
#[repr(C)]
pub struct InputBitstream {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct XpressDecompressor {
    pub lens: [u8; XPRESS_NUM_SYMBOLS],
    pub decode_table: *mut c_void,
    pub working_space: *mut c_void,
}

extern "C" {
    fn make_huffman_decode_table(
        decode_table: *mut c_void,
        num_symbols: usize,
        table_bits: usize,
        lens: *const u8,
        max_codeword_len: usize,
        working_space: *mut c_void,
        decode_table_size: usize,
    ) -> i32;
    fn init_input_bitstream(is: *mut InputBitstream, data: *const u8, size: usize);
    fn read_huffsym(
        is: *mut InputBitstream,
        decode_table: *mut c_void,
        table_bits: usize,
        max_codeword_len: usize,
    ) -> u32;
    fn bitstream_ensure_bits(is: *mut InputBitstream, nbits: u32);
    fn bitstream_pop_bits(is: *mut InputBitstream, nbits: usize) -> u32;
    fn bitstream_read_byte(is: *mut InputBitstream) -> u32;
    fn bitstream_read_u16(is: *mut InputBitstream) -> u32;
    fn lz_copy(
        length: u32,
        offset: u32,
        out_begin: *mut u8,
        out_next: *mut u8,
        out_end: *mut u8,
        min_match_len: u32,
    ) -> i32;
    fn kmalloc_obj(size: usize, flags: u32) -> *mut XpressDecompressor;
    fn kfree(ptr: *mut XpressDecompressor);
}

pub const GFP_NOFS: u32 = 0;

pub unsafe fn xpress_decompress(
    d: *mut XpressDecompressor,
    compressed_data: *const c_void,
    compressed_size: usize,
    uncompressed_data: *mut c_void,
    uncompressed_size: usize,
) -> i32 {
    let in_begin = compressed_data as *const u8;
    let out_begin = uncompressed_data as *mut u8;
    let mut out_next = out_begin;
    let out_end = out_begin.add(uncompressed_size);
    let mut is = InputBitstream { _opaque: [] };

    /* Read the Huffman codeword lengths (512 4-bit values packed into 256 bytes). */
    if compressed_size < XPRESS_NUM_SYMBOLS / 2 {
        return -1;
    }
    for i in 0..(XPRESS_NUM_SYMBOLS / 2) {
        (*d).lens[2 * i] = in_begin.add(i).read() & 0xf;
        (*d).lens[2 * i + 1] = in_begin.add(i).read() >> 4;
    }

    /* Build a decoding table for the Huffman code. */
    if make_huffman_decode_table(
        (*d).decode_table,
        XPRESS_NUM_SYMBOLS,
        XPRESS_TABLEBITS,
        (*d).lens.as_ptr(),
        XPRESS_MAX_CODEWORD_LEN,
        (*d).working_space,
        0,
    ) != 0 {
        return -1;
    }

    init_input_bitstream(
        &mut is,
        in_begin.add(XPRESS_NUM_SYMBOLS / 2),
        compressed_size - XPRESS_NUM_SYMBOLS / 2,
    );

    while out_next != out_end {
        let sym = read_huffsym(
            &mut is,
            (*d).decode_table,
            XPRESS_TABLEBITS,
            XPRESS_MAX_CODEWORD_LEN,
        );
        if sym < XPRESS_NUM_CHARS as u32 {
            out_next.write(sym as u8);
            out_next = out_next.add(1);
        } else {
            let mut length = sym & 0xf;
            let log2_offset = (sym >> 4) & 0xf;

            bitstream_ensure_bits(&mut is, 16);
            let offset = (1u32 << log2_offset) | bitstream_pop_bits(&mut is, log2_offset as usize);

            if length == 0xf {
                length += bitstream_read_byte(&mut is);
                if length == 0xf + 0xff {
                    length = bitstream_read_u16(&mut is);
                }
            }
            length += XPRESS_MIN_MATCH_LEN;

            if lz_copy(length, offset, out_begin, out_next, out_end, XPRESS_MIN_MATCH_LEN) != 0 {
                return -1;
            }
            out_next = out_next.add(length as usize);
        }
    }
    0
}

pub unsafe fn xpress_allocate_decompressor() -> *mut XpressDecompressor {
    kmalloc_obj(core::mem::size_of::<XpressDecompressor>(), GFP_NOFS)
}

pub unsafe fn xpress_free_decompressor(d: *mut XpressDecompressor) {
    kfree(d);
}

unsafe fn xpress_scratch_size(_chunk_size: u32) -> usize {
    core::mem::size_of::<XpressDecompressor>()
}

unsafe fn xpress_decompress_chunk(
    scratch: *mut c_void,
    src: *const c_void,
    src_len: usize,
    dst: *mut c_void,
    dst_len: usize,
    _chunk_size: u32,
) -> i32 {
    xpress_decompress(scratch as *mut XpressDecompressor, src, src_len, dst, dst_len)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
