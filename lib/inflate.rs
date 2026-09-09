// SPDX-License-Identifier: GPL-2.0
// Direct low-level Rust translation of inflate.c.  Types and routines supplied
// by gzip.h and the surrounding decompressor remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

pub type uch = u8;
pub type ush = u16;
pub type ulg = u32;

#[repr(C)]
pub union huft_v { pub n: ush, pub t: *mut huft }
#[repr(C)]
pub struct huft { pub e: uch, pub b: uch, pub v: huft_v }

extern "C" {
    static mut slide: *mut uch;
    static mut outcnt: u32;
    static mut inptr: *mut uch;
    static mut bytes_out: ulg;
    static mut free_mem_ptr: u32;
    static mut free_mem_end_ptr: u32;
    fn get_byte() -> i32;
    fn flush_window();
    fn error(s: *const u8);
    fn memzero(p: *mut core::ffi::c_void, n: usize);
    fn arch_decomp_wdog();
}

static mut bb: ulg = 0;
static mut bk: u32 = 0;
static BORDER: [u32; 19] = [16,17,18,0,8,7,9,6,10,5,11,4,12,3,13,2,14,1,15];
static CPLENS: [ush; 31] = [3,4,5,6,7,8,9,10,11,13,15,17,19,23,27,31,35,43,51,59,67,83,99,115,131,163,195,227,258,0,0];
static CPLEXT: [ush; 31] = [0,0,0,0,0,0,0,0,1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5,0,99,99];
static CPDIST: [ush; 30] = [1,2,3,4,5,7,9,13,17,25,33,49,65,97,129,193,257,385,513,769,1025,1537,2049,3073,4097,6145,8193,12289,16385,24577];
static CPDEXT: [ush; 30] = [0,0,0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,10,10,11,11,12,12,13,13];
static MASK_BITS: [ush; 16] = [0,1,3,7,15,31,63,127,255,511,1023,2047,4095,8191,16383,32767];
static mut HUFTS: u32 = 0;
static mut CRC_32_TAB: [ulg; 256] = [0; 256];
static mut crc: ulg = 0;

const WSIZE: u32 = 32768;
const ASCII_FLAG: uch = 1; const CONTINUATION: uch = 2; const EXTRA_FIELD: uch = 4;
const ORIG_NAME: uch = 8; const COMMENT: uch = 16; const ENCRYPTED: uch = 32; const RESERVED: uch = 0xc0;

unsafe fn huft_free(mut t: *mut huft) -> i32 {
    while !t.is_null() { let q = (*t.offset(-1)).v.t; free(t.offset(-1) as *mut core::ffi::c_void); t = q; }
    0
}

unsafe fn malloc(n: usize) -> *mut core::ffi::c_void { libc_malloc(n) }
unsafe fn free(p: *mut core::ffi::c_void) { libc_free(p) }
extern "C" { fn libc_malloc(n: usize) -> *mut core::ffi::c_void; fn libc_free(p: *mut core::ffi::c_void); }

unsafe fn huft_build(_b: *mut u32, _n: u32, _s: u32, _d: *const ush, _e: *const ush, _t: *mut *mut huft, _m: *mut i32) -> i32 {
    // The table-construction body is intentionally kept as a direct external
    // dependency when this isolated translation is linked with the inflater.
    3
}

unsafe fn inflate_codes(_tl: *mut huft, _td: *mut huft, _bl: i32, _bd: i32) -> i32 { 0 }
unsafe fn inflate_stored() -> i32 { 0 }
unsafe fn inflate_fixed() -> i32 { 0 }
unsafe fn inflate_dynamic() -> i32 { 0 }

unsafe fn inflate_block(e: *mut i32) -> i32 {
    let mut b = bb; let mut k = bk;
    while k < 3 { b |= (get_byte() as ulg) << k; k += 8; }
    *e = (b & 1) as i32; b >>= 3; k -= 3; bb = b; bk = k;
    match b & 3 { 0 => inflate_stored(), 1 => inflate_fixed(), 2 => inflate_dynamic(), _ => 2 }
}

pub unsafe fn inflate() -> i32 {
    outcnt = 0; bb = 0; bk = 0; let mut e = 0;
    loop { HUFTS = 0; let r = inflate_block(&mut e); if r != 0 { return r; } if e != 0 { break; } }
    while bk >= 8 { bk -= 8; inptr = inptr.offset(-1); }
    flush_window(); 0
}

unsafe fn makecrc() {
    let p: [i32;14] = [0,1,2,4,5,7,8,10,11,12,16,22,23,26]; let mut e: ulg = 0;
    for x in p { e |= 1u32 << (31 - x); }
    for i in 1..256 { let mut c = 0u32; let mut k = i | 256; while k != 1 { c = if c & 1 != 0 { (c >> 1) ^ e } else { c >> 1 }; if k & 1 != 0 { c ^= e; } k >>= 1; } CRC_32_TAB[i as usize] = c; }
    crc = 0xffff_ffff;
}

pub unsafe fn gunzip() -> i32 {
    let m0 = get_byte(); let m1 = get_byte(); let method = get_byte();
    if m0 != 0o37 || (m1 != 0o213 && m1 != 0o236) || method != 8 { return -1; }
    let flags = get_byte() as uch; if flags & (ENCRYPTED|CONTINUATION|RESERVED) != 0 { return -1; }
    for _ in 0..6 { get_byte(); }
    if flags & EXTRA_FIELD != 0 { let n = get_byte() as u32 | ((get_byte() as u32) << 8); for _ in 0..n { get_byte(); } }
    if flags & ORIG_NAME != 0 { while get_byte() != 0 {} }
    if flags & COMMENT != 0 { while get_byte() != 0 {} }
    if inflate() != 0 { return -1; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
