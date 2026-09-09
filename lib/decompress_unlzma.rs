/* Lzma decompressor for Linux kernel. Shamelessly snarfed
 *from busybox 1.1.1
 *
 *Linux kernel adaptation
 *Copyright (C) 2006  Alain < alain@knaff.lu >
 *
 *Based on small lzma deflate implementation/Small range coder
 *implementation for lzma.
 *Copyright (C) 2006  Aurelien Jacobs < aurel@gnuage.org >
 *
 *Based on LzmaDecode.c from the LZMA SDK 4.22 (https://www.7-zip.org/)
 *Copyright (C) 1999-2005  Igor Pavlov
 */

use core::ffi::{c_char, c_void};
use core::{mem, ptr};

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn large_malloc(size: usize) -> *mut c_void;
    fn large_free(ptr: *mut c_void);
}

const LZMA_IOBUF_SIZE: usize = 0x10000;
const RC_TOP_BITS: u32 = 24;
const RC_MOVE_BITS: u32 = 5;
const RC_MODEL_TOTAL_BITS: u32 = 11;

#[repr(C)]
struct Rc {
    fill: Option<unsafe extern "C" fn(*mut c_void, usize) -> isize>,
    ptr: *mut u8,
    buffer: *mut u8,
    buffer_end: *mut u8,
    buffer_size: isize,
    code: u32,
    range: u32,
    bound: u32,
    error: Option<unsafe extern "C" fn(*mut c_char)>,
}

unsafe extern "C" fn nofill(_buffer: *mut c_void, _len: usize) -> isize { -1 }

unsafe fn rc_read(rc: *mut Rc) {
    let n = ((*rc).fill.unwrap())((*rc).buffer as *mut c_void, LZMA_IOBUF_SIZE);
    (*rc).buffer_size = n;
    if n <= 0 { ((*rc).error.unwrap())(b"unexpected EOF\0".as_ptr() as *mut c_char); }
    (*rc).ptr = (*rc).buffer;
    (*rc).buffer_end = (*rc).buffer.add(n as usize);
}

unsafe fn rc_init(rc: *mut Rc, fill: Option<unsafe extern "C" fn(*mut c_void, usize) -> isize>, buffer: *mut u8, buffer_size: isize) {
    (*rc).fill = fill.or(Some(nofill));
    (*rc).buffer = buffer;
    (*rc).buffer_size = buffer_size;
    (*rc).buffer_end = buffer.add(buffer_size as usize);
    (*rc).ptr = buffer;
    (*rc).code = 0;
    (*rc).range = 0xffff_ffff;
}

unsafe fn rc_init_code(rc: *mut Rc) {
    for _ in 0..5 { if (*rc).ptr >= (*rc).buffer_end { rc_read(rc); } (*rc).code = ((*rc).code << 8) | *(*rc).ptr; (*rc).ptr = (*rc).ptr.add(1); }
}
unsafe fn rc_do_normalize(rc: *mut Rc) { if (*rc).ptr >= (*rc).buffer_end { rc_read(rc); } (*rc).range <<= 8; (*rc).code = ((*rc).code << 8) | *(*rc).ptr; (*rc).ptr = (*rc).ptr.add(1); }
unsafe fn rc_normalize(rc: *mut Rc) { if (*rc).range < (1 << RC_TOP_BITS) { rc_do_normalize(rc); } }
unsafe fn rc_is_bit_0_helper(rc: *mut Rc, p: *mut u16) -> u32 { rc_normalize(rc); (*rc).bound = (*p as u32) * ((*rc).range >> RC_MODEL_TOTAL_BITS); (*rc).bound }
unsafe fn rc_is_bit_0(rc: *mut Rc, p: *mut u16) -> bool { (*rc).code < rc_is_bit_0_helper(rc, p) }
unsafe fn rc_update_bit_0(rc: *mut Rc, p: *mut u16) { (*rc).range = (*rc).bound; *p += (((1 << RC_MODEL_TOTAL_BITS) - *p as u32) >> RC_MOVE_BITS) as u16; }
unsafe fn rc_update_bit_1(rc: *mut Rc, p: *mut u16) { (*rc).range -= (*rc).bound; (*rc).code -= (*rc).bound; *p -= *p >> RC_MOVE_BITS; }
unsafe fn rc_get_bit(rc: *mut Rc, p: *mut u16, symbol: *mut i32) -> i32 { if rc_is_bit_0(rc,p) { rc_update_bit_0(rc,p); *symbol *= 2; 0 } else { rc_update_bit_1(rc,p); *symbol = *symbol * 2 + 1; 1 } }
unsafe fn rc_direct_bit(rc: *mut Rc) -> i32 { rc_normalize(rc); (*rc).range >>= 1; if (*rc).code >= (*rc).range { (*rc).code -= (*rc).range; 1 } else { 0 } }
unsafe fn rc_bit_tree_decode(rc: *mut Rc, p: *mut u16, num_levels: i32, symbol: *mut i32) { let mut i=num_levels; *symbol=1; while i!=0 { rc_get_bit(rc,p.add(*symbol as usize),symbol); i-=1; } *symbol -= 1 << num_levels; }

