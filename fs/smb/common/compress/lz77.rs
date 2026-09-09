// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024-2026, SUSE LLC
 * Copyright (C) 2026 Namjae Jeon <linkinjeon@kernel.org>
 *
 * Authors: Enzo Matsumiya <ematsumiya@suse.de>
 *          Namjae Jeon <linkinjeon@kernel.org>
 *
 * Implementation of the LZ77 "plain" compression algorithm, as per MS-XCA spec.
 */

// Linux kernel dependencies supplied by the surrounding repository.
use core::ptr;

const LZ77_MATCH_MAX_DIST: usize = 8 * 1024;
const LZ77_HASH_LOG: u32 = 15;
const LZ77_HASH_SIZE: usize = 1usize << LZ77_HASH_LOG;
const LZ77_RSTEP_SIZE: usize = core::mem::size_of::<u32>();
const LZ77_MSTEP_SIZE: usize = core::mem::size_of::<u64>();
const LZ77_SKIP_TRIGGER: u32 = 4;
const LZ77_FLAG_MAX: u32 = 32;

#[inline(always)]
unsafe fn lz77_read8(ptr: *const u8) -> u8 { ptr::read_unaligned(ptr) }

#[inline(always)]
unsafe fn lz77_read32(ptr: *const u32) -> u32 { ptr::read_unaligned(ptr) }

#[inline(always)]
unsafe fn lz77_read64(ptr: *const u64) -> u64 { ptr::read_unaligned(ptr) }

#[inline(always)]
unsafe fn lz77_write8(ptr: *mut u8, v: u8) { ptr::write_unaligned(ptr, v); }

#[inline(always)]
unsafe fn lz77_write16(ptr: *mut u16, v: u16) { ptr::copy_nonoverlapping(v.to_le_bytes().as_ptr(), ptr as *mut u8, 2); }

#[inline(always)]
unsafe fn lz77_write32(ptr: *mut u32, v: u32) { ptr::copy_nonoverlapping(v.to_le_bytes().as_ptr(), ptr as *mut u8, 4); }

#[inline(always)]
unsafe fn lz77_match_len(mut mat: *const u8, mut cur: *const u8, end: *const u8) -> u32 {
    let start = cur;
    while cur.add(LZ77_MSTEP_SIZE) <= end {
        let diff = lz77_read64(cur as *const u64) ^ lz77_read64(mat as *const u64);
        if diff == 0 { cur = cur.add(LZ77_MSTEP_SIZE); mat = mat.add(LZ77_MSTEP_SIZE); continue; }
        cur = cur.add((diff.trailing_zeros() >> 3) as usize);
        return cur.offset_from(start) as u32;
    }
    while cur < end && lz77_read8(cur) == lz77_read8(mat) { cur = cur.add(1); mat = mat.add(1); }
    cur.offset_from(start) as u32
}

#[inline(always)]
unsafe fn lz77_encode_match(mut dst: *mut u8, nib: &mut *mut u8, mut dist: u16, mut len: u32) -> *mut u8 {
    len -= 3; dist = (dist - 1) << 3;
    if len < 7 { lz77_write16(dst as *mut u16, dist + len as u16); return dst.add(2); }
    dist |= 7; lz77_write16(dst as *mut u16, dist); dst = dst.add(2); len -= 7;
    if (*nib).is_null() { lz77_write8(dst, core::cmp::min(len, 15) as u8); *nib = dst; dst = dst.add(1); }
    else { let b = *nib; lz77_write8(b, *b | (core::cmp::min(len, 15) as u8) << 4); *nib = ptr::null_mut(); }
    if len < 15 { return dst; }
    len -= 15;
    if len < 255 { lz77_write8(dst, len as u8); return dst.add(1); }
    lz77_write8(dst, 0xff); dst = dst.add(1); len += 22;
    if len <= 0xffff { lz77_write16(dst as *mut u16, len as u16); return dst.add(2); }
    lz77_write16(dst as *mut u16, 0); dst = dst.add(2); lz77_write32(dst as *mut u32, len); dst.add(4)
}

#[inline(always)]
unsafe fn lz77_encode_literals(mut start: *const u8, end: *const u8, mut dst: *mut u8, f: &mut u32, fc: &mut u32, fp: &mut *mut u8) -> *mut u8 {
    while start < end { let len = core::cmp::min(end.offset_from(start) as u32, LZ77_FLAG_MAX - *fc); ptr::copy_nonoverlapping(start, dst, len as usize); dst = dst.add(len as usize); start = start.add(len as usize); *f <<= len; *fc += len; if *fc == LZ77_FLAG_MAX { lz77_write32(*fp as *mut u32, *f); *fc = 0; *fp = dst; dst = dst.add(4); } }
    dst
}

#[inline(always)]
fn lz77_hash(v: u32) -> u32 { ((v ^ 0x9E3779B9).wrapping_mul(0x85EBCA6B)) >> (32 - LZ77_HASH_LOG) }

// External allocator and error symbols are supplied by the kernel integration.
extern "C" { fn smb_lz77_compressed_alloc_size(slen: u32) -> u32; }

