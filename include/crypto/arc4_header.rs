/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Common values for ARC4 Cipher Algorithm
 */

// Dependency equivalent of <linux/types.h> is supplied externally.

pub const ARC4_MIN_KEY_SIZE: usize = 1;
pub const ARC4_MAX_KEY_SIZE: usize = 256;
pub const ARC4_BLOCK_SIZE: usize = 1;

#[repr(C)]
pub struct arc4_ctx {
    pub S: [u32; 256],
    pub x: u32,
    pub y: u32,
}

unsafe extern "C" {
    pub fn arc4_setkey(
        ctx: *mut arc4_ctx,
        in_key: *const u8,
        key_len: core::ffi::c_uint,
    ) -> core::ffi::c_int;

    pub fn arc4_crypt(
        ctx: *mut arc4_ctx,
        out: *mut u8,
        input: *const u8,
        len: core::ffi::c_uint,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
