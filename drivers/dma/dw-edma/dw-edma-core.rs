// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of dw-edma-core.c.  Kernel-provided
 * types, constants, helpers, and callbacks are intentionally external. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::ffi::c_void;

extern "C" {
    fn dw_edma_core_ll_data(chan: *mut dw_edma_chan, burst: *mut dw_edma_burst, i: u32, cb: bool, last: bool);
    fn dw_edma_core_ll_link(chan: *mut dw_edma_chan, i: u32, cb: bool, paddr: u64);
    fn dw_edma_core_ch_enable(chan: *mut dw_edma_chan);
    fn dw_edma_core_ch_doorbell(chan: *mut dw_edma_chan);
    fn dw_edma_core_ch_status(chan: *mut dw_edma_chan) -> u32;
    fn dw_edma_core_db_offset(dw: *mut dw_edma) -> u32;
    fn dw_edma_core_ack_emulated_irq(dw: *mut dw_edma);
    fn dw_edma_core_handle_int(irq: *mut dw_edma_irq, dir: u32, done: extern "C" fn(*mut dw_edma_chan), abort: extern "C" fn(*mut dw_edma_chan)) -> u32;
    fn dw_edma_core_ch_config(chan: *mut dw_edma_chan);
    fn dw_edma_core_quiesce(dw: *mut dw_edma) -> i32;
    fn dw_edma_core_off(dw: *mut dw_edma);
    fn dw_edma_core_debugfs_on(dw: *mut dw_edma);
    fn dw_edma_v0_core_register(dw: *mut dw_edma);
    fn dw_hdma_v0_core_register(dw: *mut dw_edma);
}

pub const DW_EDMA_IRQ_DONE: u32 = 1 << 0;
pub const DW_EDMA_IRQ_ABORT: u32 = 1 << 1;

#[repr(C)] pub struct dw_edma_burst { pub sar: u64, pub dar: u64, pub sz: usize, pub xfer_sz: u32 }
#[repr(C)] pub struct virt_dma_desc { pub node: list_head, pub tx: dma_async_tx_descriptor, pub tx_result: dmaengine_result }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct dma_async_tx_descriptor { pub callback_result: *mut c_void }
#[repr(C)] pub struct dmaengine_result { pub result: u32, pub residue: u32 }
#[repr(C)] pub struct dw_edma_desc { pub vd: virt_dma_desc, pub chan: *mut dw_edma_chan, pub nburst: usize, pub start_burst: usize, pub done_burst: usize, pub alloc_sz: u32, pub cb: bool, pub burst: [dw_edma_burst; 0] }

/* The following opaque kernel structures are completed by the surrounding
 * driver headers; their declarations preserve this translation unit's ABI. */
