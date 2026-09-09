// Translated from defutil.h. C includes and build-time definitions are supplied externally.

pub const LENGTH_CODES: usize = 29;
pub const LITERALS: usize = 256;
pub const L_CODES: usize = LITERALS + 1 + LENGTH_CODES;
pub const D_CODES: usize = 30;
pub const BL_CODES: usize = 19;
pub const HEAP_SIZE: usize = 2 * L_CODES + 1;
pub const MAX_BITS: usize = 15;
pub const INIT_STATE: i32 = 42;
pub const BUSY_STATE: i32 = 113;
pub const FINISH_STATE: i32 = 666;

#[repr(C)]
pub union ct_data_fc {
    pub freq: ush,
    pub code: ush,
}

#[repr(C)]
pub union ct_data_dl {
    pub dad: ush,
    pub len: ush,
}

#[repr(C)]
pub struct ct_data {
    pub fc: ct_data_fc,
    pub dl: ct_data_dl,
}

pub type static_tree_desc = static_tree_desc_s;
#[repr(C)]
pub struct static_tree_desc_s {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tree_desc {
    pub dyn_tree: *mut ct_data,
    pub max_code: i32,
    pub stat_desc: *mut static_tree_desc,
}

pub type Pos = ush;
pub type IPos = ::core::ffi::c_uint;

#[repr(C)]
pub struct deflate_state {
    pub strm: z_streamp,
    pub status: i32,
    pub pending_buf: *mut Byte,
    pub pending_buf_size: ulg,
    pub pending_out: *mut Byte,
    pub pending: i32,
    pub noheader: i32,
    pub data_type: Byte,
    pub method: Byte,
    pub last_flush: i32,
    pub w_size: uInt,
    pub w_bits: uInt,
    pub w_mask: uInt,
    pub window: *mut Byte,
    pub window_size: ulg,
    pub prev: *mut Pos,
    pub head: *mut Pos,
    pub ins_h: uInt,
    pub hash_size: uInt,
    pub hash_bits: uInt,
    pub hash_mask: uInt,
    pub hash_shift: uInt,
    pub block_start: ::core::ffi::c_long,
    pub match_length: uInt,
    pub prev_match: IPos,
    pub match_available: i32,
    pub strstart: uInt,
    pub match_start: uInt,
    pub lookahead: uInt,
    pub prev_length: uInt,
    pub max_chain_length: uInt,
    pub max_lazy_match: uInt,
    pub level: i32,
    pub strategy: i32,
    pub good_match: uInt,
    pub nice_match: i32,
    pub dyn_ltree: [ct_data; HEAP_SIZE],
    pub dyn_dtree: [ct_data; 2 * D_CODES + 1],
    pub bl_tree: [ct_data; 2 * BL_CODES + 1],
    pub l_desc: tree_desc,
    pub d_desc: tree_desc,
    pub bl_desc: tree_desc,
    pub bl_count: [ush; MAX_BITS + 1],
    pub heap: [i32; 2 * L_CODES + 1],
    pub heap_len: i32,
    pub heap_max: i32,
    pub depth: [uch; 2 * L_CODES + 1],
    pub l_buf: *mut uch,
    pub lit_bufsize: uInt,
    pub last_lit: uInt,
    pub d_buf: *mut ush,
    pub opt_len: ulg,
    pub static_len: ulg,
    pub compressed_len: ulg,
    pub matches: uInt,
    pub last_eob_len: i32,
    #[cfg(feature = "DEBUG_ZLIB")]
    pub bits_sent: ulg,
    pub bi_buf: ush,
    pub bi_valid: i32,
}

#[inline]
pub unsafe fn put_byte(s: *mut deflate_state, c: Byte) {
    (*s).pending_buf.add((*s).pending as usize).write(c);
    (*s).pending += 1;
}

pub const MIN_LOOKAHEAD: uInt = MAX_MATCH + MIN_MATCH + 1;
#[inline]
pub unsafe fn MAX_DIST(s: *const deflate_state) -> uInt { (*s).w_size - MIN_LOOKAHEAD }

extern "C" {
    pub fn zlib_tr_init(s: *mut deflate_state);
    pub fn zlib_tr_tally(s: *mut deflate_state, dist: u32, lc: u32) -> i32;
    pub fn zlib_tr_flush_block(s: *mut deflate_state, buf: *mut ::core::ffi::c_char, stored_len: ulg, eof: i32) -> ulg;
    pub fn zlib_tr_align(s: *mut deflate_state);
    pub fn zlib_tr_stored_block(s: *mut deflate_state, buf: *mut ::core::ffi::c_char, stored_len: ulg, eof: i32);
    pub fn zlib_tr_stored_type_only(s: *mut deflate_state);
}

#[inline]
pub unsafe fn put_short(s: *mut deflate_state, w: ush) {
    put_byte(s, (w & 0xff) as uch);
    put_byte(s, (w >> 8) as uch);
}

#[inline]
pub fn bi_reverse(mut code: u32, mut len: i32) -> u32 {
    let mut res = 0;
    loop { res |= code & 1; code >>= 1; res <<= 1; len -= 1; if len <= 0 { break; } }
    res >> 1
}

#[inline]
pub unsafe fn bi_flush(s: *mut deflate_state) {
    if (*s).bi_valid == 16 { put_short(s, (*s).bi_buf); (*s).bi_buf = 0; (*s).bi_valid = 0; }
    else if (*s).bi_valid >= 8 { put_byte(s, (*s).bi_buf as Byte); (*s).bi_buf >>= 8; (*s).bi_valid -= 8; }
}

#[inline]
pub unsafe fn bi_windup(s: *mut deflate_state) {
    if (*s).bi_valid > 8 { put_short(s, (*s).bi_buf); } else if (*s).bi_valid > 0 { put_byte(s, (*s).bi_buf as Byte); }
    (*s).bi_buf = 0; (*s).bi_valid = 0;
    #[cfg(feature = "DEBUG_ZLIB")]
    { (*s).bits_sent = ((*s).bits_sent + 7) & !7; }
}

#[repr(C)]
pub enum block_state { need_more, block_done, finish_started, finish_done }
pub const Buf_size: i32 = 8 * 2 * ::core::mem::size_of::<::core::ffi::c_char>() as i32;

#[inline]
pub unsafe fn zlib_tr_send_bits(s: *mut deflate_state, value: i32, length: i32) {
    if (*s).bi_valid > Buf_size - length {
        (*s).bi_buf |= (value << (*s).bi_valid) as ush;
        put_short(s, (*s).bi_buf);
        (*s).bi_buf = (value as ush) >> (Buf_size - (*s).bi_valid);
        (*s).bi_valid += length - Buf_size;
    } else { (*s).bi_buf |= (value << (*s).bi_valid) as ush; (*s).bi_valid += length; }
}

#[inline]
pub unsafe fn flush_pending(strm: z_streamp) {
    bi_flush((*strm).state as *mut deflate_state);
    let s = (*strm).state as *mut deflate_state;
    let mut len = (*s).pending as u32;
    if len > (*strm).avail_out { len = (*strm).avail_out; }
    if len == 0 { return; }
    if !(*strm).next_out.is_null() { ::core::ptr::copy_nonoverlapping((*s).pending_out, (*strm).next_out, len as usize); (*strm).next_out = (*strm).next_out.add(len as usize); }
    (*s).pending_out = (*s).pending_out.add(len as usize); (*strm).total_out += len; (*strm).avail_out -= len; (*s).pending -= len as i32;
    if (*s).pending == 0 { (*s).pending_out = (*s).pending_buf; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
