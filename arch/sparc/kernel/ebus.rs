// SPDX-License-Identifier: GPL-2.0
/* ebus.c: EBUS DMA library code.
 *
 * Copyright (C) 1997  Eddie C. Dost  (ecd@skynet.be)
 * Copyright (C) 1999  David S. Miller (davem@redhat.com)
 */

// External kernel, EBUS DMA, and I/O definitions are supplied by other files.

const EBDMA_CSR: usize = 0x00;
const EBDMA_ADDR: usize = 0x04;
const EBDMA_COUNT: usize = 0x08;

const EBDMA_CSR_INT_PEND: u32 = 0x00000001;
const EBDMA_CSR_ERR_PEND: u32 = 0x00000002;
const EBDMA_CSR_DRAIN: u32 = 0x00000004;
const EBDMA_CSR_INT_EN: u32 = 0x00000010;
const EBDMA_CSR_RESET: u32 = 0x00000080;
const EBDMA_CSR_WRITE: u32 = 0x00000100;
const EBDMA_CSR_EN_DMA: u32 = 0x00000200;
const EBDMA_CSR_CYC_PEND: u32 = 0x00000400;
const EBDMA_CSR_DIAG_RD_DONE: u32 = 0x00000800;
const EBDMA_CSR_DIAG_WR_DONE: u32 = 0x00001000;
const EBDMA_CSR_EN_CNT: u32 = 0x00002000;
const EBDMA_CSR_TC: u32 = 0x00004000;
const EBDMA_CSR_DIS_CSR_DRN: u32 = 0x00010000;
const EBDMA_CSR_BURST_SZ_MASK: u32 = 0x000c0000;
const EBDMA_CSR_BURST_SZ_1: u32 = 0x00080000;
const EBDMA_CSR_BURST_SZ_4: u32 = 0x00000000;
const EBDMA_CSR_BURST_SZ_8: u32 = 0x00040000;
const EBDMA_CSR_BURST_SZ_16: u32 = 0x000c0000;
const EBDMA_CSR_DIAG_EN: u32 = 0x00100000;
const EBDMA_CSR_DIS_ERR_PEND: u32 = 0x00400000;
const EBDMA_CSR_TCI_DIS: u32 = 0x00800000;
const EBDMA_CSR_EN_NEXT: u32 = 0x01000000;
const EBDMA_CSR_DMA_ON: u32 = 0x02000000;
const EBDMA_CSR_A_LOADED: u32 = 0x04000000;
const EBDMA_CSR_NA_LOADED: u32 = 0x08000000;
const EBDMA_CSR_DEV_ID_MASK: u32 = 0xf0000000;

const EBUS_DMA_RESET_TIMEOUT: i32 = 10000;

unsafe fn __ebus_dma_reset(p: *mut ebus_dma_info, no_drain: i32) {
    let mut val: u32 = 0;
    writel(EBDMA_CSR_RESET, (*p).regs.add(EBDMA_CSR));
    udelay(1);
    if no_drain != 0 { return; }
    let mut i = EBUS_DMA_RESET_TIMEOUT;
    while i > 0 {
        val = readl((*p).regs.add(EBDMA_CSR));
        if (val & (EBDMA_CSR_DRAIN | EBDMA_CSR_CYC_PEND)) == 0 { break; }
        udelay(10);
        i -= 1;
    }
}

unsafe extern "C" fn ebus_dma_irq(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let p = dev_id as *mut ebus_dma_info;
    let mut flags: c_ulong = 0;
    let csr;
    spin_lock_irqsave(&mut (*p).lock, &mut flags);
    csr = readl((*p).regs.add(EBDMA_CSR));
    writel(csr, (*p).regs.add(EBDMA_CSR));
    spin_unlock_irqrestore(&mut (*p).lock, flags);
    if csr & EBDMA_CSR_ERR_PEND != 0 {
        printk(KERN_CRIT, "ebus_dma(%s): DMA error!\n", (*p).name);
        ((*p).callback.unwrap())(p, EBUS_DMA_EVENT_ERROR, (*p).client_cookie);
        return IRQ_HANDLED;
    } else if csr & EBDMA_CSR_INT_PEND != 0 {
        ((*p).callback.unwrap())(p, if csr & EBDMA_CSR_TC != 0 { EBUS_DMA_EVENT_DMA } else { EBUS_DMA_EVENT_DEVICE }, (*p).client_cookie);
        return IRQ_HANDLED;
    }
    IRQ_NONE
}

pub unsafe fn ebus_dma_register(p: *mut ebus_dma_info) -> i32 {
    if (*p).regs.is_null() { return -EINVAL; }
    if (*p).flags & !(EBUS_DMA_FLAG_USE_EBDMA_HANDLER | EBUS_DMA_FLAG_TCI_DISABLE) != 0 { return -EINVAL; }
    if (*p).flags & EBUS_DMA_FLAG_USE_EBDMA_HANDLER != 0 && (*p).callback.is_none() { return -EINVAL; }
    if strlen((*p).name) == 0 { return -EINVAL; }
    __ebus_dma_reset(p, 1);
    let mut csr = EBDMA_CSR_BURST_SZ_16 | EBDMA_CSR_EN_CNT;
    if (*p).flags & EBUS_DMA_FLAG_TCI_DISABLE != 0 { csr |= EBDMA_CSR_TCI_DIS; }
    writel(csr, (*p).regs.add(EBDMA_CSR));
    0
}

