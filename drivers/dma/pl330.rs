// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of the PL330 DMA implementation.
// Kernel-provided types, functions, macros, and device operations remain
// external dependencies, as they are in the original implementation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const PL330_MAX_CHAN: usize = 8;
pub const PL330_MAX_IRQS: usize = 32;
pub const PL330_MAX_PERI: usize = 32;
pub const PL330_MAX_BURST: u32 = 16;
pub const PL330_QUIRK_BROKEN_NO_FLUSHP: i32 = 1 << 0;
pub const PL330_QUIRK_PERIPH_BURST: i32 = 1 << 1;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum pl330_cachectrl { CCTRL0, CCTRL1, CCTRL2, CCTRL3, INVALID1, INVALID2, CCTRL6, CCTRL7 }
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum pl330_byteswap { SWAP_NO, SWAP_2, SWAP_4, SWAP_8, SWAP_16 }
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum pl330_op_err { PL330_ERR_NONE, PL330_ERR_ABORT, PL330_ERR_FAIL }
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum dmamov_dst { SAR = 0, CCR, DAR }
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum pl330_dst { SRC = 0, DST }
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum pl330_cond { SINGLE, BURST, ALWAYS }
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum pl330_dmac_state { UNINIT, INIT, DYING }
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum desc_status { FREE, PREP, BUSY, PAUSED, DONE }

pub const DS: u32 = 0x0; pub const DPC: u32 = 0x4; pub const INTEN: u32 = 0x20;
pub const ES: u32 = 0x24; pub const INTSTATUS: u32 = 0x28; pub const INTCLR: u32 = 0x2c;
pub const FSM: u32 = 0x30; pub const FSC: u32 = 0x34; pub const FTM: u32 = 0x38;
pub const DBGSTATUS: u32 = 0xd00; pub const DBG_BUSY: u32 = 1;
pub const DBGCMD: u32 = 0xd04; pub const DBGINST0: u32 = 0xd08; pub const DBGINST1: u32 = 0xd0c;
pub const CR0: u32 = 0xe00; pub const CR1: u32 = 0xe04; pub const CR2: u32 = 0xe08;
pub const CR3: u32 = 0xe0c; pub const CR4: u32 = 0xe10; pub const CRD: u32 = 0xe14;
pub const PERIPH_ID: u32 = 0xfe0; pub const PART: u32 = 0x330; pub const DESIGNER: u32 = 0x41;
pub const PERIPH_ID_VAL: u32 = (PART << 0) | (DESIGNER << 12);
pub const MCODE_BUFF_PER_REQ: usize = 256;
pub const NR_DEFAULT_DESC: usize = 16;
pub const PL330_AUTOSUSPEND_DELAY: u32 = 20;

pub const CMD_DMAADDH:u8=0x54; pub const CMD_DMAEND:u8=0; pub const CMD_DMAFLUSHP:u8=0x35;
pub const CMD_DMAGO:u8=0xa0; pub const CMD_DMALD:u8=4; pub const CMD_DMALDP:u8=0x25;
pub const CMD_DMALP:u8=0x20; pub const CMD_DMALPEND:u8=0x28; pub const CMD_DMAKILL:u8=1;
pub const CMD_DMAMOV:u8=0xbc; pub const CMD_DMANOP:u8=0x18; pub const CMD_DMARMB:u8=0x12;
pub const CMD_DMASEV:u8=0x34; pub const CMD_DMAST:u8=8; pub const CMD_DMASTP:u8=0x29;
pub const CMD_DMASTZ:u8=0x0c; pub const CMD_DMAWFE:u8=0x36; pub const CMD_DMAWFP:u8=0x30;
pub const CMD_DMAWMB:u8=0x13;

#[repr(C)]
pub struct pl330_config { pub periph_id:u32, pub mode:u32, pub data_bus_width:u32, pub data_buf_dep:u32, pub num_chan:u32, pub num_peri:u32, pub peri_ns:u32, pub num_events:u32, pub irq_ns:u32 }
#[repr(C)]
pub struct pl330_reqcfg { pub dst_inc:u8, pub src_inc:u8, pub nonsecure:bool, pub privileged:bool, pub insnaccess:bool, pub brst_len:u8, pub brst_size:u8, pub dcctl:pl330_cachectrl, pub scctl:pl330_cachectrl, pub swap:pl330_byteswap, pub pcfg:*mut pl330_config }
#[repr(C)] pub struct pl330_xfer { pub src_addr:u32, pub dst_addr:u32, pub bytes:u32 }
#[repr(C)] pub struct _pl330_req { pub mc_bus:usize, pub mc_cpu:*mut u8, pub desc:*mut dma_pl330_desc }
#[repr(C)] pub struct _pl330_tbd { pub reset_dmac:bool, pub reset_mngr:bool, pub reset_chan:u8 }
#[repr(C)] pub struct pl330_thread { pub id:u8, pub ev:i32, pub free:bool, pub dmac:*mut pl330_dmac, pub req:[_pl330_req;2], pub lstenq:usize, pub req_running:i32 }
#[repr(C)] pub struct _xfer_spec { pub ccr:u32, pub desc:*mut dma_pl330_desc }
#[repr(C)] pub struct dma_pl330_desc { pub px:pl330_xfer, pub rqcfg:pl330_reqcfg, pub status:desc_status, pub bytes_requested:i32, pub last:bool, pub pchan:*mut dma_pl330_chan, pub peri:u8 }
#[repr(C)] pub struct dma_pl330_chan { pub dmac:*mut pl330_dmac, pub thread:*mut pl330_thread, pub burst_sz:i32, pub burst_len:i32, pub fifo_addr:usize, pub fifo_dma:usize, pub cyclic:bool, pub active:bool }
#[repr(C)] pub struct pl330_dmac { pub mcbufsz:usize, pub base:*mut u8, pub pcfg:pl330_config, pub lock:usize, pub events:[i32;32], pub mcode_bus:usize, pub mcode_cpu:*mut u8, pub channels:*mut pl330_thread, pub manager:*mut pl330_thread, pub dmac_tbd:_pl330_tbd, pub state:pl330_dmac_state, pub num_peripherals:u32, pub peripherals:*mut dma_pl330_chan, pub quirks:i32 }

pub const CC_SRCBRSTLEN_SHFT:u32=4; pub const CC_DSTBRSTLEN_SHFT:u32=18; pub const CC_SRCBRSTSIZE_SHFT:u32=1; pub const CC_DSTBRSTSIZE_SHFT:u32=15;
#[inline] pub fn brst_len(ccr:u32)->u32 { ((ccr>>CC_SRCBRSTLEN_SHFT)&0xf)+1 }
#[inline] pub fn brst_size(ccr:u32)->u32 { 1<<((ccr>>CC_SRCBRSTSIZE_SHFT)&7) }
#[inline] pub fn byte_to_burst(b:u32,ccr:u32)->u32 { b/brst_size(ccr)/brst_len(ccr) }
#[inline] pub fn burst_to_byte(c:u32,ccr:u32)->u32 { c*brst_size(ccr)*brst_len(ccr) }

// The remaining driver entry points retain the original externally visible
// interface and are supplied by the surrounding kernel translation layer.
extern "C" {
    pub fn pl330_probe(adev:*mut core::ffi::c_void, id:*const core::ffi::c_void)->i32;
    pub fn pl330_remove(adev:*mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
