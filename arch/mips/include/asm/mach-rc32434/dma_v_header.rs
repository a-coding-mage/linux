/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2002 Integrated Device Technology, Inc.
 *		All rights reserved.
 *
 * DMA register definition.
 *
 * Author : ryan.holmQVist@idt.com
 * Date	  : 20011005
 */

// Dependency declarations from <asm/mach-rc32434/dma.h> and
// <asm/mach-rc32434/rc32434.h> are supplied externally.

pub const DMA_CHAN_OFFSET: u32 = 0x14;

#[inline]
pub const fn IS_DMA_USED(x: u32) -> bool {
    (x & (DMA_DESC_FINI | DMA_DESC_DONE | DMA_DESC_TERM)) != 0
}

#[inline]
pub const fn DMA_COUNT(count: u32) -> u32 {
    count & DMA_DESC_COUNT_MSK
}

pub const DMA_HALT_TIMEOUT: i32 = 500;

#[inline]
pub unsafe fn rc32434_halt_dma(ch: *mut dma_reg) -> i32 {
    let mut timeout: i32 = 1;
    if __raw_readl(core::ptr::addr_of!((*ch).dmac)) & DMA_CHAN_RUN_BIT != 0 {
        __raw_writel(0, core::ptr::addr_of_mut!((*ch).dmac));
        timeout = DMA_HALT_TIMEOUT;
        while timeout > 0 {
            if __raw_readl(core::ptr::addr_of!((*ch).dmas)) & DMA_STAT_HALT != 0 {
                __raw_writel(0, core::ptr::addr_of_mut!((*ch).dmas));
                break;
            }
            timeout -= 1;
        }
    }

    if timeout != 0 { 0 } else { 1 }
}

#[inline]
pub unsafe fn rc32434_start_dma(ch: *mut dma_reg, dma_addr: u32) {
    __raw_writel(0, core::ptr::addr_of_mut!((*ch).dmandptr));
    __raw_writel(dma_addr, core::ptr::addr_of_mut!((*ch).dmadptr));
}

#[inline]
pub unsafe fn rc32434_chain_dma(ch: *mut dma_reg, dma_addr: u32) {
    __raw_writel(dma_addr, core::ptr::addr_of_mut!((*ch).dmandptr));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
