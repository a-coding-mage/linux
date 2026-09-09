/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2021 SiFive
 */

// Dependency corresponding to <asm/vector.h>.

#[repr(C)]
pub struct xor_block_template {
    _private: [u8; 0],
}

extern "C" {
    pub static mut xor_block_rvv: xor_block_template;
    pub static mut xor_block_8regs: xor_block_template;
    pub static mut xor_block_32regs: xor_block_template;

    fn xor_register(block: *mut xor_block_template);
    fn has_vector() -> bool;
}

pub unsafe fn arch_xor_init() {
    xor_register(&raw mut xor_block_8regs);
    xor_register(&raw mut xor_block_32regs);

    // C build-time condition: CONFIG_RISCV_ISA_V.
    #[cfg(feature = "CONFIG_RISCV_ISA_V")]
    if has_vector() {
        xor_register(&raw mut xor_block_rvv);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
