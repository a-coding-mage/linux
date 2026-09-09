// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007 Lemote Inc. & Institute of Computing Technology
 * Author: Fuxin Zhang, zhangfx@lemote.com
 */

// Dependencies supplied by the corresponding Linux/MIPS headers.
unsafe extern "C" {
    fn loongson2ef_pcibios_init();
}

unsafe extern "C" fn wbflush_loongson() {
    unsafe {
        core::arch::asm!(
            ".set\\tpush",
            ".set\\tnoreorder",
            ".set mips3",
            "sync",
            "nop",
            ".set\\tpop",
            ".set mips0",
            options(nostack, preserves_flags),
        );
    }
}

// EXPORT_SYMBOL(__wbflush)
#[no_mangle]
pub static mut __wbflush: unsafe extern "C" fn() = wbflush_loongson;

// __init
#[no_mangle]
pub unsafe extern "C" fn plat_mem_setup() {
    unsafe {
        loongson2ef_pcibios_init();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
