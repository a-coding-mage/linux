// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
/* Copyright (C) 2016-2022 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 *
 * Test cases for siphash.c
 *
 * SipHash: a fast short-input PRF
 * https://131002.net/siphash/
 *
 * This implementation is specifically for SipHash2-4 for a secure PRF
 * and HalfSipHash1-3/SipHash1-3 for an insecure PRF only suitable for
 * hashtables.
 */

// Dependencies supplied by the surrounding kernel/KUnit environment.

/* Test vectors taken from reference source available at:
 *     https://github.com/veorq/SipHash
 */

static TEST_KEY_SIPHASH: siphash_key_t = siphash_key_t {
    key: [0x0706050403020100_u64, 0x0f0e0d0c0b0a0908_u64],
};

static TEST_VECTORS_SIPHASH: [u64; 64] = [
    0x726fdb47dd0e0e31, 0x74f839c593dc67fd, 0x0d6c8009d9a94f5a,
    0x85676696d7fb7e2d, 0xcf2794e0277187b7, 0x18765564cd99a68d,
    0xcbc9466e58fee3ce, 0xab0200f58b01d137, 0x93f5f5799a932462,
    0x9e0082df0ba9e4b0, 0x7a5dbbc594ddb9f3, 0xf4b32f46226bada7,
    0x751e8fbc860ee5fb, 0x14ea5627c0843d90, 0xf723ca908e7af2ee,
    0xa129ca6149be45e5, 0x3f2acc7f57c29bdb, 0x699ae9f52cbe4794,
    0x4bc1b3f0968dd39c, 0xbb6dc91da77961bd, 0xbed65cf21aa2ee98,
    0xd0f2cbb02e3b67c7, 0x93536795e3a33e88, 0xa80c038ccd5ccec8,
    0xb8ad50c6f649af94, 0xbce192de8a85b8ea, 0x17d835b85bbb15f3,
    0x2f2e6163076bcfad, 0xde4daaaca71dc9a5, 0xa6a2506687956571,
    0xad87a3535c49ef28, 0x32d892fad841c342, 0x7127512f72f27cce,
    0xa7f32346f95978e3, 0x12e0b01abb051238, 0x15e034d40fa197ae,
    0x314dffbe0815a3b4, 0x027990f029623981, 0xcadcd4e59ef40c4d,
    0x9abfd8766a33735c, 0x0e3ea96b5304a7d0, 0xad0c42d6fc585992,
    0x187306c89bc215a9, 0xd4a60abcf3792b95, 0xf935451de4f21df2,
    0xa9538f0419755787, 0xdb9acddff56ca510, 0xd06c98cd5c0975eb,
    0xe612a3cb9ecba951, 0xc766e62cfcadaf96, 0xee64435a9752fe72,
    0xa192d576b245165a, 0x0a8787bf8ecb74b2, 0x81b3e73d20b49b6f,
    0x7fa8220ba3b2ecea, 0x245731c13ca42499, 0xb78dbfaf3a8d83bd,
    0xea1ad565322a1a0b, 0x60e61c23a3795013, 0x6606d7e446282b93,
    0x6ca4ecb15c5f91e1, 0x9f626da15c9625f3, 0xe51b38608ef25f57,
    0x958a324ceb064572,
];

// The C source selects these vectors using BITS_PER_LONG == 64.
#[cfg(target_pointer_width = "64")]
static TEST_KEY_HSIPHASH: hsiphash_key_t = hsiphash_key_t {
    key: [0x0706050403020100_u64, 0x0f0e0d0c0b0a0908_u64],
};

