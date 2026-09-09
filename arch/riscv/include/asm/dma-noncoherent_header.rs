/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 Renesas Electronics Corp.
 */

// Source dependency: <linux/dma-direct.h>

/*
 * struct riscv_nonstd_cache_ops - Structure for non-standard CMO function pointers
 *
 * @wback: Function pointer for cache writeback
 * @inv: Function pointer for invalidating cache
 * @wback_inv: Function pointer for flushing the cache (writeback + invalidating)
 */
#[repr(C)]
pub struct riscv_nonstd_cache_ops {
    pub wback: Option<unsafe extern "C" fn(paddr: phys_addr_t, size: size_t)>,
    pub inv: Option<unsafe extern "C" fn(paddr: phys_addr_t, size: size_t)>,
    pub wback_inv: Option<unsafe extern "C" fn(paddr: phys_addr_t, size: size_t)>,
}

extern "C" {
    pub static mut noncoherent_cache_ops: riscv_nonstd_cache_ops;

    pub fn riscv_noncoherent_register_cache_ops(
        ops: *const riscv_nonstd_cache_ops,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
