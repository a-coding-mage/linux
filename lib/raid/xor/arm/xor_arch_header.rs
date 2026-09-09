/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 2001 Russell King
 */

// C dependencies: <asm/neon.h> and "xor-neon.h".

// Opaque declarations supplied by the dependent translation units.
unsafe extern "C" {
    static xor_block_arm4regs: core::ffi::c_void;
    static xor_block_8regs: core::ffi::c_void;
    static xor_block_32regs: core::ffi::c_void;
    static xor_block_neon: core::ffi::c_void;

    fn xor_register(block: *const core::ffi::c_void);
    fn cpu_has_neon() -> bool;
}

#[inline(always)]
unsafe fn arch_xor_init() {
    xor_register(&raw const xor_block_arm4regs);
    xor_register(&raw const xor_block_8regs);
    xor_register(&raw const xor_block_32regs);
    // C build-time condition: CONFIG_KERNEL_MODE_NEON.
    #[cfg(feature = "CONFIG_KERNEL_MODE_NEON")]
    {
        if cpu_has_neon() {
            xor_register(&raw const xor_block_neon);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
