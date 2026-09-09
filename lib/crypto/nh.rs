// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2018 Google LLC
 */

/*
 * Implementation of the NH almost-universal hash function, specifically the
 * variant of NH used in Adiantum.  This is *not* a cryptographic hash function.
 *
 * Reference: section 6.3 of "Adiantum: length-preserving encryption for
 * entry-level processors" (https://eprint.iacr.org/2018/720.pdf).
 */

// Dependencies supplied by the surrounding kernel translation:
// crypto/nh.h, linux/export.h, linux/kernel.h, linux/module.h, linux/unaligned.h

#[cfg(config_crypto_lib_nh_arch)]
extern "C" {
    fn nh_arch(key: *const u32, message: *const u8, message_len: usize,
               hash: *mut u64) -> bool;
}

#[cfg(not(config_crypto_lib_nh_arch))]
unsafe fn nh_arch(_key: *const u32, _message: *const u8, _message_len: usize,
                  _hash: *mut u64) -> bool {
    false
}

pub unsafe fn nh(mut key: *const u32, mut message: *const u8,
                 mut message_len: usize, hash: *mut u64) {
    let mut sums: [u64; 4] = [0, 0, 0, 0];

    if nh_arch(key, message, message_len, hash) {
        return;
    }

    // static_assert(NH_PAIR_STRIDE == 2);
    // static_assert(NH_NUM_PASSES == 4);

    while message_len != 0 {
        let m0 = get_unaligned_le32(message.add(0));
        let m1 = get_unaligned_le32(message.add(4));
        let m2 = get_unaligned_le32(message.add(8));
        let m3 = get_unaligned_le32(message.add(12));

        sums[0] = sums[0].wrapping_add((m0.wrapping_add(*key.add(0)) as u64)
            .wrapping_mul(m2.wrapping_add(*key.add(2)) as u64));
        sums[1] = sums[1].wrapping_add((m0.wrapping_add(*key.add(4)) as u64)
            .wrapping_mul(m2.wrapping_add(*key.add(6)) as u64));
        sums[2] = sums[2].wrapping_add((m0.wrapping_add(*key.add(8)) as u64)
            .wrapping_mul(m2.wrapping_add(*key.add(10)) as u64));
        sums[3] = sums[3].wrapping_add((m0.wrapping_add(*key.add(12)) as u64)
            .wrapping_mul(m2.wrapping_add(*key.add(14)) as u64));
        sums[0] = sums[0].wrapping_add((m1.wrapping_add(*key.add(1)) as u64)
            .wrapping_mul(m3.wrapping_add(*key.add(3)) as u64));
        sums[1] = sums[1].wrapping_add((m1.wrapping_add(*key.add(5)) as u64)
            .wrapping_mul(m3.wrapping_add(*key.add(7)) as u64));
        sums[2] = sums[2].wrapping_add((m1.wrapping_add(*key.add(9)) as u64)
            .wrapping_mul(m3.wrapping_add(*key.add(11)) as u64));
        sums[3] = sums[3].wrapping_add((m1.wrapping_add(*key.add(13)) as u64)
            .wrapping_mul(m3.wrapping_add(*key.add(15)) as u64));
        key = key.add(NH_MESSAGE_UNIT / core::mem::size_of::<u32>());
        message = message.add(NH_MESSAGE_UNIT);
        message_len -= NH_MESSAGE_UNIT;
    }

    *hash.add(0) = cpu_to_le64(sums[0]);
    *hash.add(1) = cpu_to_le64(sums[1]);
    *hash.add(2) = cpu_to_le64(sums[2]);
    *hash.add(3) = cpu_to_le64(sums[3]);
}

// EXPORT_SYMBOL_GPL(nh);

// The following declarations are supplied by the surrounding translation.
extern "C" {
    fn get_unaligned_le32(ptr: *const u8) -> u32;
    fn cpu_to_le64(value: u64) -> u64;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
