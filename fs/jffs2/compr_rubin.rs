/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 * Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * Created by Arjan van de Ven <arjanv@redhat.com>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

// Linux headers and compr.h provide the external types, constants, and functions used here.

const RUBIN_REG_SIZE: i32 = 16;
const UPPER_BIT_RUBIN: i64 = 1i64 << (RUBIN_REG_SIZE - 1);
const LOWER_BITS_RUBIN: i64 = (1i64 << (RUBIN_REG_SIZE - 1)) - 1;
const BIT_DIVIDER_MIPS: i32 = 1043;
static mut bits_mips: [i32; 8] = [277, 249, 290, 267, 229, 341, 212, 241];

#[repr(C)]
struct pushpull {
    buf: *mut u8,
    buflen: u32,
    ofs: u32,
    reserve: u32,
}

#[repr(C)]
struct rubin_state {
    p: u64,
    q: u64,
    rec_q: u64,
    bit_number: i64,
    pp: pushpull,
    bit_divider: i32,
    bits: [i32; 8],
}

unsafe fn init_pushpull(pp: *mut pushpull, buf: *mut u8, buflen: u32, ofs: u32, reserve: u32) {
    (*pp).buf = buf;
    (*pp).buflen = buflen;
    (*pp).ofs = ofs;
    (*pp).reserve = reserve;
}

unsafe fn pushbit(pp: *mut pushpull, bit: i32, use_reserved: i32) -> i32 {
    if (*pp).ofs >= (*pp).buflen - if use_reserved != 0 { 0 } else { (*pp).reserve } { return -28; }
    let p = (*pp).buf.add(((*pp).ofs >> 3) as usize);
    if bit != 0 { *p |= 1 << (7 - ((*pp).ofs & 7)); }
    else { *p &= !(1 << (7 - ((*pp).ofs & 7))); }
    (*pp).ofs += 1;
    0
}

unsafe fn pushedbits(pp: *mut pushpull) -> i32 { (*pp).ofs as i32 }

unsafe fn pullbit(pp: *mut pushpull) -> i32 {
    let bit = (*(*pp).buf.add(((*pp).ofs >> 3) as usize) >> (7 - ((*pp).ofs & 7))) & 1;
    (*pp).ofs += 1;
    bit as i32
}

unsafe fn init_rubin(rs: *mut rubin_state, div: i32, bits: *const i32) {
    (*rs).q = 0; (*rs).p = (2 * UPPER_BIT_RUBIN) as u64; (*rs).bit_number = 0; (*rs).bit_divider = div;
    for c in 0..8 { (*rs).bits[c] = *bits.add(c); }
}

unsafe fn encode(rs: *mut rubin_state, a: i64, b: i64, symbol: i32) -> i32 {
    while (*rs).q >= UPPER_BIT_RUBIN as u64 || ((*rs).p + (*rs).q) <= UPPER_BIT_RUBIN as u64 {
        (*rs).bit_number += 1;
        let ret = pushbit(&mut (*rs).pp, if ((*rs).q as i64 & UPPER_BIT_RUBIN) != 0 { 1 } else { 0 }, 0);
        if ret != 0 { return ret; }
        (*rs).q = (((*rs).q as i64 & LOWER_BITS_RUBIN) << 1) as u64; (*rs).p <<= 1;
    }
    let mut i0 = a * (*rs).p as i64 / (a + b); if i0 <= 0 { i0 = 1; } if i0 >= (*rs).p as i64 { i0 = (*rs).p as i64 - 1; }
    let i1 = (*rs).p as i64 - i0;
    if symbol == 0 { (*rs).p = i0 as u64; } else { (*rs).p = i1 as u64; (*rs).q += i0 as u64; }
    0
}

unsafe fn end_rubin(rs: *mut rubin_state) { for _ in 0..RUBIN_REG_SIZE { pushbit(&mut (*rs).pp, if ((*rs).q as i64 & UPPER_BIT_RUBIN) != 0 { 1 } else { 0 }, 1); (*rs).q = (((*rs).q as i64 & LOWER_BITS_RUBIN) << 1) as u64; } }

