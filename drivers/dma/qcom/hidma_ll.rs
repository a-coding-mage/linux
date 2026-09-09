// SPDX-License-Identifier: GPL-2.0-only
/* Qualcomm Technologies HIDMA DMA engine low level code */

// Dependencies supplied by the surrounding kernel translation are intentionally external.

pub const HIDMA_EVRE_SIZE: u32 = 16;
pub const HIDMA_TRCA_CTRLSTS_REG: usize = 0x000;
pub const HIDMA_TRCA_RING_LOW_REG: usize = 0x008;
pub const HIDMA_TRCA_RING_HIGH_REG: usize = 0x00C;
pub const HIDMA_TRCA_RING_LEN_REG: usize = 0x010;
pub const HIDMA_TRCA_DOORBELL_REG: usize = 0x400;
pub const HIDMA_EVCA_CTRLSTS_REG: usize = 0x000;
pub const HIDMA_EVCA_INTCTRL_REG: usize = 0x004;
pub const HIDMA_EVCA_RING_LOW_REG: usize = 0x008;
pub const HIDMA_EVCA_RING_HIGH_REG: usize = 0x00C;
pub const HIDMA_EVCA_RING_LEN_REG: usize = 0x010;
pub const HIDMA_EVCA_WRITE_PTR_REG: usize = 0x020;
pub const HIDMA_EVCA_DOORBELL_REG: usize = 0x400;
pub const HIDMA_EVCA_IRQ_STAT_REG: usize = 0x100;
pub const HIDMA_EVCA_IRQ_CLR_REG: usize = 0x108;
pub const HIDMA_EVCA_IRQ_EN_REG: usize = 0x110;
pub const HIDMA_EVRE_CFG_IDX: usize = 0;
pub const HIDMA_EVRE_ERRINFO_BIT_POS: u32 = 24;
pub const HIDMA_EVRE_CODE_BIT_POS: u32 = 28;
pub const HIDMA_CH_CONTROL_MASK: u32 = 0xff;
pub const HIDMA_CH_STATE_MASK: u32 = 0xff;
pub const HIDMA_CH_STATE_BIT_POS: u32 = 8;
pub const HIDMA_IRQ_EV_CH_EOB_IRQ_BIT_POS: u32 = 0;
pub const HIDMA_IRQ_EV_CH_WR_RESP_BIT_POS: u32 = 1;
pub const HIDMA_IRQ_TR_CH_TRE_RD_RSP_ER_BIT_POS: u32 = 9;
pub const HIDMA_IRQ_TR_CH_DATA_RD_ER_BIT_POS: u32 = 10;
pub const HIDMA_IRQ_TR_CH_DATA_WR_ER_BIT_POS: u32 = 11;
pub const HIDMA_IRQ_TR_CH_INVALID_TRE_BIT_POS: u32 = 14;
pub const ENABLE_IRQS: u32 = (1 << 0) | (1 << 1) | (1 << 9) | (1 << 10) | (1 << 11) | (1 << 14);
pub const HIDMA_ERR_INT_MASK: u32 = (1 << 14) | (1 << 9) | (1 << 1) | (1 << 10) | (1 << 11);

#[repr(i32)] pub enum ChCommand { HidmaChDisable = 0, HidmaChEnable = 1, HidmaChSuspend = 2, HidmaChReset = 9 }
#[repr(i32)] pub enum ChState { HidmaChDisabled = 0, HidmaChEnabled = 1, HidmaChRunning = 2, HidmaChSuspended = 3, HidmaChStopped = 4 }
#[repr(i32)] pub enum ErrCode { HidmaEvreStatusComplete = 1, HidmaEvreStatusError = 4 }

#[inline] pub fn hidma_ch_state(val: u32) -> u32 { (val >> HIDMA_CH_STATE_BIT_POS) & HIDMA_CH_STATE_MASK }
fn hidma_is_chan_enabled(state: i32) -> bool { state == 1 || state == 2 }

pub unsafe fn hidma_ll_free(lldev: *mut hidma_lldev, tre_ch: u32) {
    if tre_ch >= (*lldev).nr_tres { dev_err((*lldev).dev, "invalid TRE number in free:%d", tre_ch); return; }
    let tre = &mut *(*lldev).trepool.add(tre_ch as usize);
    if atomic_read(&tre.allocated) != 1 { dev_err((*lldev).dev, "trying to free an unused TRE:%d", tre_ch); return; }
    atomic_set(&mut tre.allocated, 0);
}

