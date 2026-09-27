// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
/* Copyright (C) 2016-2022 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 *
 * SipHash: a fast short-input PRF
 * https://131002.net/siphash/
 *
 * This implementation is specifically for SipHash2-4 for a secure PRF
 * and HalfSipHash1-3/SipHash1-3 for an insecure PRF only suitable for
 * hashtables.
 */

// Dependencies: linux/siphash.h, linux/unaligned.h; with
// CONFIG_DCACHE_WORD_ACCESS on 64-bit also linux/dcache.h and
// asm/word-at-a-time.h.
//
// The C PREAMBLE/SIPROUND/POSTAMBLE macros are expressed as a small state
// type; every function performs the same operations in the same order.

use core::ffi::c_void;

/// SipHash state: `PREAMBLE(len)` initializes it, `SIPROUND` permutes it.
struct Sip {
    v0: u64,
    v1: u64,
    v2: u64,
    v3: u64,
    b: u64,
}

impl Sip {
    /// `PREAMBLE(len)`
    #[inline(always)]
    fn new(len: u64, key: &[u64; 2]) -> Self {
        Sip {
            v0: SIPHASH_CONST_0 ^ key[0],
            v1: SIPHASH_CONST_1 ^ key[1],
            v2: SIPHASH_CONST_2 ^ key[0],
            v3: SIPHASH_CONST_3 ^ key[1],
            b: len << 56,
        }
    }

    /// `SIPROUND`: SIPHASH_PERMUTATION(v0, v1, v2, v3)
    #[inline(always)]
    fn round(&mut self) {
        self.v0 = self.v0.wrapping_add(self.v1);
        self.v1 = self.v1.rotate_left(13);
        self.v1 ^= self.v0;
        self.v0 = self.v0.rotate_left(32);
        self.v2 = self.v2.wrapping_add(self.v3);
        self.v3 = self.v3.rotate_left(16);
        self.v3 ^= self.v2;
        self.v0 = self.v0.wrapping_add(self.v3);
        self.v3 = self.v3.rotate_left(21);
        self.v3 ^= self.v0;
        self.v2 = self.v2.wrapping_add(self.v1);
        self.v1 = self.v1.rotate_left(17);
        self.v1 ^= self.v2;
        self.v2 = self.v2.rotate_left(32);
    }

    /// One message word with `rounds` SIPROUNDs (`v3 ^= m; ...; v0 ^= m`).
    #[inline(always)]
    fn compress(&mut self, m: u64, rounds: u32) {
        self.v3 ^= m;
        for _ in 0..rounds {
            self.round();
        }
        self.v0 ^= m;
    }

    /// `POSTAMBLE` (SipHash-2-4) or `HPOSTAMBLE` on 64-bit (SipHash-1-3).
    #[inline(always)]
    fn finish(mut self, c_rounds: u32, d_rounds: u32) -> u64 {
        let b = self.b;
        self.v3 ^= b;
        for _ in 0..c_rounds {
            self.round();
        }
        self.v0 ^= b;
        self.v2 ^= 0xff;
        for _ in 0..d_rounds {
            self.round();
        }
        (self.v0 ^ self.v1) ^ (self.v2 ^ self.v3)
    }
}

/// The trailing `len % 8` bytes, little-endian (the C `switch (left)` or,
/// with CONFIG_DCACHE_WORD_ACCESS on 64-bit, `load_unaligned_zeropad()`).
#[inline(always)]
unsafe fn sip_tail(end: *const u8, left: usize) -> u64 {
    #[cfg(all(CONFIG_DCACHE_WORD_ACCESS, CONFIG_64BIT))]
    {
        if left != 0 {
            return u64::from_le(load_unaligned_zeropad(end.cast()) & bytemask_from_count(left as _));
        }
        0
    }
    #[cfg(not(all(CONFIG_DCACHE_WORD_ACCESS, CONFIG_64BIT)))]
    {
        let mut b: u64 = 0;
        for i in 0..left {
            b |= (*end.add(i) as u64) << (8 * i);
        }
        b
    }
}

