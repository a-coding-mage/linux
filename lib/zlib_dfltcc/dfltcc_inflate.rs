// SPDX-License-Identifier: Zlib

// Dependencies supplied by the surrounding zlib and kernel translation units.

use core::ffi::{c_int, c_void};

extern "C" {
    static mut zlib_dfltcc_support: c_int;
    fn dfltcc_reset_state(state: *mut dfltcc_state);
    fn dfltcc(
        op: c_int,
        param: *mut dfltcc_param_v0,
        next_out: *mut *mut u8,
        avail_out: *mut usize,
        next_in: *mut *const u8,
        avail_in: *mut usize,
        window: *mut u8,
    ) -> dfltcc_cc;
    fn is_bit_set(value: u64, bit: c_int) -> c_int;
    fn oesc_msg(msg: *mut u8, oesc: u8) -> *mut u8;
}

#[repr(C)]
pub struct z_stream {
    pub next_in: *const u8,
    pub avail_in: usize,
    pub next_out: *mut u8,
    pub avail_out: usize,
    pub msg: *mut u8,
    pub state: *mut c_void,
}
pub type z_streamp = *mut z_stream;

#[repr(C)]
pub struct dfltcc_af { pub fns: u64, pub fmts: u64 }
#[repr(C)]
pub struct dfltcc_param_v0 {
    pub nt: u8, pub cf: u8, pub hl: u8, pub cvt: u8, pub sbb: u8,
    pub oesc: u8, pub _pad: [u8; 2], pub cv: u32,
}
#[repr(C)]
pub struct dfltcc_state {
    pub af: dfltcc_af,
    pub param: dfltcc_param_v0,
    pub msg: *mut u8,
}
#[repr(C)]
pub struct inflate_state {
    pub window: *mut u8, pub wsize: usize, pub bits: u32, pub check: u32,
    pub last: c_int, pub mode: c_int,
    pub dfltcc: dfltcc_state,
}

#[allow(non_camel_case_types)]
pub type dfltcc_cc = c_int;
pub type dfltcc_inflate_action = c_int;

pub const ZLIB_DFLTCC_DISABLED: c_int = 0;
pub const ZLIB_DFLTCC_DEFLATE_ONLY: c_int = 1;
pub const DFLTCC_XPND: c_int = 0;
pub const DFLTCC_FMT0: c_int = 0;
pub const HBT_CIRCULAR: c_int = 0;
pub const CVT_ADLER32: u8 = 0;
pub const DFLTCC_CC_AGAIN: dfltcc_cc = 0;
pub const DFLTCC_CC_OK: dfltcc_cc = 1;
pub const DFLTCC_CC_OP1_TOO_SHORT: dfltcc_cc = 2;
pub const DFLTCC_CC_OP2_TOO_SHORT: dfltcc_cc = 3;
pub const DFLTCC_CC_OP2_CORRUPT: dfltcc_cc = 4;
pub const DFLTCC_INFLATE_BREAK: dfltcc_inflate_action = 0;
pub const DFLTCC_INFLATE_CONTINUE: dfltcc_inflate_action = 1;
pub const DFLTCC_INFLATE_SOFTWARE: dfltcc_inflate_action = 2;
pub const Z_BLOCK: c_int = 5;
pub const Z_PACKET_FLUSH: c_int = 6;
pub const Z_STREAM_ERROR: c_int = -2;
pub const CHECK: c_int = 0;
pub const MEM: c_int = 1;
pub const TYPEDO: c_int = 2;
pub const BAD: c_int = 3;

#[inline]
unsafe fn get_dfltcc_state(state: *mut inflate_state) -> *mut dfltcc_state {
    &mut (*state).dfltcc
}

pub unsafe fn dfltcc_can_inflate(strm: z_streamp) -> c_int {
    let state = (*strm).state as *mut inflate_state;
    let dfltcc_state = get_dfltcc_state(state);
    if zlib_dfltcc_support == ZLIB_DFLTCC_DISABLED || zlib_dfltcc_support == ZLIB_DFLTCC_DEFLATE_ONLY { return 0; }
    (is_bit_set((*dfltcc_state).af.fns, DFLTCC_XPND) != 0 && is_bit_set((*dfltcc_state).af.fmts, DFLTCC_FMT0) != 0) as c_int
}

