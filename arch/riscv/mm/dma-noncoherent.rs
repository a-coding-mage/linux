// SPDX-License-Identifier: GPL-2.0-only
/*
 * RISC-V specific functions to support DMA for non-coherent devices
 *
 * Copyright (c) 2021 Western Digital Corporation or its affiliates.
 */

// Dependencies supplied by the kernel's DMA, MM, cache-flush, and
// dma-noncoherent headers are intentionally left as external items.

static mut NONCOHERENT_SUPPORTED: bool = false;
#[no_mangle]
pub static mut dma_cache_alignment: usize = ARCH_DMA_MINALIGN;

#[inline]
unsafe fn arch_dma_cache_wback(paddr: PhysAddr, size: usize) {
    let vaddr = phys_to_virt(paddr);

    // CONFIG_RISCV_NONSTANDARD_CACHE_OPS
    if unlikely(noncoherent_cache_ops.wback.is_some()) {
        (noncoherent_cache_ops.wback.unwrap())(paddr, size);
        return;
    }
    // ALT_CMO_OP(CLEAN, vaddr, size, riscv_cbom_block_size)
    alt_cmo_op(CacheOperation::Clean, vaddr, size, riscv_cbom_block_size);
}

#[inline]
unsafe fn arch_dma_cache_inv(paddr: PhysAddr, size: usize) {
    let vaddr = phys_to_virt(paddr);

    // CONFIG_RISCV_NONSTANDARD_CACHE_OPS
    if unlikely(noncoherent_cache_ops.inv.is_some()) {
        (noncoherent_cache_ops.inv.unwrap())(paddr, size);
        return;
    }
    // ALT_CMO_OP(INVAL, vaddr, size, riscv_cbom_block_size)
    alt_cmo_op(CacheOperation::Inval, vaddr, size, riscv_cbom_block_size);
}

#[inline]
unsafe fn arch_dma_cache_wback_inv(paddr: PhysAddr, size: usize) {
    let vaddr = phys_to_virt(paddr);

    // CONFIG_RISCV_NONSTANDARD_CACHE_OPS
    if unlikely(noncoherent_cache_ops.wback_inv.is_some()) {
        (noncoherent_cache_ops.wback_inv.unwrap())(paddr, size);
        return;
    }
    // ALT_CMO_OP(FLUSH, vaddr, size, riscv_cbom_block_size)
    alt_cmo_op(CacheOperation::Flush, vaddr, size, riscv_cbom_block_size);
}

#[inline]
unsafe fn arch_sync_dma_clean_before_fromdevice() -> bool { true }

#[inline]
unsafe fn arch_sync_dma_cpu_needs_post_dma_flush() -> bool { true }

pub unsafe fn arch_sync_dma_for_device(paddr: PhysAddr, size: usize, dir: DmaDataDirection) {
    match dir {
        DmaDataDirection::ToDevice => arch_dma_cache_wback(paddr, size),
        DmaDataDirection::FromDevice => {
            if !arch_sync_dma_clean_before_fromdevice() {
                arch_dma_cache_inv(paddr, size);
            } else {
                // fallthrough
                if is_enabled_arch_has_sync_dma_for_cpu() && arch_sync_dma_cpu_needs_post_dma_flush() {
                    arch_dma_cache_wback(paddr, size);
                } else {
                    arch_dma_cache_wback_inv(paddr, size);
                }
            }
        }
        DmaDataDirection::Bidirectional => {
            /* Skip the invalidate here if it's done later */
            if is_enabled_arch_has_sync_dma_for_cpu() && arch_sync_dma_cpu_needs_post_dma_flush() {
                arch_dma_cache_wback(paddr, size);
            } else {
                arch_dma_cache_wback_inv(paddr, size);
            }
        }
        _ => {}
    }
}

pub unsafe fn arch_sync_dma_for_cpu(paddr: PhysAddr, size: usize, dir: DmaDataDirection) {
    match dir {
        DmaDataDirection::ToDevice => {}
        DmaDataDirection::FromDevice | DmaDataDirection::Bidirectional => {
            /* FROM_DEVICE invalidate needed if speculative CPU prefetch only */
            if arch_sync_dma_cpu_needs_post_dma_flush() {
                arch_dma_cache_inv(paddr, size);
            }
        }
        _ => {}
    }
}

pub unsafe fn arch_dma_prep_coherent(page: *mut Page, size: usize) {
    let flush_addr = page_address(page);

    // CONFIG_RISCV_NONSTANDARD_CACHE_OPS
    if unlikely(noncoherent_cache_ops.wback_inv.is_some()) {
        (noncoherent_cache_ops.wback_inv.unwrap())(page_to_phys(page), size);
        return;
    }
    // ALT_CMO_OP(FLUSH, flush_addr, size, riscv_cbom_block_size)
    alt_cmo_op(CacheOperation::Flush, flush_addr, size, riscv_cbom_block_size);
}

pub unsafe fn arch_setup_dma_ops(dev: *mut Device, coherent: bool) {
    warn_taint(!coherent && riscv_cbom_block_size > ARCH_DMA_MINALIGN,
        Taint::CpuOutOfSpec,
        "%s %s: ARCH_DMA_MINALIGN smaller than riscv,cbom-block-size (%d < %d)",
        dev_driver_string(dev), dev_name(dev), ARCH_DMA_MINALIGN, riscv_cbom_block_size);

    warn_taint(!coherent && !NONCOHERENT_SUPPORTED, Taint::CpuOutOfSpec,
        "%s %s: device non-coherent but no non-coherent operations supported",
        dev_driver_string(dev), dev_name(dev));

    dev_assign_dma_coherent(dev, coherent);
}

pub unsafe fn riscv_noncoherent_supported() {
    warn(!riscv_cbom_block_size,
        "Non-coherent DMA support enabled without a block size\n");
    NONCOHERENT_SUPPORTED = true;
}

pub unsafe fn riscv_set_dma_cache_alignment() {
    if !NONCOHERENT_SUPPORTED {
        dma_cache_alignment = 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
