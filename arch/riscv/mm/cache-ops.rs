// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021 Western Digital Corporation or its affiliates.
 */

// Dependency supplied by the surrounding kernel sources:
// use asm::dma_noncoherent::riscv_nonstd_cache_ops;

extern "C" {
    static mut noncoherent_cache_ops: riscv_nonstd_cache_ops;
}

#[no_mangle]
pub unsafe extern "C" fn riscv_noncoherent_register_cache_ops(
    ops: *const riscv_nonstd_cache_ops,
) {
    if ops.is_null() {
        return;
    }

    noncoherent_cache_ops = *ops;
}

// EXPORT_SYMBOL_GPL(riscv_noncoherent_register_cache_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