pub unsafe fn ebus_dma_irq_enable(p: *mut ebus_dma_info, on: i32) -> i32 {
    let mut flags: c_ulong = 0;
    if on != 0 {
        if (*p).flags & EBUS_DMA_FLAG_USE_EBDMA_HANDLER != 0 && request_irq((*p).irq, ebus_dma_irq, IRQF_SHARED, (*p).name, p as *mut c_void) != 0 { return -EBUSY; }
        spin_lock_irqsave(&mut (*p).lock, &mut flags);
        let mut csr = readl((*p).regs.add(EBDMA_CSR)); csr |= EBDMA_CSR_INT_EN; writel(csr, (*p).regs.add(EBDMA_CSR));
        spin_unlock_irqrestore(&mut (*p).lock, flags);
    } else {
        spin_lock_irqsave(&mut (*p).lock, &mut flags);
        let mut csr = readl((*p).regs.add(EBDMA_CSR)); csr &= !EBDMA_CSR_INT_EN; writel(csr, (*p).regs.add(EBDMA_CSR));
        spin_unlock_irqrestore(&mut (*p).lock, flags);
        if (*p).flags & EBUS_DMA_FLAG_USE_EBDMA_HANDLER != 0 { free_irq((*p).irq, p as *mut c_void); }
    }
    0
}

pub unsafe fn ebus_dma_unregister(p: *mut ebus_dma_info) {
    let mut flags: c_ulong = 0; let mut irq_on = 0;
    spin_lock_irqsave(&mut (*p).lock, &mut flags);
    let mut csr = readl((*p).regs.add(EBDMA_CSR));
    if csr & EBDMA_CSR_INT_EN != 0 { csr &= !EBDMA_CSR_INT_EN; writel(csr, (*p).regs.add(EBDMA_CSR)); irq_on = 1; }
    spin_unlock_irqrestore(&mut (*p).lock, flags);
    if irq_on != 0 { free_irq((*p).irq, p as *mut c_void); }
}

pub unsafe fn ebus_dma_request(p: *mut ebus_dma_info, bus_addr: dma_addr_t, len: usize) -> i32 {
    if len >= (1usize << 24) { return -EINVAL; }
    let mut flags: c_ulong = 0; spin_lock_irqsave(&mut (*p).lock, &mut flags);
    let csr = readl((*p).regs.add(EBDMA_CSR)); let err;
    if csr & EBDMA_CSR_EN_DMA == 0 { err = -EINVAL; }
    else if csr & EBDMA_CSR_NA_LOADED != 0 { err = -EBUSY; }
    else { writel(len as u32, (*p).regs.add(EBDMA_COUNT)); writel(bus_addr as u32, (*p).regs.add(EBDMA_ADDR)); err = 0; }
    spin_unlock_irqrestore(&mut (*p).lock, flags); err
}

pub unsafe fn ebus_dma_prepare(p: *mut ebus_dma_info, write: i32) {
    let mut flags: c_ulong = 0; spin_lock_irqsave(&mut (*p).lock, &mut flags); __ebus_dma_reset(p, 0);
    let mut csr = EBDMA_CSR_INT_EN | EBDMA_CSR_EN_CNT | EBDMA_CSR_BURST_SZ_16 | EBDMA_CSR_EN_NEXT;
    if write != 0 { csr |= EBDMA_CSR_WRITE; } if (*p).flags & EBUS_DMA_FLAG_TCI_DISABLE != 0 { csr |= EBDMA_CSR_TCI_DIS; }
    writel(csr, (*p).regs.add(EBDMA_CSR)); spin_unlock_irqrestore(&mut (*p).lock, flags);
}

pub unsafe fn ebus_dma_residue(p: *mut ebus_dma_info) -> u32 { readl((*p).regs.add(EBDMA_COUNT)) }
pub unsafe fn ebus_dma_addr(p: *mut ebus_dma_info) -> u32 { readl((*p).regs.add(EBDMA_ADDR)) }

pub unsafe fn ebus_dma_enable(p: *mut ebus_dma_info, on: i32) {
    let mut flags: c_ulong = 0; spin_lock_irqsave(&mut (*p).lock, &mut flags);
    let orig_csr = readl((*p).regs.add(EBDMA_CSR)); let mut csr = orig_csr;
    if on != 0 { csr |= EBDMA_CSR_EN_DMA; } else { csr &= !EBDMA_CSR_EN_DMA; }
    if (orig_csr & EBDMA_CSR_EN_DMA) != (csr & EBDMA_CSR_EN_DMA) { writel(csr, (*p).regs.add(EBDMA_CSR)); }
    spin_unlock_irqrestore(&mut (*p).lock, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
