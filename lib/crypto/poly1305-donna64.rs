// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 *
 * This is based in part on Andrew Moon's poly1305-donna, which is in the
 * public domain.
 */

// Types and constants are supplied by the corresponding crypto headers.

#[inline]
unsafe fn get_unaligned_le64(p: *const u8) -> u64 {
    u64::from_le(core::ptr::read_unaligned(p as *const u64))
}

#[inline]
unsafe fn put_unaligned_le64(v: u64, p: *mut u8) {
    core::ptr::write_unaligned(p as *mut u64, v.to_le());
}

pub unsafe fn poly1305_core_setkey(
    key: *mut poly1305_core_key,
    raw_key: *const u8,
) {
    let t0 = get_unaligned_le64(raw_key.add(0));
    let t1 = get_unaligned_le64(raw_key.add(8));

    // r &= 0xffffffc0ffffffc0ffffffc0fffffff
    (*key).key.r64[0] = t0 & 0xffc0fffffff_u64;
    (*key).key.r64[1] = ((t0 >> 44) | (t1 << 20)) & 0xfffffc0ffff_u64;
    (*key).key.r64[2] = (t1 >> 24) & 0x00ffffffc0f_u64;

    // s = 20*r
    (*key).precomputed_s.r64[0] = (*key).key.r64[1].wrapping_mul(20);
    (*key).precomputed_s.r64[1] = (*key).key.r64[2].wrapping_mul(20);
}

pub unsafe fn poly1305_core_blocks(
    state: *mut poly1305_state,
    key: *const poly1305_core_key,
    src: *const core::ffi::c_void,
    mut nblocks: u32,
    hibit: u32,
) {
    if nblocks == 0 { return; }
    let mut input = src as *const u8;
    let hibit64 = (hibit as u64) << 40;
    let r0 = (*key).key.r64[0];
    let r1 = (*key).key.r64[1];
    let r2 = (*key).key.r64[2];
    let mut h0 = (*state).h64[0];
    let mut h1 = (*state).h64[1];
    let mut h2 = (*state).h64[2];
    let s1 = (*key).precomputed_s.r64[0];
    let s2 = (*key).precomputed_s.r64[1];

    while nblocks != 0 {
        let t0 = get_unaligned_le64(input.add(0));
        let t1 = get_unaligned_le64(input.add(8));
        h0 = h0.wrapping_add(t0 & 0xfffffffffff_u64);
        h1 = h1.wrapping_add(((t0 >> 44) | (t1 << 20)) & 0xfffffffffff_u64);
        h2 = h2.wrapping_add(((t1 >> 24) & 0x3ffffffffff_u64) | hibit64);

        let mut d0 = (h0 as u128) * (r0 as u128);
        d0 += (h1 as u128) * (s2 as u128);
        d0 += (h2 as u128) * (s1 as u128);
        let mut d1 = (h0 as u128) * (r1 as u128);
        d1 += (h1 as u128) * (r0 as u128);
        d1 += (h2 as u128) * (s2 as u128);
        let mut d2 = (h0 as u128) * (r2 as u128);
        d2 += (h1 as u128) * (r1 as u128);
        d2 += (h2 as u128) * (r0 as u128);

        let mut c = (d0 >> 44) as u64;
        h0 = d0 as u64 & 0xfffffffffff_u64;
        d1 += c as u128;
        c = (d1 >> 44) as u64;
        h1 = d1 as u64 & 0xfffffffffff_u64;
        d2 += c as u128;
        c = (d2 >> 42) as u64;
        h2 = d2 as u64 & 0x3ffffffffff_u64;
        h0 = h0.wrapping_add(c.wrapping_mul(5));
        c = h0 >> 44;
        h0 &= 0xfffffffffff_u64;
        h1 = h1.wrapping_add(c);
        input = input.add(POLY1305_BLOCK_SIZE as usize);
        nblocks -= 1;
    }
    (*state).h64[0] = h0;
    (*state).h64[1] = h1;
    (*state).h64[2] = h2;
}

pub unsafe fn poly1305_core_emit(
    state: *const poly1305_state,
    nonce: *const u32,
    dst: *mut core::ffi::c_void,
) {
    let mac = dst as *mut u8;
    let mut h0 = (*state).h64[0];
    let mut h1 = (*state).h64[1];
    let mut h2 = (*state).h64[2];
    let mut c = h1 >> 44;
    h1 &= 0xfffffffffff_u64; h2 = h2.wrapping_add(c);
    c = h2 >> 42; h2 &= 0x3ffffffffff_u64; h0 = h0.wrapping_add(c * 5);
    c = h0 >> 44; h0 &= 0xfffffffffff_u64; h1 = h1.wrapping_add(c);
    c = h1 >> 44; h1 &= 0xfffffffffff_u64; h2 = h2.wrapping_add(c);
    c = h2 >> 42; h2 &= 0x3ffffffffff_u64; h0 = h0.wrapping_add(c * 5);
    c = h0 >> 44; h0 &= 0xfffffffffff_u64; h1 = h1.wrapping_add(c);
    let mut g0 = h0.wrapping_add(5); c = g0 >> 44; g0 &= 0xfffffffffff_u64;
    let mut g1 = h1.wrapping_add(c); c = g1 >> 44; g1 &= 0xfffffffffff_u64;
    let g2 = h2.wrapping_add(c).wrapping_sub(1_u64 << 42);
    c = (g2 >> 63).wrapping_sub(1); g0 &= c; g1 &= c;
    let g2 = g2 & c; c = !c; h0 = (h0 & c) | g0; h1 = (h1 & c) | g1; h2 = (h2 & c) | g2;
    if !nonce.is_null() {
        let t0 = ((*nonce.add(1) as u64) << 32) | *nonce;
        let t1 = ((*nonce.add(3) as u64) << 32) | *nonce.add(2);
        h0 = h0.wrapping_add(t0 & 0xfffffffffff_u64); c = h0 >> 44; h0 &= 0xfffffffffff_u64;
        h1 = h1.wrapping_add(((t0 >> 44) | (t1 << 20)) & 0xfffffffffff_u64).wrapping_add(c);
        c = h1 >> 44; h1 &= 0xfffffffffff_u64; h2 = h2.wrapping_add(((t1 >> 24) & 0x3ffffffffff_u64) + c) & 0x3ffffffffff_u64;
    }
    put_unaligned_le64(h0 | (h1 << 44), mac.add(0));
    put_unaligned_le64((h1 >> 20) | (h2 << 24), mac.add(8));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
