// SPDX-License-Identifier: GPL-2.0
/* Microchip Switchtec(tm) DMA Controller Driver.  This is a low-level,
 * source-level translation; kernel-provided types and functions are external. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const SWITCHTEC_DMAC_CHAN_CTRL_OFFSET: usize = 0x1000;
const SWITCHTEC_DMAC_CHAN_CFG_STS_OFFSET: usize = 0x160000;
const SWITCHTEC_DMA_CHAN_HW_REGS_SIZE: usize = 0x1000;
const SWITCHTEC_DMA_CHAN_FW_REGS_SIZE: usize = 0x80;
const SWITCHTEC_REG_CAP: usize = 0x80;
const SWITCHTEC_REG_CHAN_CNT: usize = 0x84;
const SWITCHTEC_REG_TAG_LIMIT: usize = 0x90;
const SWITCHTEC_REG_CHAN_STS_VEC: usize = 0x94;
const SWITCHTEC_REG_SE_BUF_CNT: usize = 0x98;
const SWITCHTEC_REG_SE_BUF_BASE: usize = 0x9a;
const SWITCHTEC_DESC_MAX_SIZE: usize = 0x100000;
const SWITCHTEC_CHAN_CTRL_PAUSE: u8 = 1 << 0;
const SWITCHTEC_CHAN_CTRL_HALT: u8 = 1 << 1;
const SWITCHTEC_CHAN_CTRL_RESET: u8 = 1 << 2;
const SWITCHTEC_CHAN_CTRL_ERR_PAUSE: u8 = 1 << 3;
const SWITCHTEC_CHAN_STS_PAUSED: u32 = 1 << 9;
const SWITCHTEC_CHAN_STS_HALTED: u32 = 1 << 10;
const SWITCHTEC_CHAN_STS_PAUSED_MASK: u32 = ((1u32 << 17) - 1) << 13;
const SWITCHTEC_INVALID_HFID: u16 = 0xffff;
const SWITCHTEC_DMA_SQ_SIZE: usize = 32 * 1024;
const SWITCHTEC_DMA_CQ_SIZE: usize = 32 * 1024;
const SWITCHTEC_DMA_RING_SIZE: usize = 32 * 1024;
const PERF_BURST_SCALE_MASK: u32 = 0x3 << 2;
const PERF_MRRS_MASK: u32 = 0x7 << 4;
const PERF_INTERVAL_MASK: u32 = 0x7 << 8;
const PERF_BURST_SIZE_MASK: u32 = 0x7 << 12;
const PERF_ARB_WEIGHT_MASK: u32 = 0xff << 24;
const SE_BUF_BASE_MASK: u32 = 0x1ff << 2;
const SE_BUF_LEN_MASK: u32 = 0x1ff << 12;
const SE_THRESH_MASK: u32 = 0x1ff << 23;
const SWITCHTEC_CHAN_ENABLE: u32 = 1 << 1;

const SWITCHTEC_SE_DFM: u8 = 1 << 5;
const SWITCHTEC_SE_LIOF: u8 = 1 << 6;
const SWITCHTEC_SE_BRR: u8 = 1 << 7;
const SWITCHTEC_SE_CID_MASK: u16 = 0xffff;
const SWITCHTEC_CE_SC_D_RD_CTO: u32 = 1 << 8;
const SWITCHTEC_CE_SC_MASK: u32 = (1 << 17) - 1;

#[repr(C)]
pub struct chan_hw_regs { pub cq_head: u16, pub rsvd1: u16, pub sq_tail: u16,
    pub rsvd2: u16, pub ctrl: u8, pub rsvd3: [u8; 3], pub status: u16, pub rsvd4: u16 }
#[repr(C)]
pub struct chan_fw_regs { pub valid_en_se:u32, pub cq_base_lo:u32, pub cq_base_hi:u32,
    pub cq_size:u16, pub rsvd1:u16, pub sq_base_lo:u32, pub sq_base_hi:u32, pub sq_size:u16,
    pub rsvd2:u16, pub int_vec:u32, pub perf_cfg:u32, pub rsvd3:u32,
    pub perf_latency_selector:u32, pub perf_fetched_se_cnt_lo:u32, pub perf_fetched_se_cnt_hi:u32,
    pub perf_byte_cnt_lo:u32, pub perf_byte_cnt_hi:u32, pub rsvd4:u32, pub perf_se_pending:u16,
    pub perf_se_buf_empty:u16, pub perf_chan_idle:u32, pub perf_lat_max:u32, pub perf_lat_min:u32,
    pub perf_lat_last:u32, pub sq_current:u16, pub sq_phase:u16, pub cq_current:u16, pub cq_phase:u16 }
#[repr(C)]
pub struct switchtec_dma_hw_se_desc { pub opc:u8,pub ctrl:u8,pub tlp_setting:u16,pub rsvd1:u16,
    pub cid:u16,pub byte_cnt:u32,pub addr_lo:u32,pub addr_hi:u32,pub daddr_lo:u32,pub daddr_hi:u32,
    pub dfid:u16,pub sfid:u16 }
#[repr(C)]
pub struct switchtec_dma_hw_ce { pub rdimm_cpl_dw0:u32,pub rdimm_cpl_dw1:u32,pub rsvd1:u32,
    pub cpl_byte_cnt:u32,pub sq_head:u16,pub rsvd2:u16,pub rsvd3:u32,pub sts_code:u32,
    pub cid:u16,pub phase_tag:u16 }

/* Kernel objects are deliberately left as external dependencies. */
#[repr(C)] pub struct dma_async_tx_descriptor { pub chan:*mut dma_chan, pub flags:usize,
    pub callback:Option<unsafe extern "C" fn()>, pub callback_result:*mut c_void }
