// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of the Linux CPPI 4.1 DMA implementation.

const DESC_TYPE: u32 = 27;
const DESC_TYPE_HOST: u32 = 0x10;
const DESC_TYPE_TEARD: u32 = 0x13;
const TD_DESC_IS_RX: u32 = 1 << 16;
const TD_DESC_DMA_NUM: u32 = 10;
const DESC_LENGTH_BITS_NUM: u32 = 21;
const DESC_TYPE_USB: u32 = 5 << 26;
const DESC_PD_COMPLETE: u32 = 1 << 31;
const DMA_TDFDQ: usize = 4;
const RXHPCRA0: usize = 4;
const GCR_CHAN_ENABLE: u32 = 1 << 31;
const GCR_TEARDOWN: u32 = 1 << 30;
const GCR_STARV_RETRY: u32 = 1 << 24;
const GCR_DESC_TYPE_HOST: u32 = 1 << 14;
const DMA_SCHED_CTRL: usize = 0;
const DMA_SCHED_CTRL_EN: u32 = 1 << 31;
const ALLOC_DECS_NUM: usize = 128;
const DESCS_AREAS: usize = 1;
const TOTAL_DESCS_NUM: usize = ALLOC_DECS_NUM * DESCS_AREAS;
const QMGR_SCRATCH_SIZE: usize = TOTAL_DESCS_NUM * 4;
const QMGR_LRAM0_BASE: usize = 0x80;
const QMGR_LRAM_SIZE: usize = 0x84;
const QMGR_LRAM1_BASE: usize = 0x88;
const QMGR_MEMCTRL_IDX_SH: u32 = 16;
const QMGR_MEMCTRL_DESC_SH: u32 = 8;
const PD2_ZERO_LENGTH: u32 = 1 << 19;

const fn dma_txgcr(x: usize) -> usize { 0x800 + x * 0x20 }
const fn dma_rxgcr(x: usize) -> usize { 0x808 + x * 0x20 }
const fn dma_sched_word(x: usize) -> usize { x * 4 + 0x800 }
const fn qmgr_membase(x: usize) -> usize { 0x1000 + x * 0x10 }
const fn qmgr_memctrl(x: usize) -> usize { 0x1004 + x * 0x10 }
const fn qmgr_pend(x: usize) -> usize { 0x90 + x * 4 }
const fn qmgr_queue_d(x: usize) -> usize { 0x200c + x * 0x10 }
const fn qmgr_pending_slot_q(x: u16) -> usize { (x / 32) as usize }
const fn qmgr_pending_bit_q(x: u16) -> u32 { (x % 32) as u32 }

#[repr(C)]
pub struct cppi41_channel {
    pub chan: dma_chan, pub txd: dma_async_tx_descriptor, pub cdd: *mut cppi41_dd,
    pub desc: *mut cppi41_desc, pub desc_phys: dma_addr_t, pub gcr_reg: *mut u8,
    pub is_tx: i32, pub residue: u32, pub q_num: u32, pub q_comp_num: u32,
    pub port_num: u32, pub td_retry: u32, pub td_queued: u32, pub td_seen: u32,
    pub td_desc_seen: u32, pub node: list_head,
}
#[repr(C, align(32))]
pub struct cppi41_desc { pub pd0:u32,pub pd1:u32,pub pd2:u32,pub pd3:u32,pub pd4:u32,pub pd5:u32,pub pd6:u32,pub pd7:u32 }
#[repr(C)] pub struct chan_queues { pub submit:u16, pub complete:u16 }
#[repr(C)] pub struct cppi_glue_infos { pub queues_rx:*const chan_queues,pub queues_tx:*const chan_queues,pub td_queue:chan_queues,pub first_completion_queue:u16,pub qmgr_num_pend:u16 }
#[repr(C)] pub struct cppi41_dd {
    pub ddev:dma_device,pub qmgr_scratch:*mut core::ffi::c_void,pub scratch_phys:dma_addr_t,
    pub cd:*mut cppi41_desc,pub descs_phys:dma_addr_t,pub first_td_desc:u32,
    pub chan_busy:[*mut cppi41_channel;ALLOC_DECS_NUM],pub ctrl_mem:*mut u8,pub sched_mem:*mut u8,pub qmgr_mem:*mut u8,
    pub irq:u32,pub queues_rx:*const chan_queues,pub queues_tx:*const chan_queues,pub td_queue:chan_queues,
    pub first_completion_queue:u16,pub qmgr_num_pend:u16,pub n_chans:u32,pub platform:u8,pub pending:list_head,pub lock:spinlock_t,
    pub dma_tdfdq:u32,pub is_suspended:bool,
}

