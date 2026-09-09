/* SPDX-License-Identifier: GPL-2.0-or-later */
/* DMA driver header for STMicroelectronics STi FDMA controller */

// External kernel dependencies supplied by the surrounding translation.

pub const ST_FDMA_NR_DREQS: u32 = 32;
pub const FW_NAME_SIZE: usize = 30;
pub const DRIVER_NAME: &str = "st-fdma";

#[repr(C)]
pub struct st_fdma_generic_node {
    pub length: u32,
    pub sstride: u32,
    pub dstride: u32,
}

#[repr(C, align(32))]
pub struct st_fdma_hw_node {
    pub next: u32,
    pub control: u32,
    pub nbytes: u32,
    pub saddr: u32,
    pub daddr: u32,
    pub generic: st_fdma_generic_node,
}

pub const FDMA_NODE_CTRL_REQ_MAP_MASK: u32 = 0x1f;
pub const FDMA_NODE_CTRL_REQ_MAP_FREE_RUN: u32 = 0x0;
#[inline]
pub const fn FDMA_NODE_CTRL_REQ_MAP_DREQ(n: u32) -> u32 { n & FDMA_NODE_CTRL_REQ_MAP_MASK }
pub const FDMA_NODE_CTRL_REQ_MAP_EXT: u32 = FDMA_NODE_CTRL_REQ_MAP_MASK;
pub const FDMA_NODE_CTRL_SRC_MASK: u32 = 0x60;
pub const FDMA_NODE_CTRL_SRC_STATIC: u32 = 1 << 5;
pub const FDMA_NODE_CTRL_SRC_INCR: u32 = 1 << 6;
pub const FDMA_NODE_CTRL_DST_MASK: u32 = 0x180;
pub const FDMA_NODE_CTRL_DST_STATIC: u32 = 1 << 7;
pub const FDMA_NODE_CTRL_DST_INCR: u32 = 1 << 8;
pub const FDMA_NODE_CTRL_SECURE: u32 = 1 << 15;
pub const FDMA_NODE_CTRL_PAUSE_EON: u32 = 1 << 30;
pub const FDMA_NODE_CTRL_INT_EON: u32 = 1 << 31;

#[repr(C)]
pub struct st_fdma_sw_node {
    pub pdesc: dma_addr_t,
    pub desc: *mut st_fdma_hw_node,
}

pub const NAME_SZ: usize = 10;

#[repr(C)]
pub struct st_fdma_driverdata {
    pub id: u32,
    pub name: [core::ffi::c_char; NAME_SZ],
}

#[repr(C)]
pub struct st_fdma_desc {
    pub vdesc: virt_dma_desc,
    pub fchan: *mut st_fdma_chan,
    pub iscyclic: bool,
    pub n_nodes: core::ffi::c_uint,
    pub node: [st_fdma_sw_node; 0], // __counted_by(n_nodes)
}

#[repr(C)]
pub enum st_fdma_type {
    ST_FDMA_TYPE_FREE_RUN,
    ST_FDMA_TYPE_PACED,
}

#[repr(C)]
pub struct st_fdma_cfg {
    pub of_node: *mut device_node,
    pub type_: st_fdma_type,
    pub dev_addr: dma_addr_t,
    pub dir: dma_transfer_direction,
    pub req_line: i32,
    pub req_ctrl: i64,
}

#[repr(C)]
pub struct st_fdma_chan {
    pub fdev: *mut st_fdma_dev,
    pub node_pool: *mut dma_pool,
    pub scfg: dma_slave_config,
    pub cfg: st_fdma_cfg,
    pub dreq_line: i64,
    pub vchan: virt_dma_chan,
    pub fdesc: *mut st_fdma_desc,
    pub status: dma_status,
}

#[repr(C)]
pub struct st_fdma_dev {
    pub dev: *mut device,
    pub drvdata: *const st_fdma_driverdata,
    pub dma_device: dma_device,
    pub slim_rproc: *mut st_slim_rproc,
    pub irq: i32,
    pub chans: *mut st_fdma_chan,
    pub dreq_lock: spinlock_t,
    pub dreq_mask: c_ulong,
    pub nr_channels: u32,
    pub fw_name: [core::ffi::c_char; FW_NAME_SIZE],
}

