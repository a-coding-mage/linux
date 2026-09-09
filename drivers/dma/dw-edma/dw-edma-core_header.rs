/* SPDX-License-Identifier: GPL-2.0 */
/* Synopsys DesignWare eDMA core driver. */

// C dependencies: linux/atomic.h, linux/msi.h, linux/dma/edma.h,
// linux/workqueue.h, and ../virt-dma.h.

pub const EDMA_LL_SZ: usize = 24;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_edma_dir { EDMA_DIR_WRITE = 0, EDMA_DIR_READ }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_edma_request { EDMA_REQ_NONE = 0, EDMA_REQ_STOP, EDMA_REQ_PAUSE }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_edma_status { EDMA_ST_IDLE = 0, EDMA_ST_PAUSE, EDMA_ST_BUSY }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_edma_xfer_type {
    EDMA_XFER_SCATTER_GATHER = 0,
    EDMA_XFER_CYCLIC,
    EDMA_XFER_INTERLEAVED,
}

pub struct dw_edma_chan;
pub struct dw_edma_chunk;

#[repr(C)]
pub struct dw_edma_burst { pub sar: u64, pub dar: u64, pub sz: u32, pub xfer_sz: u32 }

#[repr(C)]
pub struct dw_edma_desc {
    pub vd: virt_dma_desc,
    pub chan: *mut dw_edma_chan,
    pub alloc_sz: u32,
    pub done_burst: usize,
    pub start_burst: usize,
    pub cb: u8,
    pub nburst: usize,
    // C flexible array member: burst[] __counted_by(nburst)
    pub burst: [dw_edma_burst; 0],
}

#[repr(C)]
pub struct dw_edma_chan {
    pub vc: virt_dma_chan,
    pub dw: *mut dw_edma,
    pub id: i32,
    pub dir: dw_edma_dir,
    pub func_no: u8,
    pub ll_max: u32,
    pub ll_region: dw_edma_region,
    pub msi: msi_msg,
    pub irq_mode: dw_edma_ch_irq_mode,
    pub request: dw_edma_request,
    pub status: dw_edma_status,
    pub configured: u8,
    pub config: dma_slave_config,
    pub non_ll: bool,
    pub irq_work: work_struct,
    pub irq_pending: atomic_t,
}

#[repr(C)]
pub struct dw_edma_irq {
    pub msi: msi_msg,
    pub dw: *mut dw_edma,
    pub wr_mask: [usize; 0], // DECLARE_BITMAP(wr_mask, HDMA_MAX_WR_CH)
    pub rd_mask: [usize; 0], // DECLARE_BITMAP(rd_mask, HDMA_MAX_RD_CH)
}

#[repr(C)]
pub struct dw_edma {
    pub name: [u8; 32],
    pub dma: dma_device,
    pub wr_ch_cnt: u16,
    pub rd_ch_cnt: u16,
    pub irq: *mut dw_edma_irq,
    pub nr_irqs: i32,
    pub chan: *mut dw_edma_chan,
    // WQ_HIGHPRI keeps completion processing responsive; WQ_UNBOUND lets
    // different channels run on different CPUs.
    pub wq: *mut workqueue_struct,
    pub lock: raw_spinlock_t,
    pub chip: *mut dw_edma_chip,
    pub core: *const dw_edma_core_ops,
}

pub type dw_edma_handler_t = unsafe extern "C" fn(*mut dw_edma_chan);

#[repr(C)]
pub struct dw_edma_core_ops {
    pub off: Option<unsafe extern "C" fn(*mut dw_edma)>,
    pub quiesce: Option<unsafe extern "C" fn(*mut dw_edma) -> i32>,
    pub ch_quiesce: Option<unsafe extern "C" fn(*mut dw_edma_chan) -> i32>,
    pub ch_count: Option<unsafe extern "C" fn(*mut dw_edma, dw_edma_dir) -> u16>,
    pub ch_status: Option<unsafe extern "C" fn(*mut dw_edma_chan) -> dma_status>,
    pub handle_int: Option<unsafe extern "C" fn(*mut dw_edma_irq, dw_edma_dir, dw_edma_handler_t, dw_edma_handler_t) -> irqreturn_t>,
    pub non_ll_start: Option<unsafe extern "C" fn(*mut dw_edma_chan, *mut dw_edma_burst)>,
    pub ll_data: Option<unsafe extern "C" fn(*mut dw_edma_chan, *mut dw_edma_burst, u32, bool, bool)>,
    pub ll_link: Option<unsafe extern "C" fn(*mut dw_edma_chan, u32, bool, u64)>,
    pub ch_doorbell: Option<unsafe extern "C" fn(*mut dw_edma_chan)>,
    pub ch_enable: Option<unsafe extern "C" fn(*mut dw_edma_chan)>,
    pub ch_config: Option<unsafe extern "C" fn(*mut dw_edma_chan)>,
    pub debugfs_on: Option<unsafe extern "C" fn(*mut dw_edma)>,
    pub ack_emulated_irq: Option<unsafe extern "C" fn(*mut dw_edma)>,
    pub db_offset: Option<unsafe extern "C" fn(*mut dw_edma) -> resource_size_t>,
}

