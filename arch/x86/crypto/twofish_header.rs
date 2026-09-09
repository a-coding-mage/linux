/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <linux/crypto.h>
// #include <crypto/twofish.h>
// #include <crypto/b128ops.h>

use core::ffi::c_void;

/* regular block cipher functions from twofish_x86_64 module */
extern "C" {
    pub fn twofish_enc_blk(ctx: *const c_void, dst: *mut u8, src: *const u8);
    pub fn twofish_dec_blk(ctx: *const c_void, dst: *mut u8, src: *const u8);

    /* 3-way parallel cipher functions */
    pub fn __twofish_enc_blk_3way(
        ctx: *const c_void,
        dst: *mut u8,
        src: *const u8,
        xor: bool,
    );
    pub fn twofish_dec_blk_3way(ctx: *const c_void, dst: *mut u8, src: *const u8);

    /* helpers from twofish_x86_64-3way module */
    pub fn twofish_dec_blk_cbc_3way(ctx: *const c_void, dst: *mut u8, src: *const u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