pub const FDMA_CMD_STA_OFST: u32 = 0xfc0;
pub const FDMA_CMD_SET_OFST: u32 = 0xfc4;
pub const FDMA_CMD_CLR_OFST: u32 = 0xfc8;
pub const FDMA_CMD_MASK_OFST: u32 = 0xfcc;
#[inline] pub const fn FDMA_CMD_START(ch: u32) -> u32 { 0x1 << (ch << 1) }
#[inline] pub const fn FDMA_CMD_PAUSE(ch: u32) -> u32 { 0x2 << (ch << 1) }
#[inline] pub const fn FDMA_CMD_FLUSH(ch: u32) -> u32 { 0x3 << (ch << 1) }
pub const FDMA_INT_STA_OFST: u32 = 0xfd0;
pub const FDMA_INT_STA_CH: u32 = 0x1;
pub const FDMA_INT_STA_ERR: u32 = 0x2;
pub const FDMA_INT_SET_OFST: u32 = 0xfd4;
pub const FDMA_INT_CLR_OFST: u32 = 0xfd8;
pub const FDMA_INT_MASK_OFST: u32 = 0xfdc;

macro_rules! fdma_read { ($fdev:expr, $name:expr) => { readl((*$fdev).slim_rproc.peri.add($name as usize)) }; }
macro_rules! fdma_write { ($fdev:expr, $val:expr, $name:expr) => { writel($val, (*$fdev).slim_rproc.peri.add($name as usize)) }; }

pub const FDMA_CH_CMD_OFST: u32 = 0x200;
pub const FDMA_CH_CMD_STA_MASK: u32 = 0x3;
pub const FDMA_CH_CMD_STA_IDLE: u32 = 0x0;
pub const FDMA_CH_CMD_STA_START: u32 = 0x1;
pub const FDMA_CH_CMD_STA_RUNNING: u32 = 0x2;
pub const FDMA_CH_CMD_STA_PAUSED: u32 = 0x3;
pub const FDMA_CH_CMD_ERR_MASK: u32 = 0x1c;
pub const FDMA_CH_CMD_ERR_INT: u32 = 0x0 << 2;
pub const FDMA_CH_CMD_ERR_NAND: u32 = 0x1 << 2;
pub const FDMA_CH_CMD_ERR_MCHI: u32 = 0x2 << 2;
pub const FDMA_CH_CMD_DATA_MASK: u32 = 0xffffffe0;

pub const FDMA_REQ_CTRL_OFST: u32 = 0x240;
pub const FDMA_NODE_SZ: usize = 128;
pub const FDMA_PTRN_OFST: u32 = 0x800;
pub const FDMA_CNTN_OFST: u32 = 0x808;
pub const FDMA_SADDRN_OFST: u32 = 0x80c;
pub const FDMA_DADDRN_OFST: u32 = 0x810;

pub const FDMA_REQ_CTRL_NUM_OPS_MASK: u32 = 0xff000000;
#[inline] pub const fn FDMA_REQ_CTRL_NUM_OPS(n: u32) -> u32 { FDMA_REQ_CTRL_NUM_OPS_MASK & (n << 24) }
pub const FDMA_REQ_CTRL_INITIATOR_MASK: u32 = 1 << 22;
pub const FDMA_REQ_CTRL_INIT0: u32 = 0;
pub const FDMA_REQ_CTRL_INIT1: u32 = 1 << 22;
pub const FDMA_REQ_CTRL_INC_ADDR_ON: u32 = 1 << 21;
pub const FDMA_REQ_CTRL_DATA_SWAP_ON: u32 = 1 << 17;
pub const FDMA_REQ_CTRL_WNR: u32 = 1 << 14;
pub const FDMA_REQ_CTRL_OPCODE_MASK: u32 = 0xf0;
pub const FDMA_REQ_CTRL_OPCODE_LD_ST1: u32 = 0x0 << 4;
pub const FDMA_REQ_CTRL_OPCODE_LD_ST2: u32 = 0x1 << 4;
pub const FDMA_REQ_CTRL_OPCODE_LD_ST4: u32 = 0x2 << 4;
pub const FDMA_REQ_CTRL_OPCODE_LD_ST8: u32 = 0x3 << 4;
pub const FDMA_REQ_CTRL_OPCODE_LD_ST16: u32 = 0x4 << 4;
pub const FDMA_REQ_CTRL_OPCODE_LD_ST32: u32 = 0x5 << 4;
pub const FDMA_REQ_CTRL_OPCODE_LD_ST64: u32 = 0x6 << 4;
pub const FDMA_REQ_CTRL_HOLDOFF_MASK: u32 = 0x7;
#[inline] pub const fn FDMA_REQ_CTRL_HOLDOFF(n: u32) -> u32 { n & FDMA_REQ_CTRL_HOLDOFF_MASK }
pub const FDMA_REQ_CTRL_CFG_MASK: u32 = FDMA_REQ_CTRL_HOLDOFF_MASK | FDMA_REQ_CTRL_DATA_SWAP_ON | FDMA_REQ_CTRL_INC_ADDR_ON | FDMA_REQ_CTRL_INITIATOR_MASK;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
