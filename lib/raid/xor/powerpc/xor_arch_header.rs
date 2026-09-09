/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *
 * Copyright (C) IBM Corporation, 2012
 *
 * Author: Anton Blanchard <anton@au.ibm.com>
 */

// Dependency supplied by the surrounding architecture-specific code:
// <asm/cpu_has_feature.h>

use core::ffi::c_int;

#[repr(C)]
pub struct xor_block_template {
    _private: [u8; 0],
}

extern "C" {
    pub static mut xor_block_altivec: xor_block_template;
    pub static mut xor_block_8regs: xor_block_template;
    pub static mut xor_block_8regs_p: xor_block_template;
    pub static mut xor_block_32regs: xor_block_template;
    pub static mut xor_block_32regs_p: xor_block_template;

    pub fn xor_register(template: *mut xor_block_template);
    pub fn cpu_has_feature(feature: c_int) -> bool;
}

// CPU_FTR_ALTIVEC is supplied by the architecture-specific CPU feature
// definitions.
extern "C" {
    pub static CPU_FTR_ALTIVEC: c_int;
}

#[inline(always)]
pub unsafe fn arch_xor_init() {
    xor_register(core::ptr::addr_of_mut!(xor_block_8regs));
    xor_register(core::ptr::addr_of_mut!(xor_block_8regs_p));
    xor_register(core::ptr::addr_of_mut!(xor_block_32regs));
    xor_register(core::ptr::addr_of_mut!(xor_block_32regs_p));

    // #ifdef CONFIG_ALTIVEC
    #[cfg(CONFIG_ALTIVEC)]
    if cpu_has_feature(CPU_FTR_ALTIVEC) {
        xor_register(core::ptr::addr_of_mut!(xor_block_altivec));
    }
    // #endif
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