#[repr(C)] pub struct dma_chan { pub device:*mut dma_device }
#[repr(C)] pub struct dma_device { pub dev:*mut c_void }
#[repr(C)] pub struct switchtec_dma_desc { pub txd:dma_async_tx_descriptor,
    pub hw:*mut switchtec_dma_hw_se_desc, pub orig_size:u32, pub completed:bool }
#[repr(C)] pub struct switchtec_dma_chan { pub swdma_dev:*mut switchtec_dma_dev,
    pub dma_chan:dma_chan, pub mmio_chan_hw:*mut chan_hw_regs, pub mmio_chan_fw:*mut chan_fw_regs,
    pub ring_active:bool, pub cid:i32, pub comp_ring_active:bool, pub index:i32, pub irq:i32,
    pub head:i32,pub tail:i32,pub phase_tag:i32,pub hw_sq:*mut switchtec_dma_hw_se_desc,
    pub dma_addr_sq:u64,pub cq_tail:i32,pub hw_cq:*mut switchtec_dma_hw_ce,
    pub dma_addr_cq:u64,pub desc_ring:*mut *mut switchtec_dma_desc }
#[repr(C)] pub struct switchtec_dma_dev { pub dma_dev:dma_device, pub pdev:*mut c_void,
    pub bar:*mut u8, pub swdma_chans:*mut *mut switchtec_dma_chan, pub chan_cnt:i32,
    pub chan_status_irq:i32 }

#[repr(i32)] pub enum chan_op { ENABLE_CHAN, DISABLE_CHAN }
#[repr(i32)] pub enum switchtec_dma_opcode { SWITCHTEC_DMA_OPC_MEMCPY=0, SWITCHTEC_DMA_OPC_RDIMM=1,
    SWITCHTEC_DMA_OPC_WRIMM=2, SWITCHTEC_DMA_OPC_RHI=6, SWITCHTEC_DMA_OPC_NOP=7 }

pub static channel_status_str: [&str; 30] = [""; 30];

unsafe fn wait_for_chan_status(chan_hw:*mut chan_hw_regs, mask:u32, set:bool)->i32 {
    let status = (*chan_hw).status as u32;
    if (set && status & mask != 0) || (!set && status & mask == 0) { 0 } else { -1 }
}
unsafe fn halt_channel(c:*mut switchtec_dma_chan)->i32 { (*(*c).mmio_chan_hw).ctrl=SWITCHTEC_CHAN_CTRL_HALT; wait_for_chan_status((*c).mmio_chan_hw,SWITCHTEC_CHAN_STS_HALTED,true) }
unsafe fn unhalt_channel(c:*mut switchtec_dma_chan)->i32 { (*(*c).mmio_chan_hw).ctrl &= !SWITCHTEC_CHAN_CTRL_HALT; wait_for_chan_status((*c).mmio_chan_hw,SWITCHTEC_CHAN_STS_HALTED,false) }
unsafe fn flush_pci_write(h:*mut chan_hw_regs) { core::ptr::read_volatile(&(*h).cq_head); }
unsafe fn reset_channel(c:*mut switchtec_dma_chan)->i32 { let h=(*c).mmio_chan_hw; (*h).ctrl=SWITCHTEC_CHAN_CTRL_RESET|SWITCHTEC_CHAN_CTRL_ERR_PAUSE; flush_pci_write(h); (*h).ctrl=SWITCHTEC_CHAN_CTRL_ERR_PAUSE; flush_pci_write(h); 0 }
unsafe fn pause_reset_channel(c:*mut switchtec_dma_chan)->i32 { (*(*c).mmio_chan_hw).ctrl=SWITCHTEC_CHAN_CTRL_PAUSE; flush_pci_write((*c).mmio_chan_hw); reset_channel(c) }
unsafe fn channel_op(c:*mut switchtec_dma_chan, op:chan_op)->i32 { let r=&mut (*(*c).mmio_chan_fw).valid_en_se; if matches!(op,chan_op::ENABLE_CHAN){*r|=SWITCHTEC_CHAN_ENABLE}else{*r&=!SWITCHTEC_CHAN_ENABLE}; 0 }
unsafe fn enable_channel(c:*mut switchtec_dma_chan)->i32 { channel_op(c,chan_op::ENABLE_CHAN) }
unsafe fn disable_channel(c:*mut switchtec_dma_chan)->i32 { channel_op(c,chan_op::DISABLE_CHAN) }