unsafe fn init_decode(rs: *mut rubin_state, div: i32, bits: *const i32) { init_rubin(rs, div, bits); (*rs).rec_q = 0; (*rs).bit_number = 0; while (*rs).bit_number < RUBIN_REG_SIZE as i64 { (*rs).bit_number += 1; (*rs).rec_q = (*rs).rec_q * 2 + pullbit(&mut (*rs).pp) as u64; } }

unsafe fn do_decode(rs: *mut rubin_state, mut p: u64, mut q: u64) {
    let mut bits = 0; loop { bits += 1; q = ((q as i64 & LOWER_BITS_RUBIN) << 1) as u64; p <<= 1; if !(q >= UPPER_BIT_RUBIN as u64 || p + q <= UPPER_BIT_RUBIN as u64) { break; } }
    (*rs).p = p; (*rs).q = q; (*rs).bit_number += bits;
    let mut rec_q = (*rs).rec_q; for _ in 0..bits { let c = pullbit(&mut (*rs).pp); rec_q = (((rec_q as i64 & LOWER_BITS_RUBIN) << 1) + c as i64) as u64; } (*rs).rec_q = rec_q;
}

unsafe fn decode(rs: *mut rubin_state, a: i64, b: i64) -> i32 {
    if (*rs).q >= UPPER_BIT_RUBIN as u64 || (*rs).p + (*rs).q <= UPPER_BIT_RUBIN as u64 { do_decode(rs, (*rs).p, (*rs).q); }
    let mut i0 = a * (*rs).p as i64 / (a + b); if i0 <= 0 { i0 = 1; } if i0 >= (*rs).p as i64 { i0 = (*rs).p as i64 - 1; }
    let threshold = (*rs).q as i64 + i0; let symbol = if (*rs).rec_q as i64 >= threshold { 1 } else { 0 }; if symbol != 0 { (*rs).q += i0 as u64; i0 = (*rs).p as i64 - i0; } (*rs).p = i0 as u64; symbol
}

unsafe fn out_byte(rs: *mut rubin_state, mut byte: u8) -> i32 { let copy = *rs; for i in 0..8 { let ret = encode(rs, ((*rs).bit_divider - (*rs).bits[i]) as i64, (*rs).bits[i] as i64, (byte & 1) as i32); if ret != 0 { *rs = copy; return ret; } byte >>= 1; } 0 }
unsafe fn in_byte(rs: *mut rubin_state) -> i32 { let mut result = 0; for i in 0..8 { result |= decode(rs, ((*rs).bit_divider - (*rs).bits[i]) as i64, (*rs).bits[i] as i64) << i; } result }

unsafe fn rubin_do_compress(bit_divider: i32, bits: *const i32, data_in: *mut u8, cpage_out: *mut u8, sourcelen: *mut u32, dstlen: *mut u32) -> i32 {
    let mut rs = rubin_state { p: 0, q: 0, rec_q: 0, bit_number: 0, pp: pushpull { buf: core::ptr::null_mut(), buflen: 0, ofs: 0, reserve: 0 }, bit_divider: 0, bits: [0; 8] };
    init_pushpull(&mut rs.pp, cpage_out, *dstlen * 8, 0, 32); init_rubin(&mut rs, bit_divider, bits); let mut pos = 0; while pos < *sourcelen && out_byte(&mut rs, *data_in.add(pos as usize)) == 0 { pos += 1; } end_rubin(&mut rs); let outpos = (pushedbits(&mut rs.pp) + 7) / 8; if outpos >= pos as i32 { return -1; } *sourcelen = pos; *dstlen = outpos as u32; 0
}

