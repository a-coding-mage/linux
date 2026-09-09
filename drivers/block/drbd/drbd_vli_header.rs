/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of drbd_vli.h. */

/*
 * Variable-length integer encoding and bitstream helpers for bitmap transfer.
 * The encoding is a little-endian, least-significant-bit-first bitstream.
 */

#[repr(C)]
pub struct bitstream_cursor {
    /* the current byte */
    pub b: *mut u8,
    /* the current bit within *b, normalized: 0..7 */
    pub bit: u32,
}

#[repr(C)]
pub struct bitstream {
    pub cur: bitstream_cursor,
    pub buf: *mut u8,
    pub buf_len: usize,
    /* number of trailing 0 bits for padding */
    pub pad_bits: u32,
}

/* finds a suitable level to decode the least significant part of in.
 * returns number of bits consumed. BUG() for bad input. */
#[inline]
pub unsafe fn vli_decode_bits(out: *mut u64, in_: u64) -> i32 {
    let mut adj: u64 = 1;
    macro_rules! level {
        ($t:expr, $b:expr, $v:expr) => {
            if (in_ & ((1u64 << $b) - 1)) == $v {
                *out = ((in_ & (!0u64 >> (64 - $t))) >> $b) + adj;
                return $t;
            }
            adj += 1u64 << ($t - $b);
        };
    }
    level!(2, 1, 0x00);
    level!(3, 2, 0x01);
    level!(5, 3, 0x03);
    level!(7, 4, 0x07);
    level!(10, 5, 0x0f);
    level!(14, 6, 0x1f);
    level!(21, 8, 0x3f);
    level!(29, 8, 0x7f);
    level!(42, 8, 0xbf);
    level!(64, 8, 0xff);
    BUG();
}

/* return number of code bits needed, or negative error number */
#[inline]
pub unsafe fn __vli_encode_bits(out: *mut u64, in_: u64) -> i32 {
    let mut max: u64 = 0;
    let mut adj: u64 = 1;
    if in_ == 0 { return -EINVAL; }
    macro_rules! level {
        ($t:expr, $b:expr, $v:expr) => {
            max += 1u64 << ($t - $b);
            if in_ <= max {
                if !out.is_null() { *out = ((in_ - adj) << $b) | $v; }
                return $t;
            }
            adj = max + 1;
        };
    }
    level!(2, 1, 0x00);
    level!(3, 2, 0x01);
    level!(5, 3, 0x03);
    level!(7, 4, 0x07);
    level!(10, 5, 0x0f);
    level!(14, 6, 0x1f);
    level!(21, 8, 0x3f);
    level!(29, 8, 0x7f);
    level!(42, 8, 0xbf);
    level!(64, 8, 0xff);
    -EOVERFLOW
}

#[inline]
pub unsafe fn bitstream_cursor_reset(cur: *mut bitstream_cursor, s: *mut u8) {
    (*cur).b = s;
    (*cur).bit = 0;
}

#[inline]
pub unsafe fn bitstream_cursor_advance(cur: *mut bitstream_cursor, mut bits: u32) {
    bits += (*cur).bit;
    (*cur).b = (*cur).b.add((bits >> 3) as usize);
    (*cur).bit = bits & 7;
}

#[inline]
pub unsafe fn bitstream_init(bs: *mut bitstream, s: *mut u8, len: usize, pad_bits: u32) {
    (*bs).buf = s;
    (*bs).buf_len = len;
    (*bs).pad_bits = pad_bits;
    bitstream_cursor_reset(&mut (*bs).cur, (*bs).buf);
}

#[inline]
pub unsafe fn bitstream_rewind(bs: *mut bitstream) {
    bitstream_cursor_reset(&mut (*bs).cur, (*bs).buf);
    memset((*bs).buf as *mut _, 0, (*bs).buf_len);
}

#[inline]
pub unsafe fn bitstream_put_bits(bs: *mut bitstream, mut val: u64, bits: u32) -> i32 {
    let mut b = (*bs).cur.b;
    if bits == 0 { return 0; }
    let used = b.offset_from((*bs).buf) as usize + ((((*bs).cur.bit + bits - 1) >> 3) as usize);
    if used >= (*bs).buf_len { return -ENOBUFS; }
    if bits < 64 { val &= !0u64 >> (64 - bits); }
    *b |= ((val & 0xff) as u8) << (*bs).cur.bit;
    let mut tmp = 8 - (*bs).cur.bit;
    while tmp < bits {
        b = b.add(1);
        *b |= ((val >> tmp) & 0xff) as u8;
        tmp += 8;
    }
    bitstream_cursor_advance(&mut (*bs).cur, bits);
    bits as i32
}

#[inline]
pub unsafe fn bitstream_get_bits(bs: *mut bitstream, out: *mut u64, mut bits: i32) -> i32 {
    if bits > 64 { return -EINVAL; }
    let end = (*bs).cur.b.offset_from((*bs).buf) as usize
        + ((((*bs).cur.bit + (*bs).pad_bits + bits as u32 - 1) >> 3) as usize);
    if end >= (*bs).buf_len {
        bits = (((*bs).buf_len - (*bs).cur.b.offset_from((*bs).buf) as usize) << 3) as i32
            - (*bs).cur.bit as i32 - (*bs).pad_bits as i32;
    }
    if bits == 0 { *out = 0; return 0; }
    let n = ((*bs).cur.bit + bits as u32 + 7) >> 3;
    let mut val = 0u64;
    if n != 0 {
        memcpy(&mut val as *mut _ as *mut _, (*bs).cur.b.add(1) as *const _, (n - 1) as usize);
        val = le64_to_cpu(val) << (8 - (*bs).cur.bit);
    }
    val |= *(*bs).cur.b >> (*bs).cur.bit;
    val &= !0u64 >> (64 - bits as u32);
    bitstream_cursor_advance(&mut (*bs).cur, bits as u32);
    *out = val;
    bits
}

#[inline]
pub unsafe fn vli_encode_bits(bs: *mut bitstream, in_: u64) -> i32 {
    let mut code = 0u64;
    let bits = __vli_encode_bits(&mut code, in_);
    if bits <= 0 { return bits; }
    bitstream_put_bits(bs, code, bits as u32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
