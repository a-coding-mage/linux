// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * lzx_decompress.c - A decompressor for the LZX compression format, which can
 * be used in "System Compressed" files.  This is based on the code from wimlib.
 * This code only supports a window size (dictionary size) of 32768 bytes, since
 * this is the only size used in System Compression.
 *
 * Copyright (C) 2015 Eric Biggers
 */

const LZX_NUM_CHARS: usize = 256;
const LZX_MIN_MATCH_LEN: u32 = 2;
const LZX_MAX_MATCH_LEN: u32 = 257;
const LZX_NUM_LENS: usize = (LZX_MAX_MATCH_LEN - LZX_MIN_MATCH_LEN + 1) as usize;
const LZX_NUM_PRIMARY_LENS: usize = 7;
const LZX_NUM_LEN_HEADERS: usize = LZX_NUM_PRIMARY_LENS + 1;
const LZX_BLOCKTYPE_VERBATIM: i32 = 1;
const LZX_BLOCKTYPE_ALIGNED: i32 = 2;
const LZX_BLOCKTYPE_UNCOMPRESSED: i32 = 3;
const LZX_NUM_OFFSET_SLOTS: usize = 30;
const LZX_MAINCODE_NUM_SYMBOLS: usize = LZX_NUM_CHARS + LZX_NUM_OFFSET_SLOTS * LZX_NUM_LEN_HEADERS;
const LZX_LENCODE_NUM_SYMBOLS: usize = LZX_NUM_LENS - LZX_NUM_PRIMARY_LENS;
const LZX_PRECODE_NUM_SYMBOLS: usize = 20;
const LZX_PRECODE_ELEMENT_SIZE: u32 = 4;
const LZX_NUM_ALIGNED_OFFSET_BITS: u32 = 3;
const LZX_ALIGNEDCODE_NUM_SYMBOLS: usize = 1 << LZX_NUM_ALIGNED_OFFSET_BITS;
const LZX_ALIGNED_OFFSET_BITMASK: u32 = (1 << LZX_NUM_ALIGNED_OFFSET_BITS) - 1;
const LZX_ALIGNEDCODE_ELEMENT_SIZE: u32 = 3;
const LZX_MAX_MAIN_CODEWORD_LEN: u32 = 16;
const LZX_MAX_LEN_CODEWORD_LEN: u32 = 16;
const LZX_MAX_PRE_CODEWORD_LEN: u32 = (1 << LZX_PRECODE_ELEMENT_SIZE) - 1;
const LZX_MAX_ALIGNED_CODEWORD_LEN: u32 = (1 << LZX_ALIGNEDCODE_ELEMENT_SIZE) - 1;
const LZX_DEFAULT_FILESIZE: i32 = 12000000;
const LZX_DEFAULT_BLOCK_SIZE: u32 = 32768;
const LZX_NUM_RECENT_OFFSETS: usize = 3;
const LZX_MAINCODE_TABLEBITS: usize = 11;
const LZX_LENCODE_TABLEBITS: usize = 10;
const LZX_PRECODE_TABLEBITS: usize = 6;
const LZX_ALIGNEDCODE_TABLEBITS: usize = 7;
const LZX_READ_LENS_MAX_OVERRUN: usize = 50;

static LZX_OFFSET_SLOT_BASE: [u32; LZX_NUM_OFFSET_SLOTS + 1] = [
    0, 1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 192,
    256, 384, 512, 768, 1024, 1536, 2048, 3072, 4096, 6144, 8192,
    12288, 16384, 24576, 32768,
];
static LZX_EXTRA_OFFSET_BITS: [u32; LZX_NUM_OFFSET_SLOTS] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8,
    9, 9, 10, 10, 11, 11, 12, 12, 13, 13,
];

