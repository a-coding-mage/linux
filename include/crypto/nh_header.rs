/* SPDX-License-Identifier: GPL-2.0 */
/*
 * NH hash function for Adiantum
 */

/* Dependency supplied by the surrounding environment: __le64. */

/* NH parameterization: */

/* Endianness: little */
/* Word size: 32 bits (works well on NEON, SSE2, AVX2) */

/* Stride: 2 words (optimal on ARM32 NEON; works okay on other CPUs too) */
pub const NH_PAIR_STRIDE: usize = 2;
pub const NH_MESSAGE_UNIT: usize = NH_PAIR_STRIDE * 2 * core::mem::size_of::<u32>();

/* Num passes (Toeplitz iteration count): 4, to give ε = 2^{-128} */
pub const NH_NUM_PASSES: usize = 4;
pub const NH_HASH_BYTES: usize = NH_NUM_PASSES * core::mem::size_of::<u64>();

/* Max message size: 1024 bytes (32x compression factor) */
pub const NH_NUM_STRIDES: usize = 64;
pub const NH_MESSAGE_WORDS: usize = NH_PAIR_STRIDE * 2 * NH_NUM_STRIDES;
pub const NH_MESSAGE_BYTES: usize = NH_MESSAGE_WORDS * core::mem::size_of::<u32>();
pub const NH_KEY_WORDS: usize =
    NH_MESSAGE_WORDS + NH_PAIR_STRIDE * 2 * (NH_NUM_PASSES - 1);
pub const NH_KEY_BYTES: usize = NH_KEY_WORDS * core::mem::size_of::<u32>();

/**
 * nh() - NH hash function for Adiantum
 * @key: The key.  @message_len + 48 bytes of it are used.  This is NH_KEY_BYTES
 *	 if @message_len has its maximum length of NH_MESSAGE_BYTES.
 * @message: The message
 * @message_len: The message length in bytes.  Must be a multiple of 16
 *		 (NH_MESSAGE_UNIT) and at most 1024 (NH_MESSAGE_BYTES).
 * @hash: (output) The resulting hash value
 *
 * Note: the pseudocode for NH in the Adiantum paper iterates over 1024-byte
 * segments of the message, computes a 32-byte hash for each, and returns all
 * the hashes concatenated together.  In contrast, this function just hashes one
 * segment and returns one hash.  It's the caller's responsibility to call this
 * function for each 1024-byte segment and collect all the hashes.
 *
 * Context: Any context.
 */
unsafe extern "C" {
    pub fn nh(
        key: *const u32,
        message: *const u8,
        message_len: usize,
        hash: *mut __le64,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
