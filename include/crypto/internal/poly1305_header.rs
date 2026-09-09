/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common values for the Poly1305 algorithm
 */

// Dependency intent from <crypto/poly1305.h> and <linux/types.h> is preserved
// through the externally supplied Poly1305 types and constants below.

/*
 * Poly1305 core functions.  These only accept whole blocks; the caller must
 * handle any needed block buffering and padding.  'hibit' must be 1 for any
 * full blocks, or 0 for the final block if it had to be padded.  If 'nonce'
 * is non-NULL, then it's added at the end to compute the Poly1305 MAC.
 * Otherwise, only the ε-almost-∆-universal hash function (not the full MAC)
 * is computed.
 */

extern "C" {
    pub fn poly1305_core_setkey(
        key: *mut poly1305_core_key,
        raw_key: *const u8,
    );

    pub fn poly1305_core_blocks(
        state: *mut poly1305_state,
        key: *const poly1305_core_key,
        src: *const core::ffi::c_void,
        nblocks: u32,
        hibit: u32,
    );

    pub fn poly1305_core_emit(
        state: *const poly1305_state,
        nonce: *const u32,
        dst: *mut core::ffi::c_void,
    );
}

pub unsafe fn poly1305_core_init(state: *mut poly1305_state) {
    *state = core::mem::zeroed();
}

pub unsafe fn poly1305_block_init_generic(
    desc: *mut poly1305_block_state,
    raw_key: *const u8,
) {
    poly1305_core_init(&mut (*desc).h);
    poly1305_core_setkey(&mut (*desc).core_r, raw_key);
}

pub unsafe fn poly1305_blocks_generic(
    state: *mut poly1305_block_state,
    src: *const u8,
    len: u32,
    padbit: u32,
) {
    poly1305_core_blocks(
        &mut (*state).h,
        &(*state).core_r,
        src as *const core::ffi::c_void,
        len / POLY1305_BLOCK_SIZE,
        padbit,
    );
}

pub unsafe fn poly1305_emit_generic(
    state: *const poly1305_state,
    digest: *mut u8,
    nonce: *const u32,
) {
    poly1305_core_emit(
        state,
        nonce,
        digest as *mut core::ffi::c_void,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