/// Shared body of `__siphash_aligned()` / `__siphash_unaligned()` and their
/// 64-bit HalfSipHash counterparts; `aligned` selects le64_to_cpup() over
/// get_unaligned_le64() for the message words.
#[inline(always)]
unsafe fn sip_bytes(data: *const c_void, len: usize, key: &[u64; 2], c_rounds: u32, d_rounds: u32, aligned: bool) -> u64 {
    let mut data = data.cast::<u8>();
    let end = data.add(len - (len % 8));
    let left = len & 7;
    let mut s = Sip::new(len as u64, key);
    while data != end {
        let m = if aligned {
            u64::from_le(data.cast::<u64>().read())
        } else {
            u64::from_le_bytes(data.cast::<[u8; 8]>().read_unaligned())
        };
        s.compress(m, c_rounds);
        data = data.add(8);
    }
    s.b |= sip_tail(end, left);
    s.finish(c_rounds, d_rounds)
}

#[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
#[no_mangle]
pub unsafe extern "C" fn __siphash_aligned(data: *const c_void, len: usize, key: *const siphash_key_t) -> u64 {
    sip_bytes(data, len, &(*key).key, 2, 4, true)
}
// EXPORT_SYMBOL(__siphash_aligned);

#[no_mangle]
pub unsafe extern "C" fn __siphash_unaligned(data: *const c_void, len: usize, key: *const siphash_key_t) -> u64 {
    sip_bytes(data, len, &(*key).key, 2, 4, false)
}
// EXPORT_SYMBOL(__siphash_unaligned);

/**
 * siphash_1u64 - compute 64-bit siphash PRF value of a u64
 * @first: first u64
 * @key: the siphash key
 */
#[no_mangle]
pub unsafe extern "C" fn siphash_1u64(first: u64, key: *const siphash_key_t) -> u64 {
    let mut s = Sip::new(8, &(*key).key);
    s.compress(first, 2);
    s.finish(2, 4)
}
// EXPORT_SYMBOL(siphash_1u64);

/**
 * siphash_2u64 - compute 64-bit siphash PRF value of 2 u64
 * @first: first u64
 * @second: second u64
 * @key: the siphash key
 */
#[no_mangle]
pub unsafe extern "C" fn siphash_2u64(first: u64, second: u64, key: *const siphash_key_t) -> u64 {
    let mut s = Sip::new(16, &(*key).key);
    s.compress(first, 2);
    s.compress(second, 2);
    s.finish(2, 4)
}
// EXPORT_SYMBOL(siphash_2u64);

/**
 * siphash_3u64 - compute 64-bit siphash PRF value of 3 u64
 * @first: first u64
 * @second: second u64
 * @third: third u64
 * @key: the siphash key
 */
#[no_mangle]
pub unsafe extern "C" fn siphash_3u64(first: u64, second: u64, third: u64, key: *const siphash_key_t) -> u64 {
    let mut s = Sip::new(24, &(*key).key);
    s.compress(first, 2);
    s.compress(second, 2);
    s.compress(third, 2);
    s.finish(2, 4)
}
// EXPORT_SYMBOL(siphash_3u64);

/**
 * siphash_4u64 - compute 64-bit siphash PRF value of 4 u64
 * @first: first u64
 * @second: second u64
 * @third: third u64
 * @forth: forth u64
 * @key: the siphash key
 */
#[no_mangle]
pub unsafe extern "C" fn siphash_4u64(first: u64, second: u64, third: u64, forth: u64, key: *const siphash_key_t) -> u64 {
    let mut s = Sip::new(32, &(*key).key);
    s.compress(first, 2);
    s.compress(second, 2);
    s.compress(third, 2);
    s.compress(forth, 2);
    s.finish(2, 4)
}
// EXPORT_SYMBOL(siphash_4u64);

#[no_mangle]
pub unsafe extern "C" fn siphash_1u32(first: u32, key: *const siphash_key_t) -> u64 {
    let mut s = Sip::new(4, &(*key).key);
    s.b |= first as u64;
    s.finish(2, 4)
}
// EXPORT_SYMBOL(siphash_1u32);

