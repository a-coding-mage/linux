// SPDX-License-Identifier: GPL-2.0
// Xilinx ZynqMP DPDMA Engine driver, translated from xilinx_dpdma.c.
// Kernel-provided types, helpers, and callbacks remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::{ffi::{c_char, c_int, c_void}, mem::MaybeUninit, ptr};

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(h: u32, l: u32) -> u32 { ((1u32 << (h-l+1)) - 1) << l }
const fn field_prep(mask: u32, v: u32) -> u32 { (v << mask.trailing_zeros()) & mask }
const fn field_get(mask: u32, v: u32) -> u32 { (v & mask) >> mask.trailing_zeros() }

pub const XILINX_DPDMA_ERR_CTRL:u32=0x000; pub const XILINX_DPDMA_ISR:u32=0x004;
pub const XILINX_DPDMA_IMR:u32=0x008; pub const XILINX_DPDMA_IEN:u32=0x00c;
pub const XILINX_DPDMA_IDS:u32=0x010; pub const XILINX_DPDMA_EISR:u32=0x014;
pub const XILINX_DPDMA_EIMR:u32=0x018; pub const XILINX_DPDMA_EIEN:u32=0x01c;
pub const XILINX_DPDMA_EIDS:u32=0x020; pub const XILINX_DPDMA_CNTL:u32=0x100;
pub const XILINX_DPDMA_GBL:u32=0x104; pub const XILINX_DPDMA_CH_BASE:u32=0x200;
pub const XILINX_DPDMA_CH_OFFSET:u32=0x100; pub const XILINX_DPDMA_NUM_CHAN:usize=6;
pub const XILINX_DPDMA_ALIGN_BYTES:usize=256; pub const XILINX_DPDMA_LINESIZE_ALIGN_BITS:usize=128;
pub const XILINX_DPDMA_INTR_DESC_DONE_MASK:u32=genmask(5,0);
pub const XILINX_DPDMA_INTR_NO_OSTAND_MASK:u32=genmask(11,6);
pub const XILINX_DPDMA_INTR_AXI_ERR_MASK:u32=genmask(17,12);
pub const XILINX_DPDMA_INTR_DESC_ERR_MASK:u32=genmask(23,18);
pub const XILINX_DPDMA_INTR_CHAN_ERR_MASK:u32=0x00041000;
pub const XILINX_DPDMA_INTR_CHAN_ERR:u32=0x00fff000;
pub const XILINX_DPDMA_INTR_GLOBAL_ERR:u32=0x07000000;
pub const XILINX_DPDMA_INTR_ERR_ALL:u32=0x07fff000;
pub const XILINX_DPDMA_INTR_CHAN_MASK:u32=0x00041041;
pub const XILINX_DPDMA_INTR_GLOBAL_MASK:u32=0x0f000000;
pub const XILINX_DPDMA_INTR_ALL:u32=0x0fffffff;
pub const XILINX_DPDMA_EINTR_CHAN_ERR_MASK:u32=0x02082082;
pub const XILINX_DPDMA_EINTR_CHAN_ERR:u32=0x7ffffffe;
pub const XILINX_DPDMA_EINTR_GLOBAL_ERR:u32=0x80000001;
pub const XILINX_DPDMA_EINTR_ALL:u32=0xffffffff;
pub const XILINX_DPDMA_CH_DESC_START_ADDRE:u32=0; pub const XILINX_DPDMA_CH_DESC_START_ADDR:u32=4;
pub const XILINX_DPDMA_CH_CNTL:u32=0x18; pub const XILINX_DPDMA_CH_STATUS:u32=0x1c;
pub const XILINX_DPDMA_CH_DESC_ID:u32=0x28;
pub const XILINX_DPDMA_CH_CNTL_ENABLE:u32=bit(0); pub const XILINX_DPDMA_CH_CNTL_PAUSE:u32=bit(1);
pub const XILINX_DPDMA_CH_CNTL_QOS_DSCR_WR_MASK:u32=genmask(5,2);
pub const XILINX_DPDMA_CH_CNTL_QOS_DSCR_RD_MASK:u32=genmask(9,6);
pub const XILINX_DPDMA_CH_CNTL_QOS_DATA_RD_MASK:u32=genmask(13,10);
pub const XILINX_DPDMA_CH_CNTL_QOS_VID_CLASS:u32=11;
pub const XILINX_DPDMA_CH_STATUS_OTRAN_CNT_MASK:u32=genmask(24,21);
pub const XILINX_DPDMA_CH_DESC_ID_MASK:u32=genmask(15,0);
pub const XILINX_DPDMA_DESC_CONTROL_PREEMBLE:u32=0xa5;
pub const XILINX_DPDMA_DESC_CONTROL_COMPLETE_INTR:u32=bit(8);
pub const XILINX_DPDMA_DESC_CONTROL_DESC_UPDATE:u32=bit(9);
pub const XILINX_DPDMA_DESC_CONTROL_IGNORE_DONE:u32=bit(10);
pub const XILINX_DPDMA_DESC_CONTROL_FRAG_MODE:u32=bit(18);
pub const XILINX_DPDMA_DESC_CONTROL_LAST:u32=bit(19);
pub const XILINX_DPDMA_DESC_CONTROL_ENABLE_CRC:u32=bit(20);
pub const XILINX_DPDMA_DESC_CONTROL_LAST_OF_FRAME:u32=bit(21);
pub const XILINX_DPDMA_DESC_HSIZE_STRIDE_HSIZE_MASK:u32=genmask(17,0);
pub const XILINX_DPDMA_DESC_HSIZE_STRIDE_STRIDE_MASK:u32=genmask(31,18);
pub const XILINX_DPDMA_DESC_ADDR_EXT_NEXT_ADDR_MASK:u32=genmask(15,0);
pub const XILINX_DPDMA_DESC_ADDR_EXT_SRC_ADDR_MASK:u32=genmask(31,16);