#[repr(C)]
pub struct lzx_decompressor {
    maincode_decode_table: [u16; (1 << LZX_MAINCODE_TABLEBITS) + LZX_MAINCODE_NUM_SYMBOLS * 2],
    maincode_lens: [u8; LZX_MAINCODE_NUM_SYMBOLS + LZX_READ_LENS_MAX_OVERRUN],
    lencode_decode_table: [u16; (1 << LZX_LENCODE_TABLEBITS) + LZX_LENCODE_NUM_SYMBOLS * 2],
    lencode_lens: [u8; LZX_LENCODE_NUM_SYMBOLS + LZX_READ_LENS_MAX_OVERRUN],
    alignedcode_decode_table: [u16; (1 << LZX_ALIGNEDCODE_TABLEBITS) + LZX_ALIGNEDCODE_NUM_SYMBOLS * 2],
    alignedcode_lens: [u8; LZX_ALIGNEDCODE_NUM_SYMBOLS],
    precode_decode_table: [u16; (1 << LZX_PRECODE_TABLEBITS) + LZX_PRECODE_NUM_SYMBOLS * 2],
    precode_lens: [u8; LZX_PRECODE_NUM_SYMBOLS],
    working_space: [u16; 2 * (1 + LZX_MAX_MAIN_CODEWORD_LEN as usize) + LZX_MAINCODE_NUM_SYMBOLS],
}

/* External declarations supplied by the surrounding implementation. */
extern "C" {
    fn get_unaligned_le32(p: *const u8) -> i32;
    fn put_unaligned_le32(v: i32, p: *mut u8);
    fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn memset(dst: *mut u8, c: i32, n: usize) -> *mut u8;
    fn read_huffsym(is: *mut input_bitstream, table: *const u16, tablebits: usize, max_len: u32) -> u32;
    fn make_huffman_decode_table(table: *mut u16, num_symbols: usize, tablebits: usize,
                                 lens: *const u8, max_len: u32, working_space: *mut u16) -> i32;
    fn bitstream_read_bits(is: *mut input_bitstream, n: u32) -> u32;
    fn bitstream_ensure_bits(is: *mut input_bitstream, n: u32);
    fn bitstream_pop_bits(is: *mut input_bitstream, n: u32) -> u32;
    fn bitstream_align(is: *mut input_bitstream);
    fn bitstream_read_u32(is: *mut input_bitstream) -> u32;
    fn bitstream_read_bytes(is: *mut input_bitstream, out: *mut u8, n: u32) -> *mut u8;
    fn bitstream_read_byte(is: *mut input_bitstream) -> u8;
    fn init_input_bitstream(is: *mut input_bitstream, data: *const core::ffi::c_void, size: usize);
    fn lz_copy(out: *mut u8, len: u32, offset: u32, end: *mut u8, min_len: u32) -> *mut u8;
    fn kmalloc_obj_lzx_decompressor(flags: u32) -> *mut lzx_decompressor;
    fn kfree(p: *mut lzx_decompressor);
}

#[repr(C)]
pub struct input_bitstream { _private: [u8; 0] }

unsafe fn undo_e8_translation(target: *mut u8, input_pos: i32) {
    let abs_offset = get_unaligned_le32(target);
    if abs_offset >= 0 {
        if abs_offset < LZX_DEFAULT_FILESIZE {
            put_unaligned_le32(abs_offset - input_pos, target);
        }
    } else if abs_offset >= -input_pos {
        put_unaligned_le32(abs_offset + LZX_DEFAULT_FILESIZE, target);
    }
}

unsafe fn lzx_postprocess(data: *mut u8, size: u32) {
    if size <= 10 { return; }
    let tail = data.add((size - 6) as usize);
    let mut saved_bytes = [0u8; 6];
    memcpy(saved_bytes.as_mut_ptr(), tail, 6);
    memset(tail, 0xe8, 6);
    let mut p = data;
    loop {
        while *p != 0xe8 { p = p.add(1); }
        if p >= tail { break; }
        undo_e8_translation(p.add(1), p.offset_from(data) as i32);
        p = p.add(5);
    }
    memcpy(tail, saved_bytes.as_ptr(), 6);
}