pub unsafe fn dfltcc_reset_inflate_state(strm: z_streamp) {
    let state = (*strm).state as *mut inflate_state;
    dfltcc_reset_state(get_dfltcc_state(state));
}

unsafe fn dfltcc_was_inflate_used(strm: z_streamp) -> c_int {
    let state = (*strm).state as *mut inflate_state;
    (!((*get_dfltcc_state(state)).param.nt != 0)) as c_int
}

unsafe fn dfltcc_inflate_disable(strm: z_streamp) -> c_int {
    let state = (*strm).state as *mut inflate_state;
    let ds = get_dfltcc_state(state);
    if dfltcc_can_inflate(strm) == 0 { return 0; }
    if dfltcc_was_inflate_used(strm) != 0 { return 1; }
    core::ptr::write_bytes(&mut (*ds).af as *mut dfltcc_af, 0, 1);
    0
}

unsafe fn dfltcc_xpnd(strm: z_streamp) -> dfltcc_cc {
    let state = (*strm).state as *mut inflate_state;
    let param = &mut (*get_dfltcc_state(state)).param as *mut dfltcc_param_v0;
    let mut avail_in = (*strm).avail_in;
    let mut avail_out = (*strm).avail_out;
    let cc = dfltcc(DFLTCC_XPND | HBT_CIRCULAR, param, &mut (*strm).next_out, &mut avail_out, &mut (*strm).next_in, &mut avail_in, (*state).window);
    (*strm).avail_in = avail_in; (*strm).avail_out = avail_out; cc
}

pub unsafe fn dfltcc_inflate(strm: z_streamp, flush: c_int, ret: *mut c_int) -> dfltcc_inflate_action {
    let state = (*strm).state as *mut inflate_state;
    let ds = get_dfltcc_state(state);
    let param = &mut (*ds).param as *mut dfltcc_param_v0;
    if flush == Z_BLOCK || flush == Z_PACKET_FLUSH {
        if dfltcc_inflate_disable(strm) != 0 { *ret = Z_STREAM_ERROR; return DFLTCC_INFLATE_BREAK; }
        return DFLTCC_INFLATE_SOFTWARE;
    }
    if (*state).last != 0 {
        if (*state).bits != 0 { (*strm).next_in = (*strm).next_in.add(1); (*strm).avail_in -= 1; (*state).bits = 0; }
        (*state).mode = CHECK; return DFLTCC_INFLATE_CONTINUE;
    }
    if (*strm).avail_in == 0 && (*param).cf == 0 { return DFLTCC_INFLATE_BREAK; }
    if (*state).window.is_null() || (*state).wsize == 0 { (*state).mode = MEM; return DFLTCC_INFLATE_CONTINUE; }
    (*param).cvt = CVT_ADLER32; (*param).sbb = (*state).bits as u8;
    if (*param).hl != 0 { (*param).nt = 0; } (*param).cv = (*state).check;
    let mut cc; loop { cc = dfltcc_xpnd(strm); if cc != DFLTCC_CC_AGAIN { break; } }
    (*strm).msg = oesc_msg((*ds).msg, (*param).oesc); (*state).last = (cc == DFLTCC_CC_OK) as c_int;
    (*state).bits = (*param).sbb as u32; (*state).check = (*param).cv;
    if cc == DFLTCC_CC_OP2_CORRUPT && (*param).oesc != 0 { (*state).mode = BAD; return DFLTCC_INFLATE_CONTINUE; }
    (*state).mode = TYPEDO;
    if cc == DFLTCC_CC_OP1_TOO_SHORT || cc == DFLTCC_CC_OP2_TOO_SHORT { DFLTCC_INFLATE_BREAK } else { DFLTCC_INFLATE_CONTINUE }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
