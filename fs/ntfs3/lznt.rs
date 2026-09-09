// SPDX-License-Identifier: GPL-2.0
/*
 *
 * Copyright (C) 2019-2021 Paragon Software GmbH, All rights reserved.
 *
 */

// Kernel headers and local headers from the C source are external Rust dependencies.

const LZNT_ERROR_ALL_ZEROS: i32 = 1;
const LZNT_CHUNK_SIZE: usize = 0x1000;

#[repr(C)]
struct lznt_hash {
    p1: *const u8,
    p2: *const u8,
}

#[repr(C)]
struct lznt {
    unc: *const u8,
    unc_end: *const u8,
    best_match: *const u8,
    max_len: usize,
    std: bool,
    hash: [lznt_hash; LZNT_CHUNK_SIZE],
}

#[inline]
unsafe fn get_match_len(ptr: *const u8, end: *const u8, prev: *const u8, max_len: usize) -> usize {
    let mut len = 0usize;
    while ptr.add(len) < end && *ptr.add(len) == *prev.add(len) {
        len += 1;
        if len >= max_len { break; }
    }
    len
}

unsafe fn longest_match_std(src: *const u8, ctx: *mut lznt) -> usize {
    let hash_index = ((40543u32 * (((((*src as u32) << 4) ^ *src.add(1) as u32) << 4) ^ *src.add(2) as u32)) >> 4) as usize & (LZNT_CHUNK_SIZE - 1);
    let hash = &mut (*ctx).hash[hash_index];
    let mut len1 = 0usize;
    let mut len2 = 0usize;
    if hash.p1 >= (*ctx).unc && hash.p1 < src && *hash.p1 == *src && *hash.p1.add(1) == *src.add(1) && *hash.p1.add(2) == *src.add(2) {
        len1 = 3;
        if (*ctx).max_len > 3 { len1 += get_match_len(src.add(3), (*ctx).unc_end, hash.p1.add(3), (*ctx).max_len - 3); }
    }
    if hash.p2 >= (*ctx).unc && hash.p2 < src && *hash.p2 == *src && *hash.p2.add(1) == *src.add(1) && *hash.p2.add(2) == *src.add(2) {
        len2 = 3;
        if (*ctx).max_len > 3 { len2 += get_match_len(src.add(3), (*ctx).unc_end, hash.p2.add(3), (*ctx).max_len - 3); }
    }
    if len1 < len2 { (*ctx).best_match = hash.p2; len1 = len2; } else { (*ctx).best_match = hash.p1; }
    hash.p2 = hash.p1;
    hash.p1 = src;
    len1
}

unsafe fn longest_match_best(src: *const u8, ctx: *mut lznt) -> usize {
    if (*ctx).unc >= src || (*ctx).max_len == 0 { return 0; }
    let mut max_len = 0usize;
    let mut ptr = (*ctx).unc;
    while ptr < src {
        let len = get_match_len(src, (*ctx).unc_end, ptr, (*ctx).max_len);
        if len >= max_len { max_len = len; (*ctx).best_match = ptr; }
        ptr = ptr.add(1);
    }
    if max_len >= 3 { max_len } else { 0 }
}

static S_MAX_LEN: [usize; 9] = [0x1002, 0x802, 0x402, 0x202, 0x102, 0x82, 0x42, 0x22, 0x12];
static S_MAX_OFF: [usize; 9] = [0x10, 0x20, 0x40, 0x80, 0x100, 0x200, 0x400, 0x800, 0x1000];

#[inline]
fn make_pair(offset: usize, len: usize, index: usize) -> u16 {
    (((offset - 1) << (12 - index)) | ((len - 3) & ((1usize << (12 - index)) - 1))) as u16
}

#[inline]
fn parse_pair(pair: u16, offset: &mut usize, index: usize) -> usize {
    *offset = 1 + (pair as usize >> (12 - index));
    3 + (pair as usize & ((1usize << (12 - index)) - 1))
}

// compress_chunk: 0 = compressed, 1 = all zero, -2 = output too small.
unsafe fn compress_chunk(match_fn: unsafe fn(*const u8, *mut lznt) -> usize, unc: *const u8, mut unc_end: *const u8, cmpr: *mut u8, cmpr_end: *mut u8, cmpr_chunk_size: *mut usize, ctx: *mut lznt) -> i32 {
    let mut cnt = 0usize; let mut idx = 0usize; let mut up = unc;
    let mut cp = cmpr.add(3); let mut cp2 = cmpr.add(2); let mut not_zero = 0u8; let mut ohdr = 0u8;
    if unc.add(LZNT_CHUNK_SIZE) < unc_end { unc_end = unc.add(LZNT_CHUNK_SIZE); }
    let last = if cmpr.add(LZNT_CHUNK_SIZE + 2) < cmpr_end { cmpr.add(LZNT_CHUNK_SIZE + 2) } else { cmpr_end };
    (*ctx).unc = unc; (*ctx).unc_end = unc_end; (*ctx).max_len = S_MAX_LEN[0];
    while up < unc_end {
        while unc.add(S_MAX_OFF[idx]) < up { idx += 1; (*ctx).max_len = S_MAX_LEN[idx]; }
        let max_len = if up.add(3) <= unc_end { match_fn(up, ctx) } else { 0 };
        if max_len == 0 { if cp >= last { return -2; } let v = *up; *cp = v; cp = cp.add(1); up = up.add(1); not_zero |= v; }
        else if cp.add(1) >= last { return -2; }
        else { let t16 = make_pair(up.offset_from((*ctx).best_match) as usize, max_len, idx); *cp = t16 as u8; *cp.add(1) = (t16 >> 8) as u8; cp = cp.add(2); ohdr |= 1 << cnt; up = up.add(max_len); }
        cnt = (cnt + 1) & 7;
        if cnt == 0 { *cp2 = ohdr; ohdr = 0; cp2 = cp; cp = cp.add(1); }
    }
    if cp2 < last { *cp2 = ohdr; } else { cp = cp.sub(1); }
    *cmpr_chunk_size = cp.offset_from(cmpr) as usize;
    let t16 = (*cmpr_chunk_size - 3) as u16 | 0xB000; *cmpr = t16 as u8; *cmpr.add(1) = (t16 >> 8) as u8;
    if not_zero != 0 { 0 } else { LZNT_ERROR_ALL_ZEROS }
}