unsafe fn read_presym(d: *const lzx_decompressor, is: *mut input_bitstream) -> u32 { read_huffsym(is, (*d).precode_decode_table.as_ptr(), LZX_PRECODE_TABLEBITS, LZX_MAX_PRE_CODEWORD_LEN) }
unsafe fn read_mainsym(d: *const lzx_decompressor, is: *mut input_bitstream) -> u32 { read_huffsym(is, (*d).maincode_decode_table.as_ptr(), LZX_MAINCODE_TABLEBITS, LZX_MAX_MAIN_CODEWORD_LEN) }
unsafe fn read_lensym(d: *const lzx_decompressor, is: *mut input_bitstream) -> u32 { read_huffsym(is, (*d).lencode_decode_table.as_ptr(), LZX_LENCODE_TABLEBITS, LZX_MAX_LEN_CODEWORD_LEN) }
unsafe fn read_alignedsym(d: *const lzx_decompressor, is: *mut input_bitstream) -> u32 { read_huffsym(is, (*d).alignedcode_decode_table.as_ptr(), LZX_ALIGNEDCODE_TABLEBITS, LZX_MAX_ALIGNED_CODEWORD_LEN) }

unsafe fn lzx_read_codeword_lens(d: *mut lzx_decompressor, is: *mut input_bitstream, lens: *mut u8, num_lens: u32) -> i32 {
    for i in 0..LZX_PRECODE_NUM_SYMBOLS { (*d).precode_lens[i] = bitstream_read_bits(is, LZX_PRECODE_ELEMENT_SIZE) as u8; }
    if make_huffman_decode_table((*d).precode_decode_table.as_mut_ptr(), LZX_PRECODE_NUM_SYMBOLS, LZX_PRECODE_TABLEBITS, (*d).precode_lens.as_ptr(), LZX_MAX_PRE_CODEWORD_LEN, (*d).working_space.as_mut_ptr()) != 0 { return -1; }
    let mut len_ptr = lens;
    let lens_end = lens.add(num_lens as usize);
    while len_ptr < lens_end {
        let mut presym = read_presym(d, is);
        let len: u8;
        let run_len: u32;
        if presym < 17 {
            len = (*len_ptr).wrapping_sub(presym as u8) % 17;
            *len_ptr = len; len_ptr = len_ptr.add(1);
            continue;
        } else if presym == 17 { run_len = 4 + bitstream_read_bits(is, 4); len = 0; }
        else if presym == 18 { run_len = 20 + bitstream_read_bits(is, 5); len = 0; }
        else {
            run_len = 4 + bitstream_read_bits(is, 1); presym = read_presym(d, is);
            if presym > 17 { return -1; }
            len = (*len_ptr).wrapping_sub(presym as u8) % 17;
        }
        for _ in 0..run_len { *len_ptr = len; len_ptr = len_ptr.add(1); }
    }
    0
}