unsafe fn jffs2_dynrubin_compress(data_in: *mut u8, cpage_out: *mut u8, sourcelen: *mut u32, dstlen: *mut u32) -> i32 {
    let mut bits = [0i32; 8]; let mut histo = [0u8; 256]; let mut mysrclen = *sourcelen; let mut mydstlen = *dstlen - 8; if *dstlen <= 12 { return -1; }
    for i in 0..mysrclen { histo[*data_in.add(i as usize) as usize] = histo[*data_in.add(i as usize) as usize].wrapping_add(1); }
    for i in 0..256 { for b in 0..8 { if i & (1 << b) != 0 { bits[b] += histo[i] as i32; } } }
    for i in 0..8 { bits[i] = (bits[i] * 256) / mysrclen as i32; if bits[i] == 0 { bits[i] = 1; } if bits[i] > 255 { bits[i] = 255; } *cpage_out.add(i) = bits[i] as u8; }
    if rubin_do_compress(256, bits.as_ptr(), data_in, cpage_out.add(8), &mut mysrclen, &mut mydstlen) != 0 { return -1; } mydstlen += 8; if mysrclen <= mydstlen { return -1; } *sourcelen = mysrclen; *dstlen = mydstlen; 0
}

unsafe fn rubin_do_decompress(bit_divider: i32, bits: *const i32, cdata_in: *mut u8, page_out: *mut u8, srclen: u32, destlen: u32) {
    let mut rs = rubin_state { p: 0, q: 0, rec_q: 0, bit_number: 0, pp: pushpull { buf: core::ptr::null_mut(), buflen: 0, ofs: 0, reserve: 0 }, bit_divider: 0, bits: [0; 8] };
    init_pushpull(&mut rs.pp, cdata_in, srclen, 0, 0); init_decode(&mut rs, bit_divider, bits); for outpos in 0..destlen { *page_out.add(outpos as usize) = in_byte(&mut rs) as u8; }
}

unsafe fn jffs2_rubinmips_decompress(data_in: *mut u8, cpage_out: *mut u8, sourcelen: u32, dstlen: u32) -> i32 { rubin_do_decompress(BIT_DIVIDER_MIPS, bits_mips.as_ptr(), data_in, cpage_out, sourcelen, dstlen); 0 }
unsafe fn jffs2_dynrubin_decompress(data_in: *mut u8, cpage_out: *mut u8, sourcelen: u32, dstlen: u32) -> i32 { let mut bits = [0i32; 8]; for c in 0..8 { bits[c] = *data_in.add(c) as i32; } rubin_do_decompress(256, bits.as_ptr(), data_in.add(8), cpage_out, sourcelen - 8, dstlen); 0 }

// The compressor descriptor type and registration functions are supplied by compr.h.
extern "C" {
    fn jffs2_register_compressor(comp: *mut jffs2_compressor) -> i32;
    fn jffs2_unregister_compressor(comp: *mut jffs2_compressor);
}

#[repr(C)]
struct jffs2_compressor {
    priority: i32, name: *const u8, compr: i32,
    compress: Option<unsafe extern "C" fn(*mut u8, *mut u8, *mut u32, *mut u32) -> i32>,
    decompress: Option<unsafe extern "C" fn(*mut u8, *mut u8, u32, u32) -> i32>,
    disabled: i32,
}

static mut jffs2_rubinmips_comp: jffs2_compressor = jffs2_compressor { priority: 0, name: b"rubinmips\0".as_ptr(), compr: 0, compress: None, decompress: Some(jffs2_rubinmips_decompress), disabled: 0 };
static mut jffs2_dynrubin_comp: jffs2_compressor = jffs2_compressor { priority: 0, name: b"dynrubin\0".as_ptr(), compr: 0, compress: Some(jffs2_dynrubin_compress), decompress: Some(jffs2_dynrubin_decompress), disabled: 0 };

#[no_mangle] pub unsafe extern "C" fn jffs2_rubinmips_init() -> i32 { jffs2_register_compressor(&mut jffs2_rubinmips_comp) }
#[no_mangle] pub unsafe extern "C" fn jffs2_rubinmips_exit() { jffs2_unregister_compressor(&mut jffs2_rubinmips_comp); }
#[no_mangle] pub unsafe extern "C" fn jffs2_dynrubin_init() -> i32 { jffs2_register_compressor(&mut jffs2_dynrubin_comp) }
#[no_mangle] pub unsafe extern "C" fn jffs2_dynrubin_exit() { jffs2_unregister_compressor(&mut jffs2_dynrubin_comp); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