unsafe fn decompress_chunk(mut unc: *mut u8, unc_end: *mut u8, mut cmpr: *const u8, cmpr_end: *const u8) -> isize {
    let start = unc; let mut ch = *cmpr; cmpr = cmpr.add(1); let mut bit = 0usize; let mut index = 0usize;
    while unc < unc_end && cmpr < cmpr_end {
        if unc.offset_from(start) as usize > LZNT_CHUNK_SIZE { return -22; }
        while index < S_MAX_OFF.len() - 1 && start.add(S_MAX_OFF[index]) < unc { index += 1; }
        if ch & (1 << bit) == 0 { *unc = *cmpr; unc = unc.add(1); cmpr = cmpr.add(1); }
        else { if cmpr.add(1) >= cmpr_end { return -22; } let pair = (*cmpr.add(1) as u16) << 8 | *cmpr as u16; cmpr = cmpr.add(2); let mut offset = 0; let mut length = parse_pair(pair, &mut offset, index); if start.add(offset) > unc { return -22; } if unc.add(length) >= unc_end { length = unc_end.offset_from(unc) as usize; } for _ in 0..length { *unc = *unc.sub(offset); unc = unc.add(1); } }
        bit = (bit + 1) & 7; if bit == 0 { if cmpr >= cmpr_end { break; } ch = *cmpr; cmpr = cmpr.add(1); }
    }
    unc.offset_from(start) as isize
}

// Allocation depends on the kernel allocator supplied by the surrounding translation.
extern "C" {
    fn kzalloc(size: usize, flags: u32) -> *mut lznt;
    fn memset(dst: *mut u8, value: i32, size: usize) -> *mut u8;
}

unsafe fn get_lznt_ctx(level: i32) -> *mut lznt {
    let size = if level != 0 { core::mem::offset_of!(lznt, hash) } else { core::mem::size_of::<lznt>() };
    let r = kzalloc(size, 0);
    if !r.is_null() { (*r).std = level == 0; }
    r
}

unsafe fn compress_lznt(unc: *const core::ffi::c_void, unc_size: usize, cmpr: *mut core::ffi::c_void, mut cmpr_size: usize, ctx: *mut lznt) -> usize {
    let mut p = cmpr as *mut u8; let end = p.add(cmpr_size); let mut unc_chunk = unc as *const u8; let unc_end = unc_chunk.add(unc_size); let mut is_zero = true;
    let matcher: unsafe fn(*const u8, *mut lznt) -> usize = if (*ctx).std { memset((*ctx).hash.as_mut_ptr() as *mut u8, 0, core::mem::size_of_val(&(*ctx).hash)); longest_match_std } else { longest_match_best };
    while unc_chunk < unc_end { cmpr_size = 0; let err = compress_chunk(matcher, unc_chunk, unc_end, p, end, &mut cmpr_size, ctx); if err < 0 { return unc_size; } if is_zero && err != LZNT_ERROR_ALL_ZEROS { is_zero = false; } p = p.add(cmpr_size); unc_chunk = unc_chunk.add(LZNT_CHUNK_SIZE); }
    if p <= end.sub(2) { *p = 0; *p.add(1) = 0; }
    if is_zero { 0 } else { p.offset_from(cmpr as *mut u8) as usize }
}

unsafe fn decompress_lznt(cmpr: *const core::ffi::c_void, cmpr_size: usize, unc: *mut core::ffi::c_void, unc_size: usize) -> isize {
    if cmpr_size < 2 { return -22; }
    let mut cp = cmpr as *const u8; let end = cp.add(cmpr_size); let start = unc as *mut u8; let mut up = start; let unc_end = up.add(unc_size);
    let mut hdr = (*cp.add(1) as u16) << 8 | *cp as u16;
    loop { let cmpr_use = 3 + (hdr as usize & (LZNT_CHUNK_SIZE - 1)); if cp.add(cmpr_use) > end { return -22; }
        let unc_use = if hdr & 0x8000 != 0 { let n = decompress_chunk(up, unc_end, cp.add(2), cp.add(cmpr_use)); if n < 0 { return n; } n as usize } else { let n = core::cmp::min(LZNT_CHUNK_SIZE, unc_end.offset_from(up) as usize); if cp.add(2).add(n) > end { return -22; } core::ptr::copy_nonoverlapping(cp.add(2), up, n); n };
        cp = cp.add(cmpr_use); up = up.add(unc_use); if up >= unc_end { break; } if cp > end.sub(2) { break; } hdr = *cp as u16 | ((*cp.add(1) as u16) << 8); if hdr == 0 { break; }
        if unc_use < LZNT_CHUNK_SIZE { let n = LZNT_CHUNK_SIZE - unc_use; let t = up.add(n); if t >= unc_end { break; } memset(up, 0, n); up = t; }
    }
    if cp > end { -22 } else { up.offset_from(start) as isize }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