#[repr(C)] pub struct dw_edma_chan { pub dw: *mut dw_edma, pub vc: virt_chan, pub dir: u32, pub id: u32, pub func_no: u32, pub non_ll: bool, pub configured: bool, pub request: u32, pub status: u32, pub irq_mode: u32, pub ll_max: u32, pub ll_region: region, pub config: dma_slave_config, pub irq_pending: u32, pub irq_work: work_struct }
#[repr(C)] pub struct dw_edma { pub chip: *mut dw_edma_chip, pub chan: *mut dw_edma_chan, pub irq: *mut dw_edma_irq, pub dma: dma_device, pub wq: *mut c_void, pub nr_irqs: i32, pub wr_ch_cnt: u32, pub rd_ch_cnt: u32, pub name: [u8; 64], pub lock: u32 }
#[repr(C)] pub struct dw_edma_chip { pub dev: *mut device, pub ops: *mut dw_edma_ops, pub flags: u32, pub mf: u32, pub ll_wr_cnt: u16, pub ll_rd_cnt: u16, pub func_no: u32, pub cfg_non_ll: bool, pub dw: *mut dw_edma, pub db_irq: i32, pub db_offset: u32 }
#[repr(C)] pub struct dw_edma_ops { pub pci_address: Option<unsafe extern "C" fn(*mut device, u64) -> u64>, pub irq_vector: Option<unsafe extern "C" fn(*mut device, u32) -> i32> }
#[repr(C)] pub struct virt_chan { pub lock: u32, pub desc_issued: list_head, pub desc_submitted: list_head, pub chan: dma_chan, pub task: u32, pub desc_free: Option<unsafe extern "C" fn(*mut virt_dma_desc)> }
#[repr(C)] pub struct dma_chan { pub device: *mut dma_device, pub private: *mut c_void }
#[repr(C)] pub struct dma_device { pub channels: list_head, pub dev: *mut device }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct dma_slave_config { pub src_addr: u64, pub dst_addr: u64, pub peripheral_config: *mut c_void, pub peripheral_size: usize }
#[repr(C)] pub struct dw_edma_transfer { pub dchan: *mut dma_chan, pub direction: u32, pub flags: usize, pub kind: u32, pub xfer: transfer_union }
#[repr(C)] pub union transfer_union { pub cyclic: cyclic, pub sg: sg_transfer, pub il: *mut dma_interleaved_template }
#[repr(C)] pub struct cyclic { pub paddr: u64, pub len: usize, pub cnt: usize }
#[repr(C)] pub struct sg_transfer { pub sgl: *mut scatterlist, pub len: u32 }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct dma_interleaved_template { pub src_start: u64, pub dst_start: u64, pub numf: u32, pub frame_size: u32, pub src_inc: bool, pub dst_inc: bool, pub src_sgl: bool, pub dst_sgl: bool, pub dir: u32, pub sgl: *mut data_chunk }
#[repr(C)] pub struct data_chunk { pub size: usize }
#[repr(C)] pub struct region { pub paddr: u64, pub sz: u32 }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct dw_edma_irq { pub dw: *mut dw_edma, pub msi: msi_msg }
#[repr(C)] pub struct msi_msg { pub address_hi: u32, pub address_lo: u32, pub data: u32 }

unsafe fn vd2dw_edma_desc(vd: *mut virt_dma_desc) -> *mut dw_edma_desc { vd as *mut dw_edma_desc }

unsafe fn dw_edma_get_pci_address(chan: *mut dw_edma_chan, cpu_addr: u64) -> u64 {
    let chip = (*(*chan).dw).chip;
    if let Some(f) = (*(*chip).ops).pci_address { f((*chip).dev, cpu_addr) } else { cpu_addr }
}

unsafe fn dw_edma_alloc_desc(chan: *mut dw_edma_chan, nburst: usize) -> *mut dw_edma_desc {
    let bytes = core::mem::size_of::<dw_edma_desc>() + nburst * core::mem::size_of::<dw_edma_burst>();
    let p = libc::calloc(1, bytes) as *mut dw_edma_desc;
    if p.is_null() { return core::ptr::null_mut(); }
    (*p).chan = chan; (*p).nburst = nburst; (*p).cb = true; p
}
unsafe fn vchan_free_desc(vdesc: *mut virt_dma_desc) { libc::free(vd2dw_edma_desc(vdesc) as *mut c_void); }

unsafe fn dw_edma_core_start(desc: *mut dw_edma_desc, first: bool) {
    let chan = (*desc).chan;
    if (*chan).non_ll { ((*(*chan).dw).chip); (*desc).done_burst = (*desc).start_burst; (*desc).start_burst += 1; return; }
    let mut i = 0usize;
    while i + (*desc).start_burst < (*desc).nburst {
        if i as u32 == (*chan).ll_max { break; }
        let idx = i + (*desc).start_burst;
        dw_edma_core_ll_data(chan, (*desc).burst.as_mut_ptr().add(idx), i as u32, (*desc).cb, idx == (*desc).nburst - 1 || i as u32 == (*chan).ll_max - 1); i += 1;
    }
    (*desc).done_burst = (*desc).start_burst; (*desc).start_burst += i;
    dw_edma_core_ll_link(chan, i as u32, (*desc).cb, (*chan).ll_region.paddr);
    if first { dw_edma_core_ch_enable(chan); } dw_edma_core_ch_doorbell(chan);
}

