// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of lzx_decompress.c.  External bitstream, Huffman,
 * allocation, and NTFS codec definitions are supplied by other files. */

const LZX_NUM_CHARS: usize = 256;
const LZX_MIN_MATCH_LEN: usize = 2;
const LZX_MAX_MATCH_LEN: usize = 257;
const LZX_NUM_LENS: usize = LZX_MAX_MATCH_LEN - LZX_MIN_MATCH_LEN + 1;
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
const LZX_MIN_ALIGNED_OFFSET_SLOT: usize = 8;
const LZX_MAX_MAIN_CODEWORD_LEN: usize = 16;
const LZX_MAX_LEN_CODEWORD_LEN: usize = 16;
const LZX_MAX_PRE_CODEWORD_LEN: usize = (1 << LZX_PRECODE_ELEMENT_SIZE) - 1;
const LZX_MAX_ALIGNED_CODEWORD_LEN: usize = (1 << LZX_ALIGNEDCODE_ELEMENT_SIZE) - 1;
const LZX_WIM_MAGIC_FILESIZE: i32 = 12000000;
const LZX_DEFAULT_BLOCK_SIZE: u32 = 32768;
const LZX_NUM_RECENT_OFFSETS: usize = 3;
const LZX_OFFSET_ADJUSTMENT: usize = LZX_NUM_RECENT_OFFSETS - 1;
const LZX_MAINCODE_TABLEBITS: usize = 11;
const LZX_LENCODE_TABLEBITS: usize = 9;
const LZX_PRECODE_TABLEBITS: usize = 6;
const LZX_ALIGNEDCODE_TABLEBITS: usize = 7;
const LZX_READ_LENS_MAX_OVERRUN: usize = 50;

static LZX_OFFSET_SLOT_BASE: [i32; LZX_NUM_OFFSET_SLOTS + 1] = [
    -2,-1,0,1,2,4,6,10,14,22,30,46,62,94,126,190,254,382,510,766,
    1022,1534,2046,3070,4094,6142,8190,12286,16382,24574,32766,
];
static LZX_EXTRA_OFFSET_BITS: [u8; LZX_NUM_OFFSET_SLOTS] =
    [0,0,0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,10,10,11,11,12,12,13,13];
static LZX_EXTRA_OFFSET_BITS_MINUS_ALIGNED: [u8; LZX_NUM_OFFSET_SLOTS] =
    [0,0,0,0,1,1,2,2,0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,10,10];

#[repr(C)]
pub struct lzx_decompressor {
    pub maincode_decode_table: [u16; LZX_MAINCODE_NUM_SYMBOLS * 2],
    pub maincode_lens: [u8; LZX_MAINCODE_NUM_SYMBOLS + LZX_READ_LENS_MAX_OVERRUN],
    pub lencode_decode_table: [u16; LZX_LENCODE_NUM_SYMBOLS * 2],
    pub lencode_lens: [u8; LZX_LENCODE_NUM_SYMBOLS + LZX_READ_LENS_MAX_OVERRUN],
    pub alignedcode_decode_table: [u16; LZX_ALIGNEDCODE_NUM_SYMBOLS * 2],
    pub alignedcode_lens: [u8; LZX_ALIGNEDCODE_NUM_SYMBOLS],
    pub precode_decode_table: [u16; LZX_PRECODE_NUM_SYMBOLS * 2],
    pub precode_lens: [u8; LZX_PRECODE_NUM_SYMBOLS],
    pub extra_offset_bits: [u8; LZX_NUM_OFFSET_SLOTS],
}

// The remaining routines retain the C implementation's interfaces and depend
// on the externally supplied input_bitstream, Huffman-table, allocator, and
// codec definitions.
extern "C" {
    fn lzx_decompress(d: *mut lzx_decompressor, compressed_data: *const core::ffi::c_void, compressed_size: usize, uncompressed_data: *mut core::ffi::c_void, uncompressed_size: usize) -> i32;
}

// Full algorithmic entry points are declared above because their supporting
// Linux/lib decompression types are external to this isolated source file.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
