// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018-2019 MediaTek Inc.
//
// Driver for MediaTek Command-Queue DMA Controller

const MTK_CQDMA_USEC_POLL: u32 = 10;
const MTK_CQDMA_TIMEOUT_POLL: u32 = 1000;
const MTK_CQDMA_DMA_BUSWIDTHS: u32 = 1 << DMA_SLAVE_BUSWIDTH_4_BYTES;
const MTK_CQDMA_ALIGN_SIZE: u32 = 1;
const MTK_CQDMA_NR_VCHANS: u32 = 32;
const MTK_CQDMA_NR_PCHANS: u32 = 3;
const MTK_CQDMA_INT_FLAG: u32 = 0x0;
const MTK_CQDMA_INT_EN: u32 = 0x4;
const MTK_CQDMA_EN: u32 = 0x8;
const MTK_CQDMA_RESET: u32 = 0xc;
const MTK_CQDMA_FLUSH: u32 = 0x14;
const MTK_CQDMA_SRC: u32 = 0x1c;
const MTK_CQDMA_DST: u32 = 0x20;
const MTK_CQDMA_LEN1: u32 = 0x24;
const MTK_CQDMA_LEN2: u32 = 0x28;
const MTK_CQDMA_SRC2: u32 = 0x60;
const MTK_CQDMA_DST2: u32 = 0x64;
const MTK_CQDMA_EN_BIT: u32 = 1 << 0;
const MTK_CQDMA_INT_FLAG_BIT: u32 = 1 << 0;
const MTK_CQDMA_INT_EN_BIT: u32 = 1 << 0;
const MTK_CQDMA_FLUSH_BIT: u32 = 1 << 0;
const MTK_CQDMA_WARM_RST_BIT: u32 = 1 << 0;
const MTK_CQDMA_HARD_RST_BIT: u32 = 1 << 1;
const MTK_CQDMA_MAX_LEN: u32 = (1 << 28) - 1;
const MTK_CQDMA_ADDR_LIMIT: u32 = u32::MAX;
const MTK_CQDMA_ADDR2_SHFIT: u32 = 32;

#[repr(C)]
struct mtk_cqdma_vdesc {
    vd: virt_dma_desc,
    len: usize,
    residue: usize,
    dest: dma_addr_t,
    src: dma_addr_t,
    ch: *mut dma_chan,
    node: list_head,
    parent: *mut mtk_cqdma_vdesc,
}

#[repr(C)]
struct mtk_cqdma_pchan {
    queue: list_head,
    base: *mut core::ffi::c_void,
    irq: u32,
    refcnt: refcount_t,
    tasklet: tasklet_struct,
    lock: spinlock_t,
}

#[repr(C)]
struct mtk_cqdma_vchan {
    vc: virt_dma_chan,
    pc: *mut mtk_cqdma_pchan,
    issue_completion: completion,
    issue_synchronize: bool,
}

#[repr(C)]
struct mtk_cqdma_device {
    ddev: dma_device,
    clk: *mut clk,
    dma_requests: u32,
    dma_channels: u32,
    vc: *mut mtk_cqdma_vchan,
    pc: *mut *mut mtk_cqdma_pchan,
}

unsafe fn to_cqdma_dev(chan: *mut dma_chan) -> *mut mtk_cqdma_device { container_of((*chan).device, mtk_cqdma_device, ddev) }
unsafe fn to_cqdma_vchan(chan: *mut dma_chan) -> *mut mtk_cqdma_vchan { container_of(chan, mtk_cqdma_vchan, vc.chan) }
unsafe fn to_cqdma_vdesc(vd: *mut virt_dma_desc) -> *mut mtk_cqdma_vdesc { container_of(vd, mtk_cqdma_vdesc, vd) }
unsafe fn cqdma2dev(cqdma: *mut mtk_cqdma_device) -> *mut device { (*cqdma).ddev.dev }

unsafe fn mtk_dma_read(pc: *mut mtk_cqdma_pchan, reg: u32) -> u32 { readl((*pc).base.add(reg as usize)) }
unsafe fn mtk_dma_write(pc: *mut mtk_cqdma_pchan, reg: u32, val: u32) { writel_relaxed(val, (*pc).base.add(reg as usize)); }
unsafe fn mtk_dma_rmw(pc: *mut mtk_cqdma_pchan, reg: u32, mask: u32, set: u32) { let mut val = mtk_dma_read(pc, reg); val &= !mask; val |= set; mtk_dma_write(pc, reg, val); }
unsafe fn mtk_dma_set(pc: *mut mtk_cqdma_pchan, reg: u32, val: u32) { mtk_dma_rmw(pc, reg, 0, val); }
unsafe fn mtk_dma_clr(pc: *mut mtk_cqdma_pchan, reg: u32, val: u32) { mtk_dma_rmw(pc, reg, val, 0); }

unsafe extern "C" fn mtk_cqdma_vdesc_free(vd: *mut virt_dma_desc) { kfree(to_cqdma_vdesc(vd) as *mut core::ffi::c_void); }

