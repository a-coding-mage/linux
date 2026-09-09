// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * The "hash function" used as the core of the ChaCha stream cipher (RFC7539)
 *
 * Copyright (C) 2015 Martin Willi
 */

unsafe fn chacha_permute(state: &mut crate::chacha_state, nrounds: i32) {
    let x = state.x.as_mut_ptr();

    /* whitelist the allowed round counts */
    debug_assert!(nrounds == 20 || nrounds == 12);

    let mut i = 0;
    while i < nrounds {
        (*x.add(0)) = (*x.add(0)).wrapping_add(*x.add(4));
        *x.add(12) = (*x.add(12) ^ *x.add(0)).rotate_left(16);
        *x.add(1) = (*x.add(1)).wrapping_add(*x.add(5));
        *x.add(13) = (*x.add(13) ^ *x.add(1)).rotate_left(16);
        *x.add(2) = (*x.add(2)).wrapping_add(*x.add(6));
        *x.add(14) = (*x.add(14) ^ *x.add(2)).rotate_left(16);
        *x.add(3) = (*x.add(3)).wrapping_add(*x.add(7));
        *x.add(15) = (*x.add(15) ^ *x.add(3)).rotate_left(16);

        *x.add(8) = (*x.add(8)).wrapping_add(*x.add(12));
        *x.add(4) = (*x.add(4) ^ *x.add(8)).rotate_left(12);
        *x.add(9) = (*x.add(9)).wrapping_add(*x.add(13));
        *x.add(5) = (*x.add(5) ^ *x.add(9)).rotate_left(12);
        *x.add(10) = (*x.add(10)).wrapping_add(*x.add(14));
        *x.add(6) = (*x.add(6) ^ *x.add(10)).rotate_left(12);
        *x.add(11) = (*x.add(11)).wrapping_add(*x.add(15));
        *x.add(7) = (*x.add(7) ^ *x.add(11)).rotate_left(12);

        *x.add(0) = (*x.add(0)).wrapping_add(*x.add(4));
        *x.add(12) = (*x.add(12) ^ *x.add(0)).rotate_left(8);
        *x.add(1) = (*x.add(1)).wrapping_add(*x.add(5));
        *x.add(13) = (*x.add(13) ^ *x.add(1)).rotate_left(8);
        *x.add(2) = (*x.add(2)).wrapping_add(*x.add(6));
        *x.add(14) = (*x.add(14) ^ *x.add(2)).rotate_left(8);
        *x.add(3) = (*x.add(3)).wrapping_add(*x.add(7));
        *x.add(15) = (*x.add(15) ^ *x.add(3)).rotate_left(8);

        *x.add(8) = (*x.add(8)).wrapping_add(*x.add(12));
        *x.add(4) = (*x.add(4) ^ *x.add(8)).rotate_left(7);
        *x.add(9) = (*x.add(9)).wrapping_add(*x.add(13));
        *x.add(5) = (*x.add(5) ^ *x.add(9)).rotate_left(7);
        *x.add(10) = (*x.add(10)).wrapping_add(*x.add(14));
        *x.add(6) = (*x.add(6) ^ *x.add(10)).rotate_left(7);
        *x.add(11) = (*x.add(11)).wrapping_add(*x.add(15));
        *x.add(7) = (*x.add(7) ^ *x.add(11)).rotate_left(7);

        *x.add(0) = (*x.add(0)).wrapping_add(*x.add(5));
        *x.add(15) = (*x.add(15) ^ *x.add(0)).rotate_left(16);
        *x.add(1) = (*x.add(1)).wrapping_add(*x.add(6));
        *x.add(12) = (*x.add(12) ^ *x.add(1)).rotate_left(16);
        *x.add(2) = (*x.add(2)).wrapping_add(*x.add(7));
        *x.add(13) = (*x.add(13) ^ *x.add(2)).rotate_left(16);
        *x.add(3) = (*x.add(3)).wrapping_add(*x.add(4));
        *x.add(14) = (*x.add(14) ^ *x.add(3)).rotate_left(16);

        *x.add(10) = (*x.add(10)).wrapping_add(*x.add(15));
        *x.add(5) = (*x.add(5) ^ *x.add(10)).rotate_left(12);
        *x.add(11) = (*x.add(11)).wrapping_add(*x.add(12));
        *x.add(6) = (*x.add(6) ^ *x.add(11)).rotate_left(12);
        *x.add(8) = (*x.add(8)).wrapping_add(*x.add(13));
        *x.add(7) = (*x.add(7) ^ *x.add(8)).rotate_left(12);
        *x.add(9) = (*x.add(9)).wrapping_add(*x.add(14));
        *x.add(4) = (*x.add(4) ^ *x.add(9)).rotate_left(12);

        *x.add(0) = (*x.add(0)).wrapping_add(*x.add(5));
        *x.add(15) = (*x.add(15) ^ *x.add(0)).rotate_left(8);
        *x.add(1) = (*x.add(1)).wrapping_add(*x.add(6));
        *x.add(12) = (*x.add(12) ^ *x.add(1)).rotate_left(8);
        *x.add(2) = (*x.add(2)).wrapping_add(*x.add(7));
        *x.add(13) = (*x.add(13) ^ *x.add(2)).rotate_left(8);
        *x.add(3) = (*x.add(3)).wrapping_add(*x.add(4));
        *x.add(14) = (*x.add(14) ^ *x.add(3)).rotate_left(8);

        *x.add(10) = (*x.add(10)).wrapping_add(*x.add(15));
        *x.add(5) = (*x.add(5) ^ *x.add(10)).rotate_left(7);
        *x.add(11) = (*x.add(11)).wrapping_add(*x.add(12));
        *x.add(6) = (*x.add(6) ^ *x.add(11)).rotate_left(7);
        *x.add(8) = (*x.add(8)).wrapping_add(*x.add(13));
        *x.add(7) = (*x.add(7) ^ *x.add(8)).rotate_left(7);
        *x.add(9) = (*x.add(9)).wrapping_add(*x.add(14));
        *x.add(4) = (*x.add(4) ^ *x.add(9)).rotate_left(7);
        i += 2;
    }
}

