/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency supplied by <asm/special_insns.h>.

extern "C" {
    pub static mut xor_block_alpha: xor_block_template;
    pub static mut xor_block_alpha_prefetch: xor_block_template;
    pub static mut xor_block_8regs: xor_block_template;
    pub static mut xor_block_32regs: xor_block_template;

    pub fn implver() -> i32;
    pub fn xor_force(block: *mut xor_block_template);
    pub fn xor_register(block: *mut xor_block_template);
}

// Opaque declaration supplied by the XOR implementation.
#[repr(C)]
pub struct xor_block_template {
    _private: [u8; 0],
}

// Build-time architecture constant supplied by the target architecture.
pub const IMPLVER_EV6: i32 = 1;

/*
 * Force the use of alpha_prefetch if EV6, as it is significantly faster in the
 * cold cache case.
 */
#[inline(always)]
pub unsafe fn arch_xor_init() {
    if implver() == IMPLVER_EV6 {
        xor_force(&raw mut xor_block_alpha_prefetch);
    } else {
        xor_register(&raw mut xor_block_8regs);
        xor_register(&raw mut xor_block_32regs);
        xor_register(&raw mut xor_block_alpha);
        xor_register(&raw mut xor_block_alpha_prefetch);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