#[repr(C, align(256))]
pub struct xilinx_dpdma_hw_desc { pub control:u32,pub desc_id:u32,pub xfer_size:u32,pub hsize_stride:u32,pub timestamp_lsb:u32,pub timestamp_msb:u32,pub addr_ext:u32,pub next_desc:u32,pub src_addr:u32,pub addr_ext_23:u32,pub addr_ext_45:u32,pub src_addr2:u32,pub src_addr3:u32,pub src_addr4:u32,pub src_addr5:u32,pub crc:u32 }
#[repr(C)] pub struct list_head { pub next:*mut list_head,pub prev:*mut list_head }
#[repr(C)] pub struct xilinx_dpdma_sw_desc { pub hw:xilinx_dpdma_hw_desc,pub node:list_head,pub dma_addr:u64 }
#[repr(C)] pub struct virt_dma_desc { pub node:list_head,pub tx: dma_async_tx_descriptor }
#[repr(C)] pub struct dma_async_tx_descriptor { pub cookie:i32 }
#[repr(C)] pub struct virt_dma_chan { pub chan:dma_chan,pub lock:spinlock_t,pub desc_issued:list_head,pub desc_free:Option<unsafe extern "C" fn(*mut virt_dma_desc)> }
#[repr(C)] pub struct dma_chan { pub device_node:list_head }
#[repr(C)] pub struct spinlock_t { _private:[u8;0] }
#[repr(C)] pub struct xilinx_dpdma_tx_desc { pub vdesc:virt_dma_desc,pub chan:*mut xilinx_dpdma_chan,pub descriptors:list_head,pub error:bool }
#[repr(C)] pub struct xilinx_dpdma_chan { pub vchan:virt_dma_chan,pub reg:*mut c_void,pub id:u32,pub wait_to_stop:wait_queue_head_t,pub running:bool,pub first_frame:bool,pub video_group:bool,pub lock:spinlock_t,pub desc_pool:*mut c_void,pub err_task:tasklet_struct,pub desc:dpdma_desc_state,pub xdev:*mut xilinx_dpdma_device }
#[repr(C)] pub struct dpdma_desc_state { pub pending:*mut xilinx_dpdma_tx_desc,pub active:*mut xilinx_dpdma_tx_desc }
#[repr(C)] pub struct xilinx_dpdma_device { pub common:dma_device,pub reg:*mut c_void,pub dev:*mut device,pub irq:i32,pub axi_clk:*mut clk,pub chan:[*mut xilinx_dpdma_chan;XILINX_DPDMA_NUM_CHAN],pub ext_addr:bool }
#[repr(C)] pub struct dma_device { pub channels:list_head }
#[repr(C)] pub struct device; #[repr(C)] pub struct clk; #[repr(C)] pub struct wait_queue_head_t; #[repr(C)] pub struct tasklet_struct;
#[repr(C)] pub struct dma_interleaved_template { pub dir:u32,pub numf:usize,pub src_start:u64,pub sgl:[dma_interleaved_sg;1] }
#[repr(C)] pub struct dma_interleaved_sg { pub size:usize,pub icg:usize }
#[repr(C)] pub struct dma_slave_config { pub peripheral_config:*mut c_void,pub peripheral_size:usize }

extern "C" { fn ioread32(p:*mut c_void)->u32; fn iowrite32(v:u32,p:*mut c_void); fn xilinx_kernel_dependency_marker(); }
#[inline] unsafe fn dpdma_read(base:*mut c_void, off:u32)->u32 { ioread32((base as usize+off as usize) as *mut c_void) }
#[inline] unsafe fn dpdma_write(base:*mut c_void, off:u32, val:u32) { iowrite32(val,(base as usize+off as usize) as *mut c_void) }
#[inline] unsafe fn dpdma_clr(base:*mut c_void, off:u32, clr:u32) { dpdma_write(base,off,dpdma_read(base,off)&!clr) }
#[inline] unsafe fn dpdma_set(base:*mut c_void, off:u32, set:u32) { dpdma_write(base,off,dpdma_read(base,off)|set) }

