/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2022-2024 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

// Dependency intent: types corresponding to <linux/types.h> are represented
// by their Rust primitive equivalents below.

pub const CHACHA_KEY_SIZE: usize = 32;
pub const CHACHA_BLOCK_SIZE: usize = 64;

/**
 * struct vgetrandom_state - State used by vDSO getrandom().
 *
 * @batch:       One and a half ChaCha20 blocks of buffered RNG output.
 *
 * @key:         Key to be used for generating next batch.
 *
 * @batch_key:   Union of the prior two members, which is exactly two full
 *               ChaCha20 blocks in size, so that @batch and @key can be filled
 *               together.
 *
 * @generation:  Snapshot of @rng_info->generation in the vDSO data page at
 *               the time @key was generated.
 *
 * @pos:         Offset into @batch of the next available random byte.
 *
 * @in_use:      Reentrancy guard for reusing a state within the same thread
 *               due to signal handlers.
 */
#[repr(C)]
pub struct vgetrandom_state {
    pub batch_key: vgetrandom_state_batch_key,
    pub generation: u64,
    pub pos: u8,
    pub in_use: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union vgetrandom_state_batch_key {
    pub fields: vgetrandom_state_fields,
    pub batch_key: [u8; CHACHA_BLOCK_SIZE * 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgetrandom_state_fields {
    pub batch: [u8; CHACHA_BLOCK_SIZE * 3 / 2],
    pub key: [u32; CHACHA_KEY_SIZE / core::mem::size_of::<u32>()],
}

/**
 * __arch_chacha20_blocks_nostack - Generate ChaCha20 stream without using the stack.
 * @dst_bytes:  Destination buffer to hold @nblocks * 64 bytes of output.
 * @key:        32-byte input key.
 * @counter:    8-byte counter, read on input and updated on return.
 * @nblocks:    Number of blocks to generate.
 *
 * Generates a given positive number of blocks of ChaCha20 output with nonce=0, and does not write
 * to any stack or memory outside of the parameters passed to it, in order to mitigate stack data
 * leaking into forked child processes.
 */
unsafe extern "C" {
    pub fn __arch_chacha20_blocks_nostack(
        dst_bytes: *mut u8,
        key: *const u32,
        counter: *mut u32,
        nblocks: usize,
    );

    /**
     * __vdso_getrandom - Architecture-specific vDSO implementation of getrandom() syscall.
     * @buffer:       Passed to __cvdso_getrandom().
     * @len:          Passed to __cvdso_getrandom().
     * @flags:        Passed to __cvdso_getrandom().
     * @opaque_state: Passed to __cvdso_getrandom().
     * @opaque_len:   Passed to __cvdso_getrandom();
     *
     * This function is implemented by making a single call to to __cvdso_getrandom(), whose
     * documentation may be consulted for more information.
     *
     * Returns:       The return value of __cvdso_getrandom().
     */
    pub fn __vdso_getrandom(
        buffer: *mut core::ffi::c_void,
        len: usize,
        flags: core::ffi::c_uint,
        opaque_state: *mut core::ffi::c_void,
        opaque_len: usize,
    ) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
