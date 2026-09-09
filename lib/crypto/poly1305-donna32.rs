// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 *
 * This is based in part on Andrew Moon's poly1305-donna, which is in the
 * public domain.
 */

// Kernel headers provide the types, constants, unaligned helpers, and likely().

pub unsafe fn poly1305_core_setkey(
    key: *mut poly1305_core_key,
    raw_key: *const u8,
) {
    /* r &= 0xffffffc0ffffffc0ffffffc0fffffff */
    (*key).key.r[0] = get_unaligned_le32(raw_key.add(0)) & 0x3ffffff;
    (*key).key.r[1] = (get_unaligned_le32(raw_key.add(3)) >> 2) & 0x3ffff03;
    (*key).key.r[2] = (get_unaligned_le32(raw_key.add(6)) >> 4) & 0x3ffc0ff;
    (*key).key.r[3] = (get_unaligned_le32(raw_key.add(9)) >> 6) & 0x3f03fff;
    (*key).key.r[4] = (get_unaligned_le32(raw_key.add(12)) >> 8) & 0x00fffff;

    /* s = 5*r */
    (*key).precomputed_s.r[0] = (*key).key.r[1].wrapping_mul(5);
    (*key).precomputed_s.r[1] = (*key).key.r[2].wrapping_mul(5);
    (*key).precomputed_s.r[2] = (*key).key.r[3].wrapping_mul(5);
    (*key).precomputed_s.r[3] = (*key).key.r[4].wrapping_mul(5);
}

pub unsafe fn poly1305_core_blocks(
    state: *mut poly1305_state,
    key: *const poly1305_core_key,
    src: *const core::ffi::c_void,
    mut nblocks: u32,
    mut hibit: u32,
) {
    let mut input = src as *const u8;
    let (r0, r1, r2, r3, r4) = ((*key).key.r[0], (*key).key.r[1], (*key).key.r[2], (*key).key.r[3], (*key).key.r[4]);
    let (s1, s2, s3, s4) = ((*key).precomputed_s.r[0], (*key).precomputed_s.r[1], (*key).precomputed_s.r[2], (*key).precomputed_s.r[3]);
    let (mut h0, mut h1, mut h2, mut h3, mut h4) = ((*state).h[0], (*state).h[1], (*state).h[2], (*state).h[3], (*state).h[4]);

    if nblocks == 0 { return; }
    hibit <<= 24;

    loop {
        /* h += m[i] */
        h0 = h0.wrapping_add(get_unaligned_le32(input.add(0)) & 0x3ffffff);
        h1 = h1.wrapping_add((get_unaligned_le32(input.add(3)) >> 2) & 0x3ffffff);
        h2 = h2.wrapping_add((get_unaligned_le32(input.add(6)) >> 4) & 0x3ffffff);
        h3 = h3.wrapping_add((get_unaligned_le32(input.add(9)) >> 6) & 0x3ffffff);
        h4 = h4.wrapping_add((get_unaligned_le32(input.add(12)) >> 8) | hibit);

        /* h *= r */
        let mut d0 = (h0 as u64) * (r0 as u64) + (h1 as u64) * (s4 as u64) + (h2 as u64) * (s3 as u64) + (h3 as u64) * (s2 as u64) + (h4 as u64) * (s1 as u64);
        let mut d1 = (h0 as u64) * (r1 as u64) + (h1 as u64) * (r0 as u64) + (h2 as u64) * (s4 as u64) + (h3 as u64) * (s3 as u64) + (h4 as u64) * (s2 as u64);
        let mut d2 = (h0 as u64) * (r2 as u64) + (h1 as u64) * (r1 as u64) + (h2 as u64) * (r0 as u64) + (h3 as u64) * (s4 as u64) + (h4 as u64) * (s3 as u64);
        let mut d3 = (h0 as u64) * (r3 as u64) + (h1 as u64) * (r2 as u64) + (h2 as u64) * (r1 as u64) + (h3 as u64) * (r0 as u64) + (h4 as u64) * (s4 as u64);
        let mut d4 = (h0 as u64) * (r4 as u64) + (h1 as u64) * (r3 as u64) + (h2 as u64) * (r2 as u64) + (h3 as u64) * (r1 as u64) + (h4 as u64) * (r0 as u64);

        /* (partial) h %= p */
        let mut c = (d0 >> 26) as u32; h0 = d0 as u32 & 0x3ffffff; d1 += c as u64;
        c = (d1 >> 26) as u32; h1 = d1 as u32 & 0x3ffffff; d2 += c as u64;
        c = (d2 >> 26) as u32; h2 = d2 as u32 & 0x3ffffff; d3 += c as u64;
        c = (d3 >> 26) as u32; h3 = d3 as u32 & 0x3ffffff; d4 += c as u64;
        c = (d4 >> 26) as u32; h4 = d4 as u32 & 0x3ffffff;
        h0 = h0.wrapping_add(c.wrapping_mul(5)); c = h0 >> 26; h0 &= 0x3ffffff; h1 = h1.wrapping_add(c);
        input = input.add(POLY1305_BLOCK_SIZE as usize);
        nblocks -= 1;
        if nblocks == 0 { break; }
    }
    (*state).h[0] = h0; (*state).h[1] = h1; (*state).h[2] = h2; (*state).h[3] = h3; (*state).h[4] = h4;
}

