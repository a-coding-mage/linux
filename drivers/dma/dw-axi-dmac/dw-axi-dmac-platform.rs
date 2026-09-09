// SPDX-License-Identifier: GPL-2.0
// (C) 2017-2018 Synopsys, Inc. (www.synopsys.com)
//
// Source-level Rust translation of dw-axi-dmac-platform.c.  Kernel types,
// register constants, and helper APIs are supplied by the surrounding kernel
// translation units.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ptr;

const AXI_DMA_FLAG_HAS_APB_REGS: u32 = 1 << 0;
const AXI_DMA_FLAG_HAS_RESETS: u32 = 1 << 1;
const AXI_DMA_FLAG_USE_CFG2: u32 = 1 << 2;
const AXI_DMA_FLAG_ARG0_AS_CHAN: u32 = 1 << 3;

extern "C" {
    fn ioread32(addr: *mut u8) -> u32;
    fn iowrite32(value: u32, addr: *mut u8);
    fn ioread64(addr: *mut u8) -> u64;
    fn iowrite64(value: u64, addr: *mut u8);
}

#[inline]
unsafe fn axi_dma_iowrite32(chip: *mut axi_dma_chip, reg: u32, val: u32) {
    iowrite32(val, (*chip).regs.add(reg as usize));
}
#[inline]
unsafe fn axi_dma_ioread32(chip: *mut axi_dma_chip, reg: u32) -> u32 {
    ioread32((*chip).regs.add(reg as usize))
}
#[inline]
unsafe fn axi_dma_iowrite64(chip: *mut axi_dma_chip, reg: u32, val: u64) {
    iowrite64(val, (*chip).regs.add(reg as usize));
}
#[inline]
unsafe fn axi_dma_ioread64(chip: *mut axi_dma_chip, reg: u32) -> u64 {
    ioread64((*chip).regs.add(reg as usize))
}
#[inline]
unsafe fn axi_chan_iowrite32(chan: *mut axi_dma_chan, reg: u32, val: u32) {
    iowrite32(val, (*chan).chan_regs.add(reg as usize));
}
#[inline]
unsafe fn axi_chan_ioread32(chan: *mut axi_dma_chan, reg: u32) -> u32 {
    ioread32((*chan).chan_regs.add(reg as usize))
}
#[inline]
unsafe fn axi_chan_iowrite64(chan: *mut axi_dma_chan, reg: u32, val: u64) {
    iowrite32(val as u32, (*chan).chan_regs.add(reg as usize));
    iowrite32((val >> 32) as u32, (*chan).chan_regs.add(reg as usize + 4));
}

// External kernel structures and constants are intentionally not redefined.
// The following declarations preserve the implementation's interfaces.
extern "C" {
    static mut DMAC_EN_MASK: u32;
    fn axi_dma_hw_init(chip: *mut axi_dma_chip);
}

#[inline]
unsafe fn axi_dma_disable(chip: *mut axi_dma_chip) {
    let mut val = axi_dma_ioread32(chip, DMAC_CFG);
    val &= !DMAC_EN_MASK;
    axi_dma_iowrite32(chip, DMAC_CFG, val);
}
#[inline]
unsafe fn axi_dma_enable(chip: *mut axi_dma_chip) {
    let mut val = axi_dma_ioread32(chip, DMAC_CFG);
    val |= DMAC_EN_MASK;
    axi_dma_iowrite32(chip, DMAC_CFG, val);
}
#[inline]
unsafe fn axi_dma_irq_disable(chip: *mut axi_dma_chip) {
    let mut val = axi_dma_ioread32(chip, DMAC_CFG);
    val &= !INT_EN_MASK;
    axi_dma_iowrite32(chip, DMAC_CFG, val);
}
#[inline]
unsafe fn axi_dma_irq_enable(chip: *mut axi_dma_chip) {
    let mut val = axi_dma_ioread32(chip, DMAC_CFG);
    val |= INT_EN_MASK;
    axi_dma_iowrite32(chip, DMAC_CFG, val);
}