pub unsafe fn smb_lz77_compress(src: *const u8, slen: u32, dst: *mut u8, dlen: *mut u32) -> i32 {
    if *dlen < smb_lz77_compressed_alloc_size(slen) { return -22; }
    let mut srcp = src; let anchor = src; let end = src.add(slen as usize); let rlim = end.sub(LZ77_MSTEP_SIZE);
    let mut dstp = dst.add(4); let mut flag_pos = dst; let mut nib = ptr::null_mut(); let mut anchor = src;
    let mut htable = vec![0u32; LZ77_HASH_SIZE]; let mut flag_count = 0u32; let mut flag = 0u32;
    let mut hash = lz77_hash(lz77_read32(srcp as *const u32));
    htable[hash as usize] = 0; hash = lz77_hash(lz77_read32(srcp.add(1) as *const u32));
    let mut srcp = srcp.add(1);
    loop {
        let mut next = srcp; let mut step = 1usize; let mut skip = 1u32;
        loop { let cur_hash = hash; srcp = next; next = next.add(step); step = skip >> LZ77_SKIP_TRIGGER; skip += 1; if next > rlim { break; } hash = lz77_hash(lz77_read32(next as *const u32)); let m = src.add(htable[cur_hash as usize] as usize); htable[cur_hash as usize] = srcp.offset_from(src) as u32; if !(m.add(LZ77_MATCH_MAX_DIST) < srcp || lz77_read32(m as *const u32) != lz77_read32(srcp as *const u32)) { let l = lz77_match_len(m, srcp, end); dstp = lz77_encode_literals(anchor, srcp, dstp, &mut flag, &mut flag_count, &mut flag_pos); dstp = lz77_encode_match(dstp, &mut nib, srcp.offset_from(m) as u16, l); srcp = srcp.add(l as usize); anchor = srcp; flag = (flag << 1) | 1; flag_count += 1; if flag_count == LZ77_FLAG_MAX { lz77_write32(flag_pos as *mut u32, flag); flag_count = 0; flag_pos = dstp; dstp = dstp.add(4); } if srcp > rlim { break; } hash = lz77_hash(lz77_read32(srcp as *const u32)); break; } }
        if next > rlim || srcp >= end { break; }
    }
    dstp = lz77_encode_literals(anchor, end, dstp, &mut flag, &mut flag_count, &mut flag_pos); let count = LZ77_FLAG_MAX - flag_count; flag = (flag << count) | ((1u32 << count) - 1); lz77_write32(flag_pos as *mut u32, flag); *dlen = dstp.offset_from(dst) as u32; if *dlen < slen { 0 } else { -90 }
}

unsafe fn lz77_decode_match_len(src: &mut *const u8, end: *const u8, token: u16, nibble: &mut u8, have: &mut bool, len: &mut u32) -> i32 {
    *len = (token & 7) as u32 + 3; if token & 7 != 7 { return 0; }
    let extra = if !*have { if *src >= end { return -22; } *nibble = **src; *src = (*src).add(1); *have = true; *nibble & 15 } else { *have = false; *nibble >> 4 };
    *len += extra as u32; if extra == 15 { if *src >= end { return -22; } let b = **src; *src = (*src).add(1); if b != 0xff { *len += b as u32; } else { if end.offset_from(*src) < 2 { return -22; } let w = u16::from_le_bytes([*(*src), *(*src).add(1)]); *src = (*src).add(2); if w != 0 { *len = w as u32 + 3; } else { if end.offset_from(*src) < 4 { return -22; } let p = *src; let v = u32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)]); *src = p.add(4); *len = match v.checked_add(3) { Some(x) => x, None => return -22 }; } } } 0
}

pub unsafe fn smb_lz77_decompress(src: *const u8, slen: u32, dst: *mut u8, dlen: u32) -> i32 {
    let mut sp = src; let send = src.add(slen as usize); let mut dp = dst; let dend = dst.add(dlen as usize); let mut flags = 0u32; let mut fc = 0; let mut nib = 0; let mut have = false;
    while dp < dend { if fc == 0 { if send.offset_from(sp) < 4 { return -22; } flags = u32::from_le_bytes([*sp, *sp.add(1), *sp.add(2), *sp.add(3)]); sp = sp.add(4); fc = 32; } if flags & 0x80000000 == 0 { if sp >= send { return -22; } *dp = *sp; dp = dp.add(1); sp = sp.add(1); flags <<= 1; fc -= 1; continue; } flags <<= 1; fc -= 1; if send.offset_from(sp) < 2 { return -22; } let token = u16::from_le_bytes([*sp, *sp.add(1)]); sp = sp.add(2); let dist = (token >> 3) as usize + 1; if dist > dp.offset_from(dst) as usize { return -22; } let mut len = 0; if lz77_decode_match_len(&mut sp, send, token, &mut nib, &mut have, &mut len) != 0 || len as usize > dend.offset_from(dp) as usize { return -22; } for _ in 0..len { *dp = *dp.sub(dist); dp = dp.add(1); } } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