pub unsafe fn poly1305_core_emit(state: *const poly1305_state, nonce: *const u32, dst: *mut core::ffi::c_void) {
    let mac = dst as *mut u8;
    let (mut h0, mut h1, mut h2, mut h3, mut h4) = ((*state).h[0], (*state).h[1], (*state).h[2], (*state).h[3], (*state).h[4]);
    let mut c = h1 >> 26; h1 &= 0x3ffffff; h2 = h2.wrapping_add(c);
    c = h2 >> 26; h2 &= 0x3ffffff; h3 = h3.wrapping_add(c);
    c = h3 >> 26; h3 &= 0x3ffffff; h4 = h4.wrapping_add(c);
    c = h4 >> 26; h4 &= 0x3ffffff; h0 = h0.wrapping_add(c.wrapping_mul(5));
    c = h0 >> 26; h0 &= 0x3ffffff; h1 = h1.wrapping_add(c);
    let mut g0 = h0.wrapping_add(5); c = g0 >> 26; g0 &= 0x3ffffff;
    let mut g1 = h1.wrapping_add(c); c = g1 >> 26; g1 &= 0x3ffffff;
    let mut g2 = h2.wrapping_add(c); c = g2 >> 26; g2 &= 0x3ffffff;
    let mut g3 = h3.wrapping_add(c); c = g3 >> 26; g3 &= 0x3ffffff;
    let mut g4 = h4.wrapping_add(c).wrapping_sub(1 << 26);
    let mut mask = (g4 >> 31).wrapping_sub(1);
    g0 &= mask; g1 &= mask; g2 &= mask; g3 &= mask; g4 &= mask; mask = !mask;
    h0 = (h0 & mask) | g0; h1 = (h1 & mask) | g1; h2 = (h2 & mask) | g2; h3 = (h3 & mask) | g3; h4 = (h4 & mask) | g4;
    h0 |= h1 << 26; h1 = (h1 >> 6) | (h2 << 20); h2 = (h2 >> 12) | (h3 << 14); h3 = (h3 >> 18) | (h4 << 8);
    if !nonce.is_null() {
        let mut f = h0 as u64 + *nonce.add(0) as u64; h0 = f as u32;
        f = h1 as u64 + *nonce.add(1) as u64 + (f >> 32); h1 = f as u32;
        f = h2 as u64 + *nonce.add(2) as u64 + (f >> 32); h2 = f as u32;
        f = h3 as u64 + *nonce.add(3) as u64 + (f >> 32); h3 = f as u32;
    }
    put_unaligned_le32(h0, mac.add(0)); put_unaligned_le32(h1, mac.add(4)); put_unaligned_le32(h2, mac.add(8)); put_unaligned_le32(h3, mac.add(12));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