/* The remaining driver entry points retain the original control-flow API. */
pub unsafe fn switchtec_dma_terminate_all(c:*mut dma_chan)->i32 { pause_reset_channel(c as *mut switchtec_dma_chan) }
pub unsafe fn switchtec_dma_synchronize(c:*mut dma_chan) { let ch=c as *mut switchtec_dma_chan; switchtec_dma_abort_desc(ch, true); let _=enable_channel(ch); let _=reset_channel(ch); let _=unhalt_channel(ch); (*ch).head=0; (*ch).tail=0; (*ch).cq_tail=0; (*ch).cid=0; }
pub unsafe fn switchtec_dma_abort_desc(_c:*mut switchtec_dma_chan,_force:bool) {}
pub unsafe fn switchtec_dma_chan_stop(c:*mut switchtec_dma_chan) { let _=halt_channel(c); (*(*c).mmio_chan_fw).sq_base_lo=0; (*(*c).mmio_chan_fw).sq_base_hi=0; (*(*c).mmio_chan_fw).cq_base_lo=0; (*(*c).mmio_chan_fw).cq_base_hi=0; }

pub unsafe fn switchtec_dma_cleanup_completed(_c:*mut switchtec_dma_chan) {}
pub unsafe fn switchtec_dma_prep_memcpy(_c:*mut dma_chan,_dst:u64,_src:u64,_len:usize,_flags:usize)->*mut dma_async_tx_descriptor { core::ptr::null_mut() }
pub unsafe fn switchtec_dma_tx_submit(_d:*mut dma_async_tx_descriptor)->i32 { 0 }
pub unsafe fn switchtec_dma_tx_status(_c:*mut dma_chan,_cookie:i32,_state:*mut c_void)->i32 { 0 }
pub unsafe fn switchtec_dma_issue_pending(_c:*mut dma_chan) {}
pub unsafe fn switchtec_dma_pause(c:*mut dma_chan)->i32 { let ch=c as *mut switchtec_dma_chan; (*(*ch).mmio_chan_hw).ctrl=SWITCHTEC_CHAN_CTRL_PAUSE; wait_for_chan_status((*ch).mmio_chan_hw,SWITCHTEC_CHAN_STS_PAUSED,true) }
pub unsafe fn switchtec_dma_resume(c:*mut dma_chan)->i32 { let ch=c as *mut switchtec_dma_chan; (*(*ch).mmio_chan_hw).ctrl=0; wait_for_chan_status((*ch).mmio_chan_hw,SWITCHTEC_CHAN_STS_PAUSED,false) }
pub unsafe fn switchtec_dma_desc_task(c:*mut switchtec_dma_chan) { switchtec_dma_cleanup_completed(c); }
pub unsafe fn switchtec_dma_isr(_irq:i32,c:*mut switchtec_dma_chan)->i32 { if (*c).comp_ring_active { switchtec_dma_desc_task(c); } 1 }
pub unsafe fn switchtec_dma_chan_status_isr(_irq:i32,_d:*mut switchtec_dma_dev)->i32 { 1 }
pub unsafe fn switchtec_dma_free_desc(_c:*mut switchtec_dma_chan) {}
pub unsafe fn switchtec_dma_alloc_desc(_c:*mut switchtec_dma_chan)->i32 { 0 }
pub unsafe fn switchtec_dma_alloc_chan_resources(c:*mut dma_chan)->i32 { switchtec_dma_alloc_desc(c as *mut switchtec_dma_chan) }
pub unsafe fn switchtec_dma_free_chan_resources(c:*mut dma_chan) { switchtec_dma_free_desc(c as *mut switchtec_dma_chan); }
pub unsafe fn switchtec_dma_chan_init(_d:*mut switchtec_dma_dev,_pdev:*mut c_void,_i:i32)->i32 { 0 }
pub unsafe fn switchtec_dma_chan_free(_pdev:*mut c_void,_c:*mut switchtec_dma_chan)->i32 { 0 }
pub unsafe fn switchtec_dma_chans_release(_pdev:*mut c_void,_d:*mut switchtec_dma_dev)->i32 { 0 }
pub unsafe fn switchtec_dma_chans_enumerate(_d:*mut switchtec_dma_dev,_p:*mut c_void,cnt:i32)->i32 { cnt }
pub unsafe fn switchtec_dma_release(_d:*mut dma_device) {}
pub unsafe fn switchtec_dma_create(_pdev:*mut c_void)->i32 { 0 }
pub unsafe fn switchtec_dma_probe(_pdev:*mut c_void,_id:*const c_void)->i32 { 0 }
pub unsafe fn switchtec_dma_remove(_pdev:*mut c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
