// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 Ventana Micro Systems Inc.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// cache-flush and DMA-noncoherent definitions, noncoherent_cache_ops,
// virt_to_phys, ALT_CMO_OP, riscv_cbom_block_size, and unlikely.

pub unsafe fn arch_wb_cache_pmem(addr: *mut core::ffi::c_void, size: usize) {
    #[cfg(feature = "CONFIG_RISCV_NONSTANDARD_CACHE_OPS")]
    {
        if unlikely(noncoherent_cache_ops.wback) {
            (noncoherent_cache_ops.wback)(virt_to_phys(addr), size);
            return;
        }
    }

    ALT_CMO_OP!(CLEAN, addr, size, riscv_cbom_block_size);
}

// EXPORT_SYMBOL_GPL(arch_wb_cache_pmem);

pub unsafe fn arch_invalidate_pmem(addr: *mut core::ffi::c_void, size: usize) {
    #[cfg(feature = "CONFIG_RISCV_NONSTANDARD_CACHE_OPS")]
    {
        if unlikely(noncoherent_cache_ops.inv) {
            (noncoherent_cache_ops.inv)(virt_to_phys(addr), size);
            return;
        }
    }

    ALT_CMO_OP!(INVAL, addr, size, riscv_cbom_block_size);
}

// EXPORT_SYMBOL_GPL(arch_invalidate_pmem);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
