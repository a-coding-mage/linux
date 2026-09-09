/* SPDX-License-Identifier: GPL-2.0 */
/*
 * s390 ChaCha stream cipher.
 *
 * Copyright IBM Corp. 2021
 */

// The C header guard is omitted; Rust item/module boundaries provide equivalent inclusion control.
// Types u8, u32, and size_t are supplied by the surrounding translation environment.

unsafe extern "C" {
    pub fn chacha20_vx(
        out: *mut u8,
        inp: *const u8,
        len: size_t,
        key: *const u32,
        counter: *const u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
