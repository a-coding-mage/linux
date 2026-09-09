/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright(c) 2004 - 2009 Intel Corporation. All rights reserved. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented from the original header includes.

pub const IOAT_DMA_VERSION: &str = "5.00";
pub const IOAT_DMA_DCA_ANY_CPU: u32 = !0;

pub const MAX_SED_POOLS: usize = 5;
pub const IOAT_MAX_CHANS: usize = 4;
pub const IOAT_MAX_ORDER: u32 = 16;
pub const IOAT_MAX_DESCS: usize = 1usize << IOAT_MAX_ORDER;
pub const IOAT_CHUNK_SIZE: usize = SZ_512K;
pub const IOAT_DESCS_PER_CHUNK: usize = IOAT_CHUNK_SIZE / IOAT_DESC_SZ;
pub const NULL_DESC_BUFFER_SIZE: usize = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ioat_irq_mode {
    IOAT_NOIRQ = 0,
    IOAT_MSIX,
    IOAT_MSI,
    IOAT_INTX,
}

#[repr(C)]
pub struct ioatdma_device {
    pub pdev: *mut pci_dev,
    pub reg_base: *mut core::ffi::c_void,
    pub completion_pool: *mut dma_pool,
    pub sed_hw_pool: [*mut dma_pool; MAX_SED_POOLS],
    pub dma_dev: dma_device,
    pub version: u8,
    pub msix_entries: [msix_entry; IOAT_MAX_CHANS],
    pub idx: [*mut ioatdma_chan; IOAT_MAX_CHANS],
    pub dca: *mut dca_provider,
    pub irq_mode: ioat_irq_mode,
    pub cap: u32,
    pub chancnt: i32,
    pub msixtba0: u64,
    pub msixdata0: u64,
    pub msixpba: u32,
}

#[repr(C)]
pub struct ioat_descs {
    pub virt: *mut core::ffi::c_void,
    pub hw: dma_addr_t,
}

#[repr(C)]
pub struct ioatdma_chan {
    pub dma_chan: dma_chan,
    pub reg_base: *mut core::ffi::c_void,
    pub last_completion: dma_addr_t,
    pub cleanup_lock: spinlock_t,
    pub state: c_ulong,
    pub timer: timer_list,
    pub ioat_dma: *mut ioatdma_device,
    pub completion_dma: dma_addr_t,
    pub completion: *mut u64,
    pub cleanup_task: tasklet_struct,
    pub kobj: kobject,
    pub xfercap_log: usize,
    pub head: u16,
    pub issued: u16,
    pub tail: u16,
    pub dmacount: u16,
    pub alloc_order: u16,
    pub produce: u16,
    pub ring: *mut *mut ioat_ring_ent,
    pub prep_lock: spinlock_t,
    pub descs: [ioat_descs; IOAT_MAX_DESCS / IOAT_DESCS_PER_CHUNK],
    pub desc_chunks: i32,
    pub intr_coalesce: i32,
    pub prev_intr_coalesce: i32,
}

pub const IOAT_CHAN_DOWN: c_ulong = 0;
pub const IOAT_COMPLETION_ACK: c_ulong = 1;
pub const IOAT_RESET_PENDING: c_ulong = 2;
pub const IOAT_KOBJ_INIT_FAIL: c_ulong = 3;
pub const IOAT_RUN: c_ulong = 5;
pub const IOAT_CHAN_ACTIVE: c_ulong = 6;
pub const RESET_DELAY: u64 = msecs_to_jiffies(100);

#[repr(C)]
pub struct ioat_sed_ent {
    pub hw: *mut ioat_sed_raw_descriptor,
    pub dma: dma_addr_t,
    pub parent: *mut ioat_ring_ent,
    pub hw_pool: c_uint,
}

#[repr(C)]
pub union ioat_ring_ent_hw {
    pub hw: *mut ioat_dma_descriptor,
    pub xor: *mut ioat_xor_descriptor,
    pub xor_ex: *mut ioat_xor_ext_descriptor,
    pub pq: *mut ioat_pq_descriptor,
    pub pq_ex: *mut ioat_pq_ext_descriptor,
    pub pqu: *mut ioat_pq_update_descriptor,
    pub raw: *mut ioat_raw_descriptor,
}