#[no_mangle]
pub unsafe extern "C" fn siphash_3u32(first: u32, second: u32, third: u32, key: *const siphash_key_t) -> u64 {
    let combined: u64 = (second as u64) << 32 | first as u64;
    let mut s = Sip::new(12, &(*key).key);
    s.compress(combined, 2);
    s.b |= third as u64;
    s.finish(2, 4)
}
// EXPORT_SYMBOL(siphash_3u32);

/* Note that on 64-bit, we make HalfSipHash1-3 actually be SipHash1-3, for
 * performance reasons. On 32-bit, below, we actually implement HalfSipHash1-3.
 */
#[cfg(CONFIG_64BIT)]
mod hsip {
    use super::*;

    #[inline(always)]
    fn key64(key: *const hsiphash_key_t) -> [u64; 2] {
        // SAFETY: callers pass a valid key; unsigned long is 64-bit here.
        unsafe { [(*key).key[0] as u64, (*key).key[1] as u64] }
    }

    #[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
    #[no_mangle]
    pub unsafe extern "C" fn __hsiphash_aligned(data: *const c_void, len: usize, key: *const hsiphash_key_t) -> u32 {
        sip_bytes(data, len, &key64(key), 1, 3, true) as u32
    }
    // EXPORT_SYMBOL(__hsiphash_aligned);

    #[no_mangle]
    pub unsafe extern "C" fn __hsiphash_unaligned(data: *const c_void, len: usize, key: *const hsiphash_key_t) -> u32 {
        sip_bytes(data, len, &key64(key), 1, 3, false) as u32
    }
    // EXPORT_SYMBOL(__hsiphash_unaligned);

    #[no_mangle]
    pub unsafe extern "C" fn hsiphash_1u32(first: u32, key: *const hsiphash_key_t) -> u32 {
        let mut s = Sip::new(4, &key64(key));
        s.b |= first as u64;
        s.finish(1, 3) as u32
    }
    // EXPORT_SYMBOL(hsiphash_1u32);

    #[no_mangle]
    pub unsafe extern "C" fn hsiphash_2u32(first: u32, second: u32, key: *const hsiphash_key_t) -> u32 {
        let combined: u64 = (second as u64) << 32 | first as u64;
        let mut s = Sip::new(8, &key64(key));
        s.compress(combined, 1);
        s.finish(1, 3) as u32
    }
    // EXPORT_SYMBOL(hsiphash_2u32);

    #[no_mangle]
    pub unsafe extern "C" fn hsiphash_3u32(first: u32, second: u32, third: u32, key: *const hsiphash_key_t) -> u32 {
        let combined: u64 = (second as u64) << 32 | first as u64;
        let mut s = Sip::new(12, &key64(key));
        s.compress(combined, 1);
        s.b |= third as u64;
        s.finish(1, 3) as u32
    }
    // EXPORT_SYMBOL(hsiphash_3u32);

    #[no_mangle]
    pub unsafe extern "C" fn hsiphash_4u32(first: u32, second: u32, third: u32, forth: u32, key: *const hsiphash_key_t) -> u32 {
        let mut combined: u64 = (second as u64) << 32 | first as u64;
        let mut s = Sip::new(16, &key64(key));
        s.compress(combined, 1);
        combined = (forth as u64) << 32 | third as u64;
        s.compress(combined, 1);
        s.finish(1, 3) as u32
    }
    // EXPORT_SYMBOL(hsiphash_4u32);
}

/// HalfSipHash-1-3 state (32-bit only): `HPREAMBLE(len)` / `HSIPROUND`.
#[cfg(not(CONFIG_64BIT))]
mod hsip {
    use super::*;

    struct HSip {
        v0: u32,
        v1: u32,
        v2: u32,
        v3: u32,
        b: u32,
    }

    impl HSip {
        #[inline(always)]
        fn new(len: u32, key: *const hsiphash_key_t) -> Self {
            // SAFETY: callers pass a valid key.
            let (k0, k1) = unsafe { ((*key).key[0] as u32, (*key).key[1] as u32) };
            HSip {
                v0: HSIPHASH_CONST_0 ^ k0,
                v1: HSIPHASH_CONST_1 ^ k1,
                v2: HSIPHASH_CONST_2 ^ k0,
                v3: HSIPHASH_CONST_3 ^ k1,
                b: len << 24,
            }
        }