pub unsafe fn hidma_ll_request(lldev: *mut hidma_lldev, sig: u32, dev_name: *const i8, callback: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, data: *mut core::ffi::c_void, tre_ch: *mut u32) -> i32 {
    if tre_ch.is_null() || lldev.is_null() { return -22; }
    let mut i = 0;
    while i < (*lldev).nr_tres - 1 { if atomic_add_unless(&mut (*(*lldev).trepool.add(i as usize)).allocated, 1, 1) { break; } i += 1; }
    if i == (*lldev).nr_tres - 1 { return -12; }
    let tre = &mut *(*lldev).trepool.add(i as usize);
    tre.dma_sig = sig; tre.dev_name = dev_name; tre.callback = callback; tre.data = data; tre.idx = i; tre.status = 0; tre.queued = 0; tre.err_code = 0; tre.err_info = 0; tre.lldev = lldev;
    tre.tre_local[HIDMA_EVRE_CFG_IDX] = ((*lldev).chidx & 0xff) << 8; tre.tre_local[HIDMA_EVRE_CFG_IDX] |= 1 << 16;
    *tre_ch = i; if let Some(cb) = callback { cb(data); } 0
}

unsafe fn hidma_post_completed(lldev: *mut hidma_lldev, err_info: u8, err_code: u8) -> i32 {
    let it = (*lldev).tre_processed_off; let tre = *(*lldev).pending_tre_list.add((it / HIDMA_TRE_SIZE) as usize);
    if tre.is_null() { dev_warn((*lldev).dev, "tre_index [%d] and tre out of sync\n", it / HIDMA_TRE_SIZE); return -22; }
    *(*lldev).pending_tre_list.add((*tre).tre_index as usize) = core::ptr::null_mut();
    if atomic_dec_return(&mut (*lldev).pending_tre_count) < 0 { dev_warn((*lldev).dev, "tre count mismatch on completion"); atomic_set(&mut (*lldev).pending_tre_count, 0); }
    let mut next = it + HIDMA_TRE_SIZE; if next >= (*lldev).tre_ring_size { next -= (*lldev).tre_ring_size; } (*lldev).tre_processed_off = next;
    (*tre).err_info = err_info; (*tre).err_code = err_code; (*tre).queued = 0; kfifo_put(&mut (*lldev).handoff_fifo, tre); tasklet_schedule(&mut (*lldev).task); 0
}

pub unsafe fn hidma_cleanup_pending_tre(lldev: *mut hidma_lldev, err_info: u8, err_code: u8) { while atomic_read(&(*lldev).pending_tre_count) != 0 { if hidma_post_completed(lldev, err_info, err_code) != 0 { break; } } }

pub unsafe fn hidma_ll_enable(lldev: *mut hidma_lldev) -> i32 { let mut v = readl((*lldev).evca.add(HIDMA_EVCA_CTRLSTS_REG)); v = (v & !(HIDMA_CH_CONTROL_MASK << 16)) | (1 << 16); writel(v, (*lldev).evca.add(HIDMA_EVCA_CTRLSTS_REG)); let r = readl_poll_timeout((*lldev).evca.add(HIDMA_EVCA_CTRLSTS_REG), &mut v, hidma_is_chan_enabled(hidma_ch_state(v) as i32), 1000, 10000); if r != 0 { return r; } v = readl((*lldev).trca.add(HIDMA_TRCA_CTRLSTS_REG)); v = (v & !(HIDMA_CH_CONTROL_MASK << 16)) | (1 << 16); writel(v, (*lldev).trca.add(HIDMA_TRCA_CTRLSTS_REG)); let r = readl_poll_timeout((*lldev).trca.add(HIDMA_TRCA_CTRLSTS_REG), &mut v, hidma_is_chan_enabled(hidma_ch_state(v) as i32), 1000, 10000); if r != 0 { return r; } (*lldev).trch_state = 1; (*lldev).evch_state = 1; writel(ENABLE_IRQS, (*lldev).evca.add(HIDMA_EVCA_IRQ_EN_REG)); 0 }

pub unsafe fn hidma_ll_isenabled(lldev: *mut hidma_lldev) -> bool { let mut v = readl((*lldev).trca.add(HIDMA_TRCA_CTRLSTS_REG)); (*lldev).trch_state = hidma_ch_state(v) as i32; v = readl((*lldev).evca.add(HIDMA_EVCA_CTRLSTS_REG)); (*lldev).evch_state = hidma_ch_state(v) as i32; hidma_is_chan_enabled((*lldev).trch_state) && hidma_is_chan_enabled((*lldev).evch_state) }

pub unsafe fn hidma_ll_start(lldev: *mut hidma_lldev) { writel((*lldev).tre_write_offset, (*lldev).trca.add(HIDMA_TRCA_DOORBELL_REG)); }