#[repr(C)]
pub struct ioat_ring_ent {
    pub hw: ioat_ring_ent_hw,
    pub len: usize,
    pub txd: dma_async_tx_descriptor,
    pub result: *mut sum_check_flags,
    #[cfg(feature = "DEBUG")]
    pub id: i32,
    pub sed: *mut ioat_sed_ent,
}

extern "C" {
    pub fn system_has_dca_enabled(pdev: *mut pci_dev) -> i32;
    pub static mut ioat_pending_level: i32;
    pub static ioat_ktype: kobj_type;
    pub static mut ioat_cache: *mut kmem_cache;
    pub static mut ioat_sed_cache: *mut kmem_cache;

    pub fn ioat_dma_prep_memcpy_lock(c: *mut dma_chan, dma_dest: dma_addr_t, dma_src: dma_addr_t, len: usize, flags: c_ulong) -> *mut dma_async_tx_descriptor;
    pub fn ioat_prep_interrupt_lock(c: *mut dma_chan, flags: c_ulong) -> *mut dma_async_tx_descriptor;
    pub fn ioat_prep_xor(chan: *mut dma_chan, dest: dma_addr_t, src: *mut dma_addr_t, src_cnt: c_uint, len: usize, flags: c_ulong) -> *mut dma_async_tx_descriptor;
    pub fn ioat_prep_xor_val(chan: *mut dma_chan, src: *mut dma_addr_t, src_cnt: c_uint, len: usize, result: *mut sum_check_flags, flags: c_ulong) -> *mut dma_async_tx_descriptor;
    pub fn ioat_prep_pq(chan: *mut dma_chan, dst: *mut dma_addr_t, src: *mut dma_addr_t, src_cnt: c_uint, scf: *const u8, len: usize, flags: c_ulong) -> *mut dma_async_tx_descriptor;
    pub fn ioat_prep_pq_val(chan: *mut dma_chan, pq: *mut dma_addr_t, src: *mut dma_addr_t, src_cnt: c_uint, scf: *const u8, len: usize, pqres: *mut sum_check_flags, flags: c_ulong) -> *mut dma_async_tx_descriptor;
    pub fn ioat_prep_pqxor(chan: *mut dma_chan, dst: dma_addr_t, src: *mut dma_addr_t, src_cnt: c_uint, len: usize, flags: c_ulong) -> *mut dma_async_tx_descriptor;
    pub fn ioat_prep_pqxor_val(chan: *mut dma_chan, src: *mut dma_addr_t, src_cnt: c_uint, len: usize, result: *mut sum_check_flags, flags: c_ulong) -> *mut dma_async_tx_descriptor;

    pub fn ioat_dma_do_interrupt(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn ioat_dma_do_interrupt_msix(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn ioat_alloc_ring(c: *mut dma_chan, order: i32, flags: gfp_t) -> *mut *mut ioat_ring_ent;
    pub fn ioat_start_null_desc(ioat_chan: *mut ioatdma_chan);
    pub fn ioat_free_ring_ent(desc: *mut ioat_ring_ent, chan: *mut dma_chan);
    pub fn ioat_reset_hw(ioat_chan: *mut ioatdma_chan) -> i32;
    pub fn ioat_tx_status(c: *mut dma_chan, cookie: dma_cookie_t, txstate: *mut dma_tx_state) -> dma_status;
    pub fn ioat_cleanup_event(t: *mut tasklet_struct);
    pub fn ioat_timer_event(t: *mut timer_list);
    pub fn ioat_check_space_lock(ioat_chan: *mut ioatdma_chan, num_descs: i32) -> i32;
    pub fn ioat_issue_pending(chan: *mut dma_chan);
    pub fn is_bwd_ioat(pdev: *mut pci_dev) -> bool;
    pub fn ioat_dca_init(pdev: *mut pci_dev, iobase: *mut core::ffi::c_void) -> *mut dca_provider;
    pub fn ioat_kobject_add(ioat_dma: *mut ioatdma_device, ty: *const kobj_type);
    pub fn ioat_kobject_del(ioat_dma: *mut ioatdma_device);
    pub fn ioat_dma_setup_interrupts(ioat_dma: *mut ioatdma_device) -> i32;
    pub fn ioat_stop(ioat_chan: *mut ioatdma_chan);
}

pub unsafe fn to_ioat_chan(c: *mut dma_chan) -> *mut ioatdma_chan {
    c as *mut ioatdma_chan
}

pub unsafe fn ioat_chan_by_index(ioat_dma: *mut ioatdma_device, index: usize) -> *mut ioatdma_chan {
    (*ioat_dma).idx[index]
}

pub unsafe fn ioat_chansts(ioat_chan: *mut ioatdma_chan) -> u64 {
    readq((*ioat_chan).reg_base.add(IOAT_CHANSTS_OFFSET as usize))
}

pub unsafe fn ioat_chansts_to_addr(status: u64) -> u64 { status & IOAT_CHANSTS_COMPLETED_DESCRIPTOR_ADDR }

pub unsafe fn ioat_chanerr(ioat_chan: *mut ioatdma_chan) -> u32 {
    readl((*ioat_chan).reg_base.add(IOAT_CHANERR_OFFSET as usize))
}

pub unsafe fn ioat_suspend(ioat_chan: *mut ioatdma_chan) {
    let ver = (*(*ioat_chan).ioat_dma).version;
    writeb(IOAT_CHANCMD_SUSPEND, (*ioat_chan).reg_base.add(IOAT_CHANCMD_OFFSET(ver) as usize));
}

pub unsafe fn ioat_reset(ioat_chan: *mut ioatdma_chan) {
    let ver = (*(*ioat_chan).ioat_dma).version;
    writeb(IOAT_CHANCMD_RESET, (*ioat_chan).reg_base.add(IOAT_CHANCMD_OFFSET(ver) as usize));
}

pub unsafe fn ioat_reset_pending(ioat_chan: *mut ioatdma_chan) -> bool {
    let ver = (*(*ioat_chan).ioat_dma).version;
    let cmd = readb((*ioat_chan).reg_base.add(IOAT_CHANCMD_OFFSET(ver) as usize));
    (cmd & IOAT_CHANCMD_RESET) == IOAT_CHANCMD_RESET
}

pub fn is_ioat_active(status: c_ulong) -> bool { (status & IOAT_CHANSTS_STATUS) == IOAT_CHANSTS_ACTIVE }
pub fn is_ioat_idle(status: c_ulong) -> bool { (status & IOAT_CHANSTS_STATUS) == IOAT_CHANSTS_DONE }
pub fn is_ioat_halted(status: c_ulong) -> bool { (status & IOAT_CHANSTS_STATUS) == IOAT_CHANSTS_HALTED }
pub fn is_ioat_suspended(status: c_ulong) -> bool { (status & IOAT_CHANSTS_STATUS) == IOAT_CHANSTS_SUSPENDED }
pub fn is_ioat_bug(err: c_ulong) -> bool { err != 0 }

pub unsafe fn ioat_ring_size(ioat_chan: *mut ioatdma_chan) -> u32 { 1u32 << (*ioat_chan).alloc_order }
pub unsafe fn ioat_ring_active(ioat_chan: *mut ioatdma_chan) -> u16 { ((*ioat_chan).head.wrapping_sub((*ioat_chan).tail)) & (ioat_ring_size(ioat_chan) as u16 - 1) }
pub unsafe fn ioat_ring_pending(ioat_chan: *mut ioatdma_chan) -> u16 { ((*ioat_chan).head.wrapping_sub((*ioat_chan).issued)) & (ioat_ring_size(ioat_chan) as u16 - 1) }
pub unsafe fn ioat_ring_space(ioat_chan: *mut ioatdma_chan) -> u32 { ioat_ring_size(ioat_chan) - ioat_ring_active(ioat_chan) as u32 }
pub unsafe fn ioat_xferlen_to_descs(ioat_chan: *mut ioatdma_chan, len: usize) -> u16 {
    let mut n = (len >> (*ioat_chan).xfercap_log) as u16;
    n += (len & ((1usize << (*ioat_chan).xfercap_log) - 1) != 0) as u16;
    n
}
pub unsafe fn ioat_get_ring_ent(ioat_chan: *mut ioatdma_chan, idx: u16) -> *mut ioat_ring_ent {
    *(*ioat_chan).ring.add((idx as u32 & (ioat_ring_size(ioat_chan) - 1)) as usize)
}
pub unsafe fn ioat_set_chainaddr(ioat_chan: *mut ioatdma_chan, addr: u64) {
    writel((addr & 0x00000000_FFFFFFFF) as u32, (*ioat_chan).reg_base.add(IOAT2_CHAINADDR_OFFSET_LOW as usize));
    writel((addr >> 32) as u32, (*ioat_chan).reg_base.add(IOAT2_CHAINADDR_OFFSET_HIGH as usize));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
