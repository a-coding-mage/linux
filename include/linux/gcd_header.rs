/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by linux/compiler.h and linux/jump_label.h.
#[repr(C)]
pub struct static_key_true {
    _private: [u8; 0],
}

extern "C" {
    pub static efficient_ffs_key: static_key_true;

    pub fn gcd(a: usize, b: usize) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