/**
 * chacha_block_generic - generate one keystream block and increment block counter
 * @state: input state matrix
 * @out: output keystream block
 * @nrounds: number of rounds (20 or 12; 20 is recommended)
 *
 * This is the ChaCha core, a function from 64-byte strings to 64-byte strings.
 * The caller has already converted the endianness of the input.  This function
 * also handles incrementing the block counter in the input matrix.
 */
pub unsafe fn chacha_block_generic(
    state: *mut crate::chacha_state,
    out: *mut u8,
    nrounds: i32,
) {
    let mut permuted_state = *state;
    chacha_permute(&mut permuted_state, nrounds);
    for i in 0..(*state).x.len() {
        let value = permuted_state.x[i].wrapping_add((*state).x[i]);
        out.add(i * core::mem::size_of::<u32>()).cast::<u32>().write_unaligned(value.to_le());
    }
    (*state).x[12] = (*state).x[12].wrapping_add(1);
    crate::chacha_zeroize_state(&mut permuted_state);
}

/**
 * hchacha_block_generic - abbreviated ChaCha core, for XChaCha
 * @state: input state matrix
 * @out: the output words
 * @nrounds: number of rounds (20 or 12; 20 is recommended)
 */
pub unsafe fn hchacha_block_generic(
    state: *const crate::chacha_state,
    out: *mut u32,
    nrounds: i32,
) {
    let mut permuted_state = *state;
    chacha_permute(&mut permuted_state, nrounds);
    core::ptr::copy_nonoverlapping(permuted_state.x.as_ptr(), out, 4);
    core::ptr::copy_nonoverlapping(permuted_state.x.as_ptr().add(12), out.add(4), 4);
    crate::chacha_zeroize_state(&mut permuted_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
