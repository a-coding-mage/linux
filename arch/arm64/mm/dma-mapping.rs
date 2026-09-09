// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 ARM Ltd.
 * Author: Catalin Marinas <catalin.marinas@arm.com>
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left external to this translation unit.

pub unsafe fn arch_sync_dma_for_device(
    paddr: phys_addr_t,
    size: usize,
    dir: dma_data_direction,
) {
    let start: usize = phys_to_virt(paddr) as usize;

    dcache_clean_poc_nosync(start, start.wrapping_add(size));
}

pub unsafe fn arch_sync_dma_for_cpu(
    paddr: phys_addr_t,
    size: usize,
    dir: dma_data_direction,
) {
    let start: usize = phys_to_virt(paddr) as usize;

    if dir == DMA_TO_DEVICE {
        return;
    }

    dcache_inval_poc_nosync(start, start.wrapping_add(size));
}

pub unsafe fn arch_dma_prep_coherent(page: *mut page, size: usize) {
    let start: usize = page_address(page) as usize;

    dcache_clean_poc(start, start.wrapping_add(size));
}

pub unsafe fn arch_setup_dma_ops(dev: *mut device, mut coherent: bool) {
    let cls: i32 = cache_line_size_of_cpu();

    if !coherent && !CLIDR_LOC(read_sysreg(clidr_el1)) {
        dev_warn(dev, "CLIDR_EL1.LoC == 0, treating as coherent\n");
        coherent = true;
    }

    WARN_TAINT(
        !coherent && cls > ARCH_DMA_MINALIGN,
        TAINT_CPU_OUT_OF_SPEC,
        "%s %s: ARCH_DMA_MINALIGN smaller than CTR_EL0.CWG (%d < %d)",
        dev_driver_string(dev),
        dev_name(dev),
        ARCH_DMA_MINALIGN,
        cls,
    );

    dev_assign_dma_coherent(dev, coherent);

    xen_setup_dma_ops(dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