unsafe fn lzx_read_block_header(d: *mut lzx_decompressor, is: *mut input_bitstream, block_type_ret: *mut i32, block_size_ret: *mut u32, recent_offsets: *mut u32) -> i32 {
    bitstream_ensure_bits(is, 4);
    let block_type = bitstream_pop_bits(is, 3) as i32;
    let block_size = if bitstream_pop_bits(is, 1) != 0 { LZX_DEFAULT_BLOCK_SIZE } else { (bitstream_read_bits(is, 8) << 8) | bitstream_read_bits(is, 8) };
    match block_type {
        LZX_BLOCKTYPE_ALIGNED => {
            for i in 0..LZX_ALIGNEDCODE_NUM_SYMBOLS { (*d).alignedcode_lens[i] = bitstream_read_bits(is, LZX_ALIGNEDCODE_ELEMENT_SIZE) as u8; }
            if make_huffman_decode_table((*d).alignedcode_decode_table.as_mut_ptr(), LZX_ALIGNEDCODE_NUM_SYMBOLS, LZX_ALIGNEDCODE_TABLEBITS, (*d).alignedcode_lens.as_ptr(), LZX_MAX_ALIGNED_CODEWORD_LEN, (*d).working_space.as_mut_ptr()) != 0 { return -1; }
            /* fall through */
            if lzx_read_codeword_lens(d, is, (*d).maincode_lens.as_mut_ptr(), LZX_NUM_CHARS as u32) != 0 { return -1; }
            if lzx_read_codeword_lens(d, is, (*d).maincode_lens.as_mut_ptr().add(LZX_NUM_CHARS), (LZX_MAINCODE_NUM_SYMBOLS - LZX_NUM_CHARS) as u32) != 0 { return -1; }
            if make_huffman_decode_table((*d).maincode_decode_table.as_mut_ptr(), LZX_MAINCODE_NUM_SYMBOLS, LZX_MAINCODE_TABLEBITS, (*d).maincode_lens.as_ptr(), LZX_MAX_MAIN_CODEWORD_LEN, (*d).working_space.as_mut_ptr()) != 0 { return -1; }
            if lzx_read_codeword_lens(d, is, (*d).lencode_lens.as_mut_ptr(), LZX_LENCODE_NUM_SYMBOLS as u32) != 0 { return -1; }
            if make_huffman_decode_table((*d).lencode_decode_table.as_mut_ptr(), LZX_LENCODE_NUM_SYMBOLS, LZX_LENCODE_TABLEBITS, (*d).lencode_lens.as_ptr(), LZX_MAX_LEN_CODEWORD_LEN, (*d).working_space.as_mut_ptr()) != 0 { return -1; }
        }
        LZX_BLOCKTYPE_VERBATIM => {
            if lzx_read_codeword_lens(d, is, (*d).maincode_lens.as_mut_ptr(), LZX_NUM_CHARS as u32) != 0 || lzx_read_codeword_lens(d, is, (*d).maincode_lens.as_mut_ptr().add(LZX_NUM_CHARS), (LZX_MAINCODE_NUM_SYMBOLS - LZX_NUM_CHARS) as u32) != 0 { return -1; }
            if make_huffman_decode_table((*d).maincode_decode_table.as_mut_ptr(), LZX_MAINCODE_NUM_SYMBOLS, LZX_MAINCODE_TABLEBITS, (*d).maincode_lens.as_ptr(), LZX_MAX_MAIN_CODEWORD_LEN, (*d).working_space.as_mut_ptr()) != 0 || lzx_read_codeword_lens(d, is, (*d).lencode_lens.as_mut_ptr(), LZX_LENCODE_NUM_SYMBOLS as u32) != 0 { return -1; }
            if make_huffman_decode_table((*d).lencode_decode_table.as_mut_ptr(), LZX_LENCODE_NUM_SYMBOLS, LZX_LENCODE_TABLEBITS, (*d).lencode_lens.as_ptr(), LZX_MAX_LEN_CODEWORD_LEN, (*d).working_space.as_mut_ptr()) != 0 { return -1; }
        }
        LZX_BLOCKTYPE_UNCOMPRESSED => {
            bitstream_ensure_bits(is, 1); bitstream_align(is);
            for i in 0..3 { *recent_offsets.add(i) = bitstream_read_u32(is); if *recent_offsets.add(i) == 0 { return -1; } }
        }
        _ => return -1,
    }
    *block_type_ret = block_type; *block_size_ret = block_size; 0
}

