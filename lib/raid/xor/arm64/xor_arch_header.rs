/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Authors: Jackie Liu <liuyun01@kylinos.cn>
 * Copyright (C) 2018,Tianjin KYLIN Information Technology Co., Ltd.
 */

// Dependency supplied by <asm/simd.h>.
#[repr(C)]
pub struct xor_block_template {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut xor_block_neon: xor_block_template;
    pub static mut xor_block_eor3: xor_block_template;

    static mut xor_block_8regs: xor_block_template;
    static mut xor_block_32regs: xor_block_template;

    fn xor_register(template: *mut xor_block_template);
    fn cpu_has_neon() -> bool;
    fn cpu_have_named_feature(feature: core::ffi::c_int) -> bool;
}

// SHA3 is supplied by the architecture feature definitions.
unsafe extern "C" {
    static SHA3: core::ffi::c_int;
}

#[inline(always)]
fn arch_xor_init() {
    unsafe {
        xor_register(&raw mut xor_block_8regs);
        xor_register(&raw mut xor_block_32regs);
        if cpu_has_neon() {
            if cpu_have_named_feature(SHA3) {
                xor_register(&raw mut xor_block_eor3);
            } else {
                xor_register(&raw mut xor_block_neon);
            }
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