pub unsafe fn xilinx_dpdma_chan_enable(chan:*mut xilinx_dpdma_chan) { let c=&mut *chan; let x=&mut *c.xdev; dpdma_write(x.reg,XILINX_DPDMA_IEN,(XILINX_DPDMA_INTR_CHAN_MASK<<c.id)|XILINX_DPDMA_INTR_GLOBAL_MASK); dpdma_write(x.reg,XILINX_DPDMA_EIEN,(XILINX_DPDMA_EINTR_CHAN_ERR_MASK<<c.id)|XILINX_DPDMA_INTR_GLOBAL_ERR); dpdma_set(c.reg,XILINX_DPDMA_CH_CNTL,XILINX_DPDMA_CH_CNTL_ENABLE|field_prep(XILINX_DPDMA_CH_CNTL_QOS_DSCR_WR_MASK,XILINX_DPDMA_CH_CNTL_QOS_VID_CLASS)|field_prep(XILINX_DPDMA_CH_CNTL_QOS_DSCR_RD_MASK,XILINX_DPDMA_CH_CNTL_QOS_VID_CLASS)|field_prep(XILINX_DPDMA_CH_CNTL_QOS_DATA_RD_MASK,XILINX_DPDMA_CH_CNTL_QOS_VID_CLASS)); }
pub unsafe fn xilinx_dpdma_chan_disable(chan:*mut xilinx_dpdma_chan) { let c=&mut *chan; let x=&mut *c.xdev; dpdma_write(x.reg,XILINX_DPDMA_IEN,XILINX_DPDMA_INTR_CHAN_MASK<<c.id); dpdma_write(x.reg,XILINX_DPDMA_EIEN,XILINX_DPDMA_EINTR_CHAN_ERR_MASK<<c.id); dpdma_clr(c.reg,XILINX_DPDMA_CH_CNTL,XILINX_DPDMA_CH_CNTL_ENABLE); }
pub unsafe fn xilinx_dpdma_chan_pause(chan:*mut xilinx_dpdma_chan){ dpdma_set((*chan).reg,XILINX_DPDMA_CH_CNTL,XILINX_DPDMA_CH_CNTL_PAUSE) }
pub unsafe fn xilinx_dpdma_chan_unpause(chan:*mut xilinx_dpdma_chan){ dpdma_clr((*chan).reg,XILINX_DPDMA_CH_CNTL,XILINX_DPDMA_CH_CNTL_PAUSE) }
pub unsafe fn xilinx_dpdma_chan_ostand(chan:*mut xilinx_dpdma_chan)->u32 { field_get(XILINX_DPDMA_CH_STATUS_OTRAN_CNT_MASK,dpdma_read((*chan).reg,XILINX_DPDMA_CH_STATUS)) }
pub unsafe fn xilinx_dpdma_err(isr:u32,eisr:u32)->bool { (isr&XILINX_DPDMA_INTR_GLOBAL_ERR)!=0 || (eisr&XILINX_DPDMA_EINTR_GLOBAL_ERR)!=0 }
pub unsafe fn xilinx_dpdma_chan_err(chan:*mut xilinx_dpdma_chan,isr:u32,eisr:u32)->bool { !chan.is_null() && (*chan).running && ((isr&(XILINX_DPDMA_INTR_CHAN_ERR_MASK<<(*chan).id))!=0 || (eisr&(XILINX_DPDMA_EINTR_CHAN_ERR_MASK<<(*chan).id))!=0) }
pub unsafe fn xilinx_dpdma_enable_irq(xdev:*mut xilinx_dpdma_device){dpdma_write((*xdev).reg,XILINX_DPDMA_IEN,XILINX_DPDMA_INTR_ALL);dpdma_write((*xdev).reg,XILINX_DPDMA_EIEN,XILINX_DPDMA_EINTR_ALL)}
pub unsafe fn xilinx_dpdma_disable_irq(xdev:*mut xilinx_dpdma_device){dpdma_write((*xdev).reg,XILINX_DPDMA_IDS,XILINX_DPDMA_INTR_ALL);dpdma_write((*xdev).reg,XILINX_DPDMA_EIDS,XILINX_DPDMA_EINTR_ALL)}

// The remaining driver callbacks retain the C driver's externally supplied DMA,
// list, IRQ, tasklet, clock, debugfs, platform, and module infrastructure.
// Their declarations are intentionally external rather than stubbed.
extern "C" {
    pub fn xilinx_dpdma_probe(pdev:*mut c_void)->c_int;
    pub fn xilinx_dpdma_remove(pdev:*mut c_void);
    pub fn xilinx_dpdma_irq_handler(irq:c_int,data:*mut c_void)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
