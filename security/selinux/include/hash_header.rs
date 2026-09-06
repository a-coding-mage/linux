/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * Based on MurmurHash3, written by Austin Appleby and placed in the
 * public domain.
 */
#[inline]
pub fn av_hash(key1: u32, key2: u32, key3: u32, mask: u32) -> u32 {
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;
    const R1: u32 = 15;
    const R2: u32 = 13;
    const M: u32 = 5;
    const N: u32 = 0xe6546b64;

    let mut hash: u32 = 0;

    macro_rules! mix {
        ($input:expr) => {{
            let mut v: u32 = $input;
            v = v.wrapping_mul(C1);
            v = v.rotate_left(R1);
            v = v.wrapping_mul(C2);
            hash ^= v;
            hash = hash.rotate_left(R2);
            hash = hash.wrapping_mul(M).wrapping_add(N);
        }};
    }

    mix!(key1);
    mix!(key2);
    mix!(key3);

    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85ebca6b);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash ^= hash >> 16;

    hash & mask
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