pub unsafe fn hidma_ll_queue_request(lldev: *mut hidma_lldev, tre_ch: u32) { let tre = &mut *(*lldev).trepool.add(tre_ch as usize); tre.tre_index = (*lldev).tre_write_offset / HIDMA_TRE_SIZE; *(*lldev).pending_tre_list.add(tre.tre_index as usize) = tre; core::ptr::copy_nonoverlapping(tre.tre_local.as_ptr(), (*lldev).tre_ring.add((*lldev).tre_write_offset as usize), HIDMA_TRE_SIZE as usize); tre.err_code = 0; tre.err_info = 0; tre.queued = 1; atomic_inc(&mut (*lldev).pending_tre_count); (*lldev).tre_write_offset = ((*lldev).tre_write_offset + HIDMA_TRE_SIZE) % (*lldev).tre_ring_size; }

pub unsafe fn hidma_ll_setup_irq(lldev: *mut hidma_lldev, msi: bool) { (*lldev).msi_support = msi; writel(0, (*lldev).evca.add(HIDMA_EVCA_IRQ_CLR_REG)); writel(0, (*lldev).evca.add(HIDMA_EVCA_IRQ_EN_REG)); let mut v = readl((*lldev).evca.add(HIDMA_EVCA_INTCTRL_REG)) & !0xf; if !msi { v |= 1; } writel(v, (*lldev).evca.add(HIDMA_EVCA_INTCTRL_REG)); writel(ENABLE_IRQS, (*lldev).evca.add(HIDMA_EVCA_IRQ_CLR_REG)); writel(ENABLE_IRQS, (*lldev).evca.add(HIDMA_EVCA_IRQ_EN_REG)); }

pub unsafe fn hidma_ll_status(lldev: *mut hidma_lldev, tre_ch: u32) -> dma_status { let e = (*(*lldev).trepool.add(tre_ch as usize)).err_code; if e & 1 != 0 { DMA_COMPLETE } else if e & 4 != 0 { DMA_ERROR } else { DMA_IN_PROGRESS } }

// Remaining declarations and hardware-specific helpers are supplied by hidma.h/kernel bindings.
extern "C" { fn dev_err(dev: *mut device, fmt: *const i8, ...); fn dev_warn(dev: *mut device, fmt: *const i8, ...); fn readl(p: *mut u8) -> u32; fn writel(v: u32, p: *mut u8); fn readl_relaxed(p: *mut u8) -> u32; fn writel_relaxed(v: u32, p: *mut u8); fn readl_poll_timeout(p: *mut u8, v: *mut u32, cond: bool, delay: u32, timeout: u32) -> i32; fn atomic_read(p: &i32) -> i32; fn atomic_set(p: &mut i32, v: i32); fn atomic_inc(p: &mut i32); fn atomic_dec_return(p: &mut i32) -> i32; fn atomic_add_unless(p: &mut i32, a: i32, u: i32) -> bool; fn kfifo_put<T>(f: *mut kfifo, v: T) -> bool; fn tasklet_schedule(t: *mut tasklet); }
extern "C" { fn hidma_ll_disable(lldev: *mut hidma_lldev) -> i32; }
#[repr(C)] pub struct hidma_lldev { pub nr_tres: u32, pub trepool: *mut hidma_tre, pub pending_tre_list: *mut *mut hidma_tre, pub tre_processed_off: u32, pub evre_processed_off: u32, pub tre_write_offset: u32, pub tre_ring_size: u32, pub evre_ring_size: u32, pub tre_ring: *mut u32, pub evre_ring: *mut u32, pub evca: *mut u8, pub trca: *mut u8, pub dev: *mut device, pub pending_tre_count: i32, pub handoff_fifo: kfifo, pub task: tasklet, pub trch_state: i32, pub evch_state: i32, pub msi_support: bool, pub chidx: u8, pub initialized: i32 }
#[repr(C)] pub struct hidma_tre { pub allocated: i32, pub dma_sig: u32, pub dev_name: *const i8, pub callback: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub data: *mut core::ffi::c_void, pub idx: u32, pub status: u32, pub queued: u32, pub err_code: u8, pub err_info: u8, pub lldev: *mut hidma_lldev, pub tre_index: u32, pub tre_local: [u32; 8], pub int_flags: u32 }
pub type dma_status = i32; pub const DMA_COMPLETE: i32 = 0; pub const DMA_ERROR: i32 = 1; pub const DMA_IN_PROGRESS: i32 = 2; pub const HIDMA_TRE_SIZE: u32 = 32; #[repr(C)] pub struct device; #[repr(C)] pub struct kfifo; #[repr(C)] pub struct tasklet;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