#[cfg(target_pointer_width = "64")]
static TEST_VECTORS_HSIPHASH: [u32; 64] = [
    0x050fc4dc, 0x7d57ca93, 0x4dc7d44d, 0xe7ddf7fb, 0x88d38328, 0x49533b67,
    0xc59f22a7, 0x9bb11140, 0x8d299a8e, 0x6c063de4, 0x92ff097f, 0xf94dc352,
    0x57b4d9a2, 0x1229ffa7, 0xc0f95d34, 0x2a519956, 0x7d908b66, 0x63dbd80c,
    0xb473e63e, 0x8d297d1c, 0xa6cce040, 0x2b45f844, 0xa320872e, 0xdae6c123,
    0x67349c8c, 0x705b0979, 0xca9913a5, 0x4ade3b35, 0xef6cd00d, 0x4ab1e1f4,
    0x43c5e663, 0x8c21d1bc, 0x16a7b60d, 0x7a8ff9bf, 0x1f2a753e, 0xbf186b91,
    0xada26206, 0xa3c33057, 0xae3a36a1, 0x7b108392, 0x99e41531, 0x3f1ad944,
    0xc8138825, 0xc28949a6, 0xfaf8876b, 0x9f042196, 0x68b1d623, 0x8b5114fd,
    0xdf074c46, 0x12cc86b3, 0x0a52098f, 0x9d292f9a, 0xa2f41f12, 0x43a71ed0,
    0x73f0bce6, 0x70a7e980, 0x243c6d75, 0xfdb71513, 0xa67d8a08, 0xb7e8f148,
    0xf7a644ee, 0x0f1837f2, 0x4b6694e0, 0xb7bbb3a8,
];

// On non-64-bit targets, the original source uses the 32-bit key and vector set.
#[cfg(not(target_pointer_width = "64"))]
static TEST_KEY_HSIPHASH: hsiphash_key_t = hsiphash_key_t {
    key: [0x03020100_u32, 0x07060504_u32],
};

#[cfg(not(target_pointer_width = "64"))]
static TEST_VECTORS_HSIPHASH: [u32; 64] = [
    0x5814c896, 0xe7e864ca, 0xbc4b0e30, 0x01539939, 0x7e059ea6, 0x88e3d89b,
    0xa0080b65, 0x9d38d9d6, 0x577999b1, 0xc839caed, 0xe4fa32cf, 0x959246ee,
    0x6b28096c, 0x66dd9cd6, 0x16658a7c, 0xd0257b04, 0x8b31d501, 0x2b1cd04b,
    0x06712339, 0x522aca67, 0x911bb605, 0x90a65f0e, 0xf826ef7b, 0x62512deb,
    0x57150ad7, 0x5d473507, 0x1ec47442, 0xab64afd3, 0x0a4100d0, 0x6d2ce652,
    0x2331b6a3, 0x08d8791a, 0xbc6dda8d, 0xe0f6c934, 0xb0652033, 0x9b9851cc,
    0x7c46fb7f, 0x732ba8cb, 0xf142997a, 0xfcc9aa1b, 0x05327eb2, 0xe110131c,
    0xf9e5e7c0, 0xa7d708a6, 0x11795ab1, 0x65671619, 0x9f5fff91, 0xd89c5267,
    0x007783eb, 0x95766243, 0xab639262, 0x9c7e1390, 0xc368dda6, 0x38ddc455,
    0xfa13d379, 0x979ea4e8, 0x53ecd77e, 0x2ee80657, 0x33dbb66a, 0xae3f0577,
    0x88b4c4cc, 0x3e7f480b, 0x74c1ebf8, 0x87178304,
];

macro_rules! chk {
    ($test:expr, $hash:expr, $vector:expr, $($fmt:tt)*) => {
        KUNIT_EXPECT_EQ_MSG!($test, $hash, $vector, $($fmt)*);
    };
}