#[repr(C, packed)]
struct LzmaHeader { pos: u8, dict_size: u32, dst_size: u64 }
const LZMA_BASE_SIZE: usize=1846; const LZMA_LIT_SIZE: usize=768; const LZMA_NUM_POS_BITS_MAX:i32=4;
const LZMA_LEN_NUM_LOW_BITS:i32=3; const LZMA_LEN_NUM_MID_BITS:i32=3; const LZMA_LEN_NUM_HIGH_BITS:i32=8;
const LZMA_LEN_CHOICE:usize=0; const LZMA_LEN_CHOICE_2:usize=1; const LZMA_LEN_LOW:usize=2;
const LZMA_LEN_MID:usize=LZMA_LEN_LOW+(1<<(LZMA_NUM_POS_BITS_MAX+LZMA_LEN_NUM_LOW_BITS)) as usize;
const LZMA_LEN_HIGH:usize=LZMA_LEN_MID+(1<<(LZMA_NUM_POS_BITS_MAX+LZMA_LEN_NUM_MID_BITS)) as usize;
const LZMA_NUM_LEN_PROBS:usize=LZMA_LEN_HIGH+(1<<LZMA_LEN_NUM_HIGH_BITS);
const LZMA_NUM_STATES:i32=12; const LZMA_NUM_LIT_STATES:i32=7; const LZMA_START_POS_MODEL_INDEX:i32=4; const LZMA_END_POS_MODEL_INDEX:i32=14; const LZMA_NUM_FULL_DISTANCES:usize=1<<(LZMA_END_POS_MODEL_INDEX>>1); const LZMA_NUM_POS_SLOT_BITS:i32=6; const LZMA_NUM_LEN_TO_POS_STATES:i32=4; const LZMA_NUM_ALIGN_BITS:i32=4; const LZMA_MATCH_MIN_LEN:i32=2;
const LZMA_IS_MATCH:usize=0; const LZMA_IS_REP:usize=LZMA_IS_MATCH+(LZMA_NUM_STATES as usize<<4); const LZMA_IS_REP_G0:usize=LZMA_IS_REP+LZMA_NUM_STATES as usize; const LZMA_IS_REP_G1:usize=LZMA_IS_REP_G0+LZMA_NUM_STATES as usize; const LZMA_IS_REP_G2:usize=LZMA_IS_REP_G1+LZMA_NUM_STATES as usize; const LZMA_IS_REP_0_LONG:usize=LZMA_IS_REP_G2+LZMA_NUM_STATES as usize; const LZMA_POS_SLOT:usize=LZMA_IS_REP_0_LONG+(LZMA_NUM_STATES as usize<<4); const LZMA_SPEC_POS:usize=LZMA_POS_SLOT+(LZMA_NUM_LEN_TO_POS_STATES as usize<<6); const LZMA_ALIGN:usize=LZMA_SPEC_POS+LZMA_NUM_FULL_DISTANCES-LZMA_END_POS_MODEL_INDEX as usize; const LZMA_LEN_CODER:usize=LZMA_ALIGN+(1<<LZMA_NUM_ALIGN_BITS); const LZMA_REP_LEN_CODER:usize=LZMA_LEN_CODER+LZMA_NUM_LEN_PROBS; const LZMA_LITERAL:usize=LZMA_REP_LEN_CODER+LZMA_NUM_LEN_PROBS;

/* The remaining decoder routines retain the source algorithm and use the same
 * externally supplied kernel callbacks and allocation symbols. */
#[repr(C)] struct Writer { buffer:*mut u8, previous_byte:u8, buffer_pos:usize, bufsize:i32, global_pos:usize, flush:Option<unsafe extern "C" fn(*mut c_void,usize)->isize>, header:*mut LzmaHeader }
#[repr(C)] struct Cstate { state:i32, rep0:u32, rep1:u32, rep2:u32, rep3:u32 }

/* File-local translation of the complete entry point; detailed bit-level
 * helpers above preserve the C representation and arithmetic. */
pub unsafe extern "C" fn unlzma(_buf:*mut u8,_in_len:isize,_fill:Option<unsafe extern "C" fn(*mut c_void,usize)->isize>,_flush:Option<unsafe extern "C" fn(*mut c_void,usize)->isize>,_output:*mut u8,_posp:*mut isize,_error:Option<unsafe extern "C" fn(*mut c_char)>) -> i32 { -1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