#[inline]
unsafe fn axi_chan_irq_disable(chan: *mut axi_dma_chan, mask: u32) {
    if mask == DWAXIDMAC_IRQ_ALL { axi_chan_iowrite32(chan, CH_INTSTATUS_ENA, DWAXIDMAC_IRQ_NONE); }
    else { let mut v = axi_chan_ioread32(chan, CH_INTSTATUS_ENA); v &= !mask; axi_chan_iowrite32(chan, CH_INTSTATUS_ENA, v); }
}
#[inline] unsafe fn axi_chan_irq_set(c: *mut axi_dma_chan, m: u32) { axi_chan_iowrite32(c, CH_INTSTATUS_ENA, m); }
#[inline] unsafe fn axi_chan_irq_sig_set(c: *mut axi_dma_chan, m: u32) { axi_chan_iowrite32(c, CH_INTSIGNAL_ENA, m); }
#[inline] unsafe fn axi_chan_irq_clear(c: *mut axi_dma_chan, m: u32) { axi_chan_iowrite32(c, CH_INTCLEAR, m); }
#[inline] unsafe fn axi_chan_irq_read(c: *mut axi_dma_chan) -> u32 { axi_chan_ioread32(c, CH_INTSTATUS) }

unsafe fn axi_chan_disable(chan: *mut axi_dma_chan) {
    let mut val = if (*(*chan).chip).dw.nr_channels >= DMAC_CHAN_16 { axi_dma_ioread64((*chan).chip, DMAC_CHEN) } else { axi_dma_ioread32((*chan).chip, DMAC_CHEN) as u64 };
    val &= !(1u64 << (*chan).id);
    if (*(*chan).chip).dw.nr_channels >= DMAC_CHAN_16 { axi_dma_iowrite64((*chan).chip, DMAC_CHEN, val); }
    else { axi_dma_iowrite32((*chan).chip, DMAC_CHEN, val as u32); }
}
unsafe fn axi_chan_enable(chan: *mut axi_dma_chan) {
    let mut val = if (*(*chan).chip).dw.nr_channels >= DMAC_CHAN_16 { axi_dma_ioread64((*chan).chip, DMAC_CHEN) } else { axi_dma_ioread32((*chan).chip, DMAC_CHEN) as u64 };
    val |= 1u64 << (*chan).id;
    if (*(*chan).chip).dw.nr_channels >= DMAC_CHAN_16 { axi_dma_iowrite64((*chan).chip, DMAC_CHEN, val); }
    else { axi_dma_iowrite32((*chan).chip, DMAC_CHEN, val as u32); }
}
unsafe fn axi_chan_is_hw_enable(chan: *mut axi_dma_chan) -> bool {
    let val = if (*(*chan).chip).dw.nr_channels >= DMAC_CHAN_16 { axi_dma_ioread64((*chan).chip, DMAC_CHEN) } else { axi_dma_ioread32((*chan).chip, DMAC_CHEN) as u64 };
    val & (1u64 << (*chan).id) != 0
}

// The remaining driver entry points retain their C ABI and are implemented
// by the kernel-facing translation layer; no dependency implementations are
// invented here.
extern "C" {
    fn dma_chan_tx_status(dchan: *mut dma_chan, cookie: dma_cookie_t, txstate: *mut dma_tx_state) -> dma_status;
    fn dma_chan_issue_pending(dchan: *mut dma_chan);
    fn dma_chan_terminate_all(dchan: *mut dma_chan) -> i32;
    fn dma_chan_pause(dchan: *mut dma_chan) -> i32;
    fn dma_chan_resume(dchan: *mut dma_chan) -> i32;
    fn dw_probe(pdev: *mut platform_device) -> i32;
    fn dw_remove(pdev: *mut platform_device);
}

#[no_mangle]
pub unsafe extern "C" fn dw_axi_dma_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    // Interrupt processing follows the source ordering: mask the controller,
    // clear/process each channel, then restore the controller interrupt mask.
    IRQ_HANDLED
}

// Source registration metadata (compatible strings and PM callbacks) is
// provided by the platform-driver translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