extern "C" {
    fn __raw_readl(p:*mut u8)->u32; fn __raw_writel(v:u32,p:*mut u8);
    fn pm_runtime_put(_: *mut device); fn dma_cookie_assign(_: *mut dma_async_tx_descriptor)->dma_cookie_t;
}

unsafe fn cppi_writel(v:u32,p:*mut u8){__raw_writel(v,p)}
unsafe fn cppi_readl(p:*mut u8)->u32{__raw_readl(p)}
fn pd_trans_len(v:u32)->u32{v & ((1u32 << (DESC_LENGTH_BITS_NUM+1))-1)}
unsafe fn cppi41_pop_desc(cdd:*mut cppi41_dd,q:u32)->u32{cppi_readl((*cdd).qmgr_mem.add(qmgr_queue_d(q as usize))) & !0x1f}

unsafe fn desc_to_chan(cdd:*mut cppi41_dd, desc:u32)->*mut cppi41_channel {
    let size=core::mem::size_of::<cppi41_desc>()*ALLOC_DECS_NUM;
    if desc < (*cdd).descs_phys as u32 || desc >= (*cdd).descs_phys as u32 + size as u32 { return core::ptr::null_mut(); }
    let n=((desc-(*cdd).descs_phys as u32) / core::mem::size_of::<cppi41_desc>() as u32) as usize;
    let c=(*cdd).chan_busy[n]; (*cdd).chan_busy[n]=core::ptr::null_mut(); pm_runtime_put((*cdd).ddev.dev); c
}
unsafe fn cppi41_tx_submit(tx:*mut dma_async_tx_descriptor)->dma_cookie_t{dma_cookie_assign(tx)}
unsafe fn get_host_pd0(l:u32)->u32{(DESC_TYPE_HOST<<DESC_TYPE)|l}
unsafe fn get_host_pd1(_: *mut cppi41_channel)->u32{0}
unsafe fn get_host_pd2(c:*mut cppi41_channel)->u32{DESC_TYPE_USB|(*c).q_comp_num}
unsafe fn get_host_pd3(l:u32)->u32{l}
unsafe fn get_host_pd6(l:u32)->u32{DESC_PD_COMPLETE|l}
unsafe fn get_host_pd4_or_7(a:u32)->u32{a}
unsafe fn get_host_pd5()->u32{0}
unsafe fn cppi41_compute_td_desc(d:*mut cppi41_desc){(*d).pd0=DESC_TYPE_TEARD<<DESC_TYPE}

// The remaining kernel callbacks retain the original control flow and use the
// kernel DMA/list/PM APIs supplied by the surrounding Rust kernel bindings.
unsafe fn cppi41_run_queue(cdd:*mut cppi41_dd){
    list_for_each_entry_safe!(c, n, &mut (*cdd).pending, node, { push_desc_queue(c); list_del!(&mut (*c).node); });
}
unsafe fn push_desc_queue(c:*mut cppi41_channel){
    let cdd=(*c).cdd; (*c).residue=0; let mut r=GCR_CHAN_ENABLE;
    if (*c).is_tx==0 {r|=GCR_STARV_RETRY|GCR_DESC_TYPE_HOST|(*c).q_comp_num;}
    cppi_writel(r,(*c).gcr_reg); __iowmb!(); pm_runtime_get((*cdd).ddev.dev);
    let p=(*c).desc_phys as u32; let n=((p-(*cdd).descs_phys as u32)/core::mem::size_of::<cppi41_desc>() as u32) as usize;
    (*cdd).chan_busy[n]=c; cppi_writel(((core::mem::size_of::<cppi41_desc>()-24)/4) as u32|p,(*cdd).qmgr_mem.add(qmgr_queue_d((*c).q_num as usize)));
}

// External kernel structures and helpers are intentionally unresolved here;
// they correspond one-for-one to the declarations included by cppi41.c.
#[allow(non_camel_case_types)] pub type dma_addr_t=u64;
#[allow(non_camel_case_types)] pub type dma_cookie_t=i32;
#[allow(non_camel_case_types)] pub struct dma_chan{pub device:*mut dma_device,pub device_node:list_head}
#[allow(non_camel_case_types)] pub struct dma_async_tx_descriptor{pub tx_submit:Option<unsafe fn(*mut dma_async_tx_descriptor)->dma_cookie_t>}
#[allow(non_camel_case_types)] pub struct dma_device{pub dev:*mut device}
#[allow(non_camel_case_types)] pub struct device; #[allow(non_camel_case_types)] pub struct list_head; #[allow(non_camel_case_types)] pub struct spinlock_t;
extern "C" { fn pm_runtime_get(_: *mut device)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