        /// HSIPHASH_PERMUTATION(v0, v1, v2, v3)
        #[inline(always)]
        fn round(&mut self) {
            self.v0 = self.v0.wrapping_add(self.v1);
            self.v1 = self.v1.rotate_left(5);
            self.v1 ^= self.v0;
            self.v0 = self.v0.rotate_left(16);
            self.v2 = self.v2.wrapping_add(self.v3);
            self.v3 = self.v3.rotate_left(8);
            self.v3 ^= self.v2;
            self.v0 = self.v0.wrapping_add(self.v3);
            self.v3 = self.v3.rotate_left(7);
            self.v3 ^= self.v0;
            self.v2 = self.v2.wrapping_add(self.v1);
            self.v1 = self.v1.rotate_left(13);
            self.v1 ^= self.v2;
            self.v2 = self.v2.rotate_left(16);
        }

        #[inline(always)]
        fn compress(&mut self, m: u32) {
            self.v3 ^= m;
            self.round();
            self.v0 ^= m;
        }

        /// `HPOSTAMBLE`
        #[inline(always)]
        fn finish(mut self) -> u32 {
            let b = self.b;
            self.v3 ^= b;
            self.round();
            self.v0 ^= b;
            self.v2 ^= 0xff;
            self.round();
            self.round();
            self.round();
            self.v1 ^ self.v3
        }
    }

    #[inline(always)]
    unsafe fn hsip_bytes(data: *const c_void, len: usize, key: *const hsiphash_key_t, aligned: bool) -> u32 {
        let mut data = data.cast::<u8>();
        let end = data.add(len - (len % 4));
        let left = len & 3;
        let mut s = HSip::new(len as u32, key);
        while data != end {
            let m = if aligned {
                u32::from_le(data.cast::<u32>().read())
            } else {
                u32::from_le_bytes(data.cast::<[u8; 4]>().read_unaligned())
            };
            s.compress(m);
            data = data.add(4);
        }
        for i in 0..left {
            s.b |= (*end.add(i) as u32) << (8 * i);
        }
        s.finish()
    }

    #[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
    #[no_mangle]
    pub unsafe extern "C" fn __hsiphash_aligned(data: *const c_void, len: usize, key: *const hsiphash_key_t) -> u32 {
        hsip_bytes(data, len, key, true)
    }
    // EXPORT_SYMBOL(__hsiphash_aligned);

    #[no_mangle]
    pub unsafe extern "C" fn __hsiphash_unaligned(data: *const c_void, len: usize, key: *const hsiphash_key_t) -> u32 {
        hsip_bytes(data, len, key, false)
    }
    // EXPORT_SYMBOL(__hsiphash_unaligned);

    #[no_mangle]
    pub unsafe extern "C" fn hsiphash_1u32(first: u32, key: *const hsiphash_key_t) -> u32 {
        let mut s = HSip::new(4, key);
        s.compress(first);
        s.finish()
    }
    // EXPORT_SYMBOL(hsiphash_1u32);

    #[no_mangle]
    pub unsafe extern "C" fn hsiphash_2u32(first: u32, second: u32, key: *const hsiphash_key_t) -> u32 {
        let mut s = HSip::new(8, key);
        s.compress(first);
        s.compress(second);
        s.finish()
    }
    // EXPORT_SYMBOL(hsiphash_2u32);

    #[no_mangle]
    pub unsafe extern "C" fn hsiphash_3u32(first: u32, second: u32, third: u32, key: *const hsiphash_key_t) -> u32 {
        let mut s = HSip::new(12, key);
        s.compress(first);
        s.compress(second);
        s.compress(third);
        s.finish()
    }
    // EXPORT_SYMBOL(hsiphash_3u32);

    #[no_mangle]
    pub unsafe extern "C" fn hsiphash_4u32(first: u32, second: u32, third: u32, forth: u32, key: *const hsiphash_key_t) -> u32 {
        let mut s = HSip::new(16, key);
        s.compress(first);
        s.compress(second);
        s.compress(third);
        s.compress(forth);
        s.finish()
    }
    // EXPORT_SYMBOL(hsiphash_4u32);
}
pub use hsip::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