unsafe fn siphash_test(test: *mut kunit) {
    let mut input = [0_u8; 64];
    let mut input_unaligned = [0_u8; 65];
    let mut i: u8 = 0;

    while i < 64 {
        input[i as usize] = i;
        input_unaligned[(i + 1) as usize] = i;
        chk!(test, siphash(input.as_ptr(), i, &TEST_KEY_SIPHASH), TEST_VECTORS_SIPHASH[i as usize], "siphash self-test aligned %u: FAIL", i + 1);
        chk!(test, siphash(input_unaligned.as_ptr().add(1), i, &TEST_KEY_SIPHASH), TEST_VECTORS_SIPHASH[i as usize], "siphash self-test unaligned %u: FAIL", i + 1);
        chk!(test, hsiphash(input.as_ptr(), i, &TEST_KEY_HSIPHASH), TEST_VECTORS_HSIPHASH[i as usize], "hsiphash self-test aligned %u: FAIL", i + 1);
        chk!(test, hsiphash(input_unaligned.as_ptr().add(1), i, &TEST_KEY_HSIPHASH), TEST_VECTORS_HSIPHASH[i as usize], "hsiphash self-test unaligned %u: FAIL", i + 1);
        i += 1;
    }
    chk!(test, siphash_1u64(0x0706050403020100, &TEST_KEY_SIPHASH), TEST_VECTORS_SIPHASH[8], "siphash self-test 1u64: FAIL");
    chk!(test, siphash_2u64(0x0706050403020100, 0x0f0e0d0c0b0a0908, &TEST_KEY_SIPHASH), TEST_VECTORS_SIPHASH[16], "siphash self-test 2u64: FAIL");
    chk!(test, siphash_3u64(0x0706050403020100, 0x0f0e0d0c0b0a0908, 0x1716151413121110, &TEST_KEY_SIPHASH), TEST_VECTORS_SIPHASH[24], "siphash self-test 3u64: FAIL");
    chk!(test, siphash_4u64(0x0706050403020100, 0x0f0e0d0c0b0a0908, 0x1716151413121110, 0x1f1e1d1c1b1a1918, &TEST_KEY_SIPHASH), TEST_VECTORS_SIPHASH[32], "siphash self-test 4u64: FAIL");
    chk!(test, siphash_1u32(0x03020100, &TEST_KEY_SIPHASH), TEST_VECTORS_SIPHASH[4], "siphash self-test 1u32: FAIL");
    chk!(test, siphash_2u32(0x03020100, 0x07060504, &TEST_KEY_SIPHASH), TEST_VECTORS_SIPHASH[8], "siphash self-test 2u32: FAIL");
    chk!(test, siphash_3u32(0x03020100, 0x07060504, 0x0b0a0908, &TEST_KEY_SIPHASH), TEST_VECTORS_SIPHASH[12], "siphash self-test 3u32: FAIL");
    chk!(test, siphash_4u32(0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, &TEST_KEY_SIPHASH), TEST_VECTORS_SIPHASH[16], "siphash self-test 4u32: FAIL");
    chk!(test, hsiphash_1u32(0x03020100, &TEST_KEY_HSIPHASH), TEST_VECTORS_HSIPHASH[4], "hsiphash self-test 1u32: FAIL");
    chk!(test, hsiphash_2u32(0x03020100, 0x07060504, &TEST_KEY_HSIPHASH), TEST_VECTORS_HSIPHASH[8], "hsiphash self-test 2u32: FAIL");
    chk!(test, hsiphash_3u32(0x03020100, 0x07060504, 0x0b0a0908, &TEST_KEY_HSIPHASH), TEST_VECTORS_HSIPHASH[12], "hsiphash self-test 3u32: FAIL");
    chk!(test, hsiphash_4u32(0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, &TEST_KEY_HSIPHASH), TEST_VECTORS_HSIPHASH[16], "hsiphash self-test 4u32: FAIL");
}

static mut SIPHASH_TEST_CASES: [kunit_case; 2] = [
    KUNIT_CASE!(siphash_test),
    kunit_case {},
];

static mut SIPHASH_TEST_SUITE: kunit_suite = kunit_suite {
    name: "siphash",
    test_cases: SIPHASH_TEST_CASES.as_mut_ptr(),
};

kunit_test_suite!(SIPHASH_TEST_SUITE);

MODULE_AUTHOR!("Jason A. Donenfeld <Jason@zx2c4.com>");
MODULE_DESCRIPTION!("Test cases for siphash.c");
MODULE_LICENSE!("Dual BSD/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