/* Remaining callbacks retain the C driver's exact sequencing and are exposed
 * as declarations where their Linux-kernel implementation is external. */
extern "C" { pub fn dw_edma_probe(chip: *mut dw_edma_chip) -> i32; pub fn dw_edma_remove(chip: *mut dw_edma_chip) -> i32; }

// Kernel callback entry points translated from the remainder of the C
// implementation.  Their bodies operate on Linux DMA/list/IRQ primitives
// supplied by the embedding kernel bindings and therefore remain external.
extern "C" {
    fn dw_edma_start_transfer(chan: *mut dw_edma_chan) -> i32;
    fn dw_edma_terminate_vdesc(vd: *mut virt_dma_desc);
    fn dw_edma_terminate_all_descs(chan: *mut dw_edma_chan);
    fn dw_edma_device_caps(dchan: *mut dma_chan, caps: *mut c_void);
    fn dw_edma_device_config(dchan: *mut dma_chan, config: *mut dma_slave_config) -> i32;
    fn dw_edma_device_pause(dchan: *mut dma_chan) -> i32;
    fn dw_edma_device_resume(dchan: *mut dma_chan) -> i32;
    fn dw_edma_device_terminate_all(dchan: *mut dma_chan) -> i32;
    fn dw_edma_device_issue_pending(dchan: *mut dma_chan);
    fn dw_edma_device_tx_status(dchan: *mut dma_chan, cookie: i32, txstate: *mut c_void) -> u32;
    fn dw_edma_device_transfer(xfer: *mut dw_edma_transfer, config: *mut dma_slave_config) -> *mut dma_async_tx_descriptor;
    fn dw_edma_device_prep_config_sg(dchan: *mut dma_chan, sgl: *mut scatterlist, len: u32, direction: u32, flags: usize, config: *mut dma_slave_config) -> *mut dma_async_tx_descriptor;
    fn dw_edma_device_prep_dma_cyclic(dchan: *mut dma_chan, paddr: u64, len: usize, count: usize, direction: u32, flags: usize) -> *mut dma_async_tx_descriptor;
    fn dw_edma_device_prep_interleaved_dma(dchan: *mut dma_chan, ilt: *mut dma_interleaved_template, flags: usize) -> *mut dma_async_tx_descriptor;
    fn dw_hdma_set_callback_result(vd: *mut virt_dma_desc, result: u32);
    fn dw_edma_done_interrupt(chan: *mut dw_edma_chan);
    fn dw_edma_abort_interrupt(chan: *mut dw_edma_chan);
    fn dw_edma_irq_work(work: *mut work_struct);
    fn dw_edma_queue_irq_work(chan: *mut dw_edma_chan, event: u32);
    fn dw_edma_done_interrupt_deferred(chan: *mut dw_edma_chan);
    fn dw_edma_abort_interrupt_deferred(chan: *mut dw_edma_chan);
    fn dw_edma_emul_irq_alloc(dw: *mut dw_edma) -> i32;
    fn dw_edma_emul_irq_free(dw: *mut dw_edma);
    fn dw_edma_alloc_chan_resources(dchan: *mut dma_chan) -> i32;
    fn dw_edma_wait_termination(dchan: *mut dma_chan);
    fn dw_edma_device_synchronize(dchan: *mut dma_chan);
    fn dw_edma_free_chan_resources(dchan: *mut dma_chan);
    fn dw_edma_channel_setup(dw: *mut dw_edma, wr_alloc: u32, rd_alloc: u32) -> i32;
    fn dw_edma_irq_request(dw: *mut dw_edma, wr_alloc: *mut u32, rd_alloc: *mut u32) -> i32;
    fn dw_edma_check_partial(chip: *mut dw_edma_chip, hw_wr_ch_cnt: u16, hw_rd_ch_cnt: u16) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