unsafe fn lzx_decompress_block(d: *const lzx_decompressor, is: *mut input_bitstream, block_type: i32, block_size: u32, out_begin: *mut u8, mut out_next: *mut u8, recent_offsets: *mut u32) -> i32 {
    let block_end = out_next.add(block_size as usize);
    let ones_if_aligned: u32 = 0u32.wrapping_sub((block_type == LZX_BLOCKTYPE_ALIGNED) as u32);
    while out_next != block_end {
        let mut mainsym = read_mainsym(d, is);
        if mainsym < LZX_NUM_CHARS as u32 { *out_next = mainsym as u8; out_next = out_next.add(1); continue; }
        mainsym -= LZX_NUM_CHARS as u32;
        let mut match_len = mainsym % LZX_NUM_LEN_HEADERS as u32;
        let offset_slot = (mainsym / LZX_NUM_LEN_HEADERS as u32) as usize;
        if match_len == LZX_NUM_PRIMARY_LENS as u32 { match_len += read_lensym(d, is); }
        match_len += LZX_MIN_MATCH_LEN;
        let match_offset;
        if offset_slot < LZX_NUM_RECENT_OFFSETS {
            match_offset = *recent_offsets.add(offset_slot);
            core::ptr::swap(recent_offsets.add(offset_slot), recent_offsets);
        } else {
            let num_extra_bits = LZX_EXTRA_OFFSET_BITS[offset_slot];
            let mut off = LZX_OFFSET_SLOT_BASE[offset_slot];
            if (num_extra_bits & ones_if_aligned) >= LZX_NUM_ALIGNED_OFFSET_BITS {
                off += bitstream_read_bits(is, num_extra_bits - LZX_NUM_ALIGNED_OFFSET_BITS) << LZX_NUM_ALIGNED_OFFSET_BITS;
                off += read_alignedsym(d, is);
            } else { off += bitstream_read_bits(is, num_extra_bits); }
            match_offset = off - (LZX_NUM_RECENT_OFFSETS as u32 - 1);
            *recent_offsets.add(2) = *recent_offsets.add(1); *recent_offsets.add(1) = *recent_offsets; *recent_offsets = match_offset;
        }
        if match_len as usize > block_end.offset_from(out_next) as usize || match_offset as usize > out_next.offset_from(out_begin) as usize { return -1; }
        out_next = lz_copy(out_next, match_len, match_offset, block_end, LZX_MIN_MATCH_LEN);
    }
    0
}

pub unsafe fn lzx_allocate_decompressor() -> *mut lzx_decompressor { kmalloc_obj_lzx_decompressor(0) }

pub unsafe fn lzx_decompress(decompressor: *mut lzx_decompressor, compressed_data: *const core::ffi::c_void, compressed_size: usize, uncompressed_data: *mut core::ffi::c_void, uncompressed_size: usize) -> i32 {
    let d = decompressor; let out_begin = uncompressed_data as *mut u8; let mut out_next = out_begin; let out_end = out_begin.add(uncompressed_size); let mut is = core::mem::MaybeUninit::<input_bitstream>::uninit(); let mut recent_offsets = [1u32; LZX_NUM_RECENT_OFFSETS]; let mut e8_status = 0;
    init_input_bitstream(is.as_mut_ptr(), compressed_data, compressed_size);
    memset((*d).maincode_lens.as_mut_ptr(), 0, LZX_MAINCODE_NUM_SYMBOLS); memset((*d).lencode_lens.as_mut_ptr(), 0, LZX_LENCODE_NUM_SYMBOLS);
    while out_next != out_end {
        let mut block_type = 0; let mut block_size = 0;
        if lzx_read_block_header(d, is.as_mut_ptr(), &mut block_type, &mut block_size, recent_offsets.as_mut_ptr()) != 0 || block_size < 1 || block_size as usize > out_end.offset_from(out_next) as usize { return -1; }
        if block_type != LZX_BLOCKTYPE_UNCOMPRESSED {
            if lzx_decompress_block(d, is.as_mut_ptr(), block_type, block_size, out_begin, out_next, recent_offsets.as_mut_ptr()) != 0 { return -1; }
            e8_status |= ((*d).maincode_lens[0xe8] != 0) as i32; out_next = out_next.add(block_size as usize);
        } else { out_next = bitstream_read_bytes(is.as_mut_ptr(), out_next, block_size); if out_next.is_null() { return -1; } if block_size & 1 != 0 { bitstream_read_byte(is.as_mut_ptr()); } e8_status = 1; }
    }
    if e8_status != 0 { lzx_postprocess(out_begin, uncompressed_size as u32); } 0
}

pub unsafe fn lzx_free_decompressor(decompressor: *mut lzx_decompressor) { kfree(decompressor); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
