/* SPDX-License-Identifier: GPL-2.0-only */

#[repr(C)]
pub struct xor_block_template {
    _private: [u8; 0],
}

extern "C" {
    pub static mut xor_block_arm4regs: xor_block_template;
    pub static mut xor_block_neon: xor_block_template;

    pub fn xor_gen_neon_inner(
        dest: *mut core::ffi::c_void,
        srcs: *mut *mut core::ffi::c_void,
        src_cnt: u32,
        bytes: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