#[repr(C)] pub struct dw_edma_sg { pub sgl: *mut scatterlist, pub len: u32 }
#[repr(C)] pub struct dw_edma_cyclic { pub paddr: dma_addr_t, pub len: usize, pub cnt: usize }
#[repr(C)] pub union dw_edma_xfer { pub sg: dw_edma_sg, pub cyclic: dw_edma_cyclic, pub il: *mut dma_interleaved_template }
#[repr(C)] pub struct dw_edma_transfer { pub dchan: *mut dma_chan, pub xfer: dw_edma_xfer, pub direction: dma_transfer_direction, pub flags: c_ulong, pub r#type: dw_edma_xfer_type }

pub unsafe fn vc2dw_edma_chan(vc: *mut virt_dma_chan) -> *mut dw_edma_chan { container_of!(vc, dw_edma_chan, vc) }
pub unsafe fn dchan2dw_edma_chan(dchan: *mut dma_chan) -> *mut dw_edma_chan { vc2dw_edma_chan(to_virt_chan(dchan)) }
pub unsafe fn dw_edma_core_get_ll_paddr(chan: *mut dw_edma_chan) -> u64 { if (*chan).dir == dw_edma_dir::EDMA_DIR_WRITE { (*(*chan).dw).chip.as_ref().unwrap().ll_region_wr[(*chan).id as usize].paddr } else { (*(*chan).dw).chip.as_ref().unwrap().ll_region_rd[(*chan).id as usize].paddr } }

pub unsafe fn dw_edma_core_off(dw: *mut dw_edma) { ((*(*dw).core).off.unwrap())(dw) }
pub unsafe fn dw_edma_core_quiesce(dw: *mut dw_edma) -> i32 { ((*(*dw).core).quiesce.unwrap())(dw) }
pub unsafe fn dw_edma_core_ch_quiesce(chan: *mut dw_edma_chan) -> i32 { ((*(*(*chan).dw).core).ch_quiesce.unwrap())(chan) }
pub unsafe fn dw_edma_core_ch_count(dw: *mut dw_edma, dir: dw_edma_dir) -> u16 { ((*(*dw).core).ch_count.unwrap())(dw, dir) }
pub unsafe fn dw_edma_core_ch_status(chan: *mut dw_edma_chan) -> dma_status { ((*(*(*chan).dw).core).ch_status.unwrap())(chan) }
pub unsafe fn dw_edma_core_handle_int(dw_irq: *mut dw_edma_irq, dir: dw_edma_dir, done: dw_edma_handler_t, abort: dw_edma_handler_t) -> irqreturn_t { ((*(*(*dw_irq).dw).core).handle_int.unwrap())(dw_irq, dir, done, abort) }
pub unsafe fn dw_edma_core_ch_config(chan: *mut dw_edma_chan) { ((*(*(*chan).dw).core).ch_config.unwrap())(chan) }
pub unsafe fn dw_edma_core_ll_data(chan: *mut dw_edma_chan, burst: *mut dw_edma_burst, idx: u32, cb: bool, irq: bool) { ((*(*(*chan).dw).core).ll_data.unwrap())(chan, burst, idx, cb, irq) }
pub unsafe fn dw_edma_core_ll_link(chan: *mut dw_edma_chan, idx: u32, cb: bool, addr: u64) { ((*(*(*chan).dw).core).ll_link.unwrap())(chan, idx, cb, addr) }
pub unsafe fn dw_edma_core_ch_doorbell(chan: *mut dw_edma_chan) { ((*(*(*chan).dw).core).ch_doorbell.unwrap())(chan) }
pub unsafe fn dw_edma_core_ch_enable(chan: *mut dw_edma_chan) { ((*(*(*chan).dw).core).ch_enable.unwrap())(chan) }
pub unsafe fn dw_edma_core_debugfs_on(dw: *mut dw_edma) { ((*(*dw).core).debugfs_on.unwrap())(dw) }
pub unsafe fn dw_edma_core_ack_emulated_irq(dw: *mut dw_edma) -> i32 { if (*(*dw).core).ack_emulated_irq.is_none() { return -95; } ((*(*dw).core).ack_emulated_irq.unwrap())(dw); 0 }
pub unsafe fn dw_edma_core_db_offset(dw: *mut dw_edma) -> resource_size_t { ((*(*dw).core).db_offset.unwrap())(dw) }
pub unsafe fn dw_edma_core_ch_ignore_irq(chan: *mut dw_edma_chan) -> bool { let dw = (*chan).dw; if (*(*dw).chip).flags & DW_EDMA_CHIP_LOCAL != 0 { (*chan).irq_mode == DW_EDMA_CH_IRQ_REMOTE } else { (*chan).irq_mode == DW_EDMA_CH_IRQ_LOCAL } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