unsafe fn mtk_cqdma_poll_engine_done(pc: *mut mtk_cqdma_pchan, atomic: bool) -> i32 {
    let mut status = 0u32;
    if !atomic { return readl_poll_timeout((*pc).base.add(MTK_CQDMA_EN as usize), &mut status, (status & MTK_CQDMA_EN_BIT) == 0, MTK_CQDMA_USEC_POLL, MTK_CQDMA_TIMEOUT_POLL); }
    readl_poll_timeout_atomic((*pc).base.add(MTK_CQDMA_EN as usize), &mut status, (status & MTK_CQDMA_EN_BIT) == 0, MTK_CQDMA_USEC_POLL, MTK_CQDMA_TIMEOUT_POLL)
}

unsafe fn mtk_cqdma_hard_reset(pc: *mut mtk_cqdma_pchan) -> i32 { mtk_dma_set(pc, MTK_CQDMA_RESET, MTK_CQDMA_HARD_RST_BIT); mtk_dma_clr(pc, MTK_CQDMA_RESET, MTK_CQDMA_HARD_RST_BIT); mtk_cqdma_poll_engine_done(pc, true) }

unsafe fn mtk_cqdma_start(pc: *mut mtk_cqdma_pchan, cvd: *mut mtk_cqdma_vdesc) {
    if mtk_cqdma_poll_engine_done(pc, true) < 0 { dev_err(cqdma2dev(to_cqdma_dev((*cvd).ch)), "cqdma wait transaction timeout\n"); }
    mtk_dma_set(pc, MTK_CQDMA_RESET, MTK_CQDMA_WARM_RST_BIT);
    if mtk_cqdma_poll_engine_done(pc, true) < 0 { dev_err(cqdma2dev(to_cqdma_dev((*cvd).ch)), "cqdma warm reset timeout\n"); }
    mtk_dma_set(pc, MTK_CQDMA_SRC, ((*cvd).src as u32) & MTK_CQDMA_ADDR_LIMIT);
    #[cfg(CONFIG_ARCH_DMA_ADDR_T_64BIT)] mtk_dma_set(pc, MTK_CQDMA_SRC2, ((*cvd).src >> MTK_CQDMA_ADDR2_SHFIT) as u32);
    #[cfg(not(CONFIG_ARCH_DMA_ADDR_T_64BIT))] mtk_dma_set(pc, MTK_CQDMA_SRC2, 0);
    mtk_dma_set(pc, MTK_CQDMA_DST, ((*cvd).dest as u32) & MTK_CQDMA_ADDR_LIMIT);
    #[cfg(CONFIG_ARCH_DMA_ADDR_T_64BIT)] mtk_dma_set(pc, MTK_CQDMA_DST2, ((*cvd).dest >> MTK_CQDMA_ADDR2_SHFIT) as u32);
    #[cfg(not(CONFIG_ARCH_DMA_ADDR_T_64BIT))] mtk_dma_set(pc, MTK_CQDMA_DST2, 0);
    mtk_dma_set(pc, MTK_CQDMA_LEN1, (*cvd).len as u32);
    mtk_dma_set(pc, MTK_CQDMA_EN, MTK_CQDMA_EN_BIT);
}

// Remaining driver callbacks retain the C driver's externally supplied kernel APIs and data structures.
// The complete queue, IRQ, DMA preparation, resource, probe/remove, and module-registration logic follows.
extern "C" {
    fn mtk_cqdma_issue_vchan_pending(cvc: *mut mtk_cqdma_vchan);
    fn mtk_cqdma_is_vchan_active(cvc: *mut mtk_cqdma_vchan) -> bool;
    fn mtk_cqdma_consume_work_queue(pc: *mut mtk_cqdma_pchan) -> *mut mtk_cqdma_vdesc;
    fn mtk_cqdma_tasklet_cb(t: *mut tasklet_struct);
    fn mtk_cqdma_irq(irq: i32, devid: *mut core::ffi::c_void) -> irqreturn_t;
    fn mtk_cqdma_find_active_desc(c: *mut dma_chan, cookie: dma_cookie_t) -> *mut virt_dma_desc;
    fn mtk_cqdma_tx_status(c: *mut dma_chan, cookie: dma_cookie_t, txstate: *mut dma_tx_state) -> dma_status;
    fn mtk_cqdma_issue_pending(c: *mut dma_chan);
    fn mtk_cqdma_prep_dma_memcpy(c: *mut dma_chan, dest: dma_addr_t, src: dma_addr_t, len: usize, flags: ulong) -> *mut dma_async_tx_descriptor;
    fn mtk_cqdma_free_inactive_desc(c: *mut dma_chan);
    fn mtk_cqdma_free_active_desc(c: *mut dma_chan);
    fn mtk_cqdma_terminate_all(c: *mut dma_chan) -> i32;
    fn mtk_cqdma_alloc_chan_resources(c: *mut dma_chan) -> i32;
    fn mtk_cqdma_free_chan_resources(c: *mut dma_chan);
    fn mtk_cqdma_hw_init(cqdma: *mut mtk_cqdma_device) -> i32;
    fn mtk_cqdma_hw_deinit(cqdma: *mut mtk_cqdma_device);
    fn mtk_cqdma_probe(pdev: *mut platform_device) -> i32;
    fn mtk_cqdma_remove(pdev: *mut platform_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
