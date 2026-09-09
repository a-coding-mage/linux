/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * DMA header for AMD Queue-based DMA Subsystem
 *
 * Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
 */

/* Linux dependencies are supplied by the surrounding translation unit. */

pub const DISABLE: u32 = 0;
pub const ENABLE: u32 = 1;
pub const QDMA_MIN_IRQ: u32 = 3;
pub const QDMA_INTR_NAME_MAX_LEN: usize = 30;
pub const QDMA_INTR_PREFIX: &str = "amd-qdma";
pub const QDMA_IDENTIFIER: u32 = 0x1FD3;
pub const QDMA_DEFAULT_RING_SIZE: u32 = (1u32 << 10) + 1;
pub const QDMA_DEFAULT_RING_ID: u32 = 0;
pub const QDMA_POLL_INTRVL_US: u32 = 10; /* 10us */
pub const QDMA_POLL_TIMEOUT_US: u32 = 500 * 1000; /* 500ms */
pub const QDMA_DMAP_REG_STRIDE: u32 = 16;
pub const QDMA_CTXT_REGMAP_LEN: u32 = 8; /* 8 regs */
pub const QDMA_MM_DESC_SIZE: u32 = 32; /* Bytes */
pub const QDMA_MM_DESC_LEN_BITS: u32 = 28;
pub const QDMA_MM_DESC_MAX_LEN: u32 = (1u32 << QDMA_MM_DESC_LEN_BITS) - 1;
pub const QDMA_MIN_DMA_ALLOC_SIZE: u32 = 4096;
pub const QDMA_INTR_RING_SIZE: u32 = 1u32 << 13;
pub const QDMA_INTR_RING_IDX_MASK: u32 = 0x3ff;
pub const QDMA_INTR_RING_BASE: fn(u64) -> u64 = |addr| addr >> 12;

pub const QDMA_IDENTIFIER_REGOFF: u32 = 0x0;
pub const QDMA_IDENTIFIER_MASK: u32 = 0xffff0000;
pub const QDMA_QUEUE_ARM_BIT: u32 = 1u32 << 16;

#[macro_export]
macro_rules! qdma_err { ($qdev:expr, $($arg:tt)*) => { dev_err!(&$qdev.pdev.dev, $($arg)*); } }
#[macro_export]
macro_rules! qdma_dbg { ($qdev:expr, $($arg:tt)*) => { dev_dbg!(&$qdev.pdev.dev, $($arg)*); } }
#[macro_export]
macro_rules! qdma_info { ($qdev:expr, $($arg:tt)*) => { dev_info!(&$qdev.pdev.dev, $($arg)*); } }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qdma_reg_fields {
    QDMA_REGF_IRQ_ENABLE, QDMA_REGF_WBK_ENABLE, QDMA_REGF_WBI_CHECK,
    QDMA_REGF_IRQ_ARM, QDMA_REGF_IRQ_VEC, QDMA_REGF_IRQ_AGG,
    QDMA_REGF_WBI_INTVL_ENABLE, QDMA_REGF_MRKR_DISABLE, QDMA_REGF_QUEUE_ENABLE,
    QDMA_REGF_QUEUE_MODE, QDMA_REGF_DESC_BASE, QDMA_REGF_DESC_SIZE,
    QDMA_REGF_RING_ID, QDMA_REGF_CMD_INDX, QDMA_REGF_CMD_CMD, QDMA_REGF_CMD_TYPE,
    QDMA_REGF_CMD_BUSY, QDMA_REGF_QUEUE_COUNT, QDMA_REGF_QUEUE_MAX,
    QDMA_REGF_QUEUE_BASE, QDMA_REGF_FUNCTION_ID, QDMA_REGF_INTR_AGG_BASE,
    QDMA_REGF_INTR_VECTOR, QDMA_REGF_INTR_SIZE, QDMA_REGF_INTR_VALID,
    QDMA_REGF_INTR_COLOR, QDMA_REGF_INTR_FUNCTION_ID, QDMA_REGF_ERR_INT_FUNC,
    QDMA_REGF_ERR_INT_VEC, QDMA_REGF_ERR_INT_ARM, QDMA_REGF_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qdma_regs {
    QDMA_REGO_CTXT_DATA, QDMA_REGO_CTXT_CMD, QDMA_REGO_CTXT_MASK,
    QDMA_REGO_MM_H2C_CTRL, QDMA_REGO_MM_C2H_CTRL, QDMA_REGO_QUEUE_COUNT,
    QDMA_REGO_RING_SIZE, QDMA_REGO_H2C_PIDX, QDMA_REGO_C2H_PIDX,
    QDMA_REGO_INTR_CIDX, QDMA_REGO_FUNC_ID, QDMA_REGO_ERR_INT,
    QDMA_REGO_ERR_STAT, QDMA_REGO_MAX,
}

#[repr(C)]
pub struct qdma_reg_field { pub lsb: u16, pub msb: u16 }
#[repr(C)]
pub struct qdma_reg { pub off: u32, pub count: u32 }

#[macro_export]
macro_rules! QDMA_REGF { ($msb:expr, $lsb:expr) => { qdma_reg_field { lsb: $lsb, msb: $msb } } }
#[macro_export]
macro_rules! QDMA_REGO { ($off:expr, $count:expr) => { qdma_reg { off: $off, count: $count } } }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qdma_desc_size { QDMA_DESC_SIZE_8B, QDMA_DESC_SIZE_16B, QDMA_DESC_SIZE_32B, QDMA_DESC_SIZE_64B }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum qdma_queue_op_mode { QDMA_QUEUE_OP_STREAM, QDMA_QUEUE_OP_MM }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum qdma_ctxt_type {
    QDMA_CTXT_DESC_SW_C2H, QDMA_CTXT_DESC_SW_H2C, QDMA_CTXT_DESC_HW_C2H,
    QDMA_CTXT_DESC_HW_H2C, QDMA_CTXT_DESC_CR_C2H, QDMA_CTXT_DESC_CR_H2C,
    QDMA_CTXT_WRB, QDMA_CTXT_PFTCH, QDMA_CTXT_INTR_COAL, QDMA_CTXT_RSVD,
    QDMA_CTXT_HOST_PROFILE, QDMA_CTXT_TIMER, QDMA_CTXT_FMAP, QDMA_CTXT_FNC_STS,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub enum qdma_ctxt_cmd { QDMA_CTXT_CLEAR, QDMA_CTXT_WRITE, QDMA_CTXT_READ, QDMA_CTXT_INVALIDATE, QDMA_CTXT_MAX }

pub struct qdma_device;
pub struct qdma_queue;
pub struct virt_dma_desc;
pub struct virt_dma_chan;
pub struct scatterlist;
pub struct dma_slave_config;
pub struct dma_device;
pub struct platform_device;
pub struct regmap;
pub struct mutex;
pub type dma_addr_t = u64;
pub type __le32 = u32;
pub type __le64 = u64;

#[repr(C)]
pub struct qdma_ctxt_sw_desc { pub desc_base: u64, pub vec: u16 }
#[repr(C)]
pub struct qdma_ctxt_intr { pub agg_base: u64, pub vec: u16, pub size: u32, pub valid: bool, pub color: bool }
#[repr(C)]
pub struct qdma_ctxt_fmap { pub qbase: u16, pub qmax: u16 }

#[repr(C, packed)]
pub struct qdma_mm_desc { pub src_addr: __le64, pub len: __le32, pub reserved1: __le32, pub dst_addr: __le64, pub reserved2: __le64 }
#[repr(C)]
pub struct qdma_mm_vdesc { pub vdesc: virt_dma_desc, pub queue: *mut qdma_queue, pub sgl: *mut scatterlist, pub sg_off: u64, pub sg_len: u32, pub dev_addr: u64, pub pidx: u32, pub pending_descs: u32, pub cfg: dma_slave_config }

#[macro_export]
macro_rules! QDMA_VDESC_QUEUED { ($vdesc:expr) => { (*$vdesc).sg_len == 0 } }

#[repr(C)]
pub struct qdma_queue {
    pub qdev: *mut qdma_device, pub vchan: virt_dma_chan, pub dir: dma_transfer_direction,
    pub cfg: dma_slave_config, pub desc_base: *mut qdma_mm_desc, pub submitted_vdesc: *mut qdma_mm_vdesc,
    pub issued_vdesc: *mut qdma_mm_vdesc, pub dma_desc_base: dma_addr_t, pub pidx_reg: u32,
    pub cidx_reg: u32, pub ring_size: u32, pub idx_mask: u32, pub qid: u16, pub pidx: u32, pub cidx: u32,
}
#[repr(C)]
pub struct qdma_intr_ring {
    pub qdev: *mut qdma_device, pub base: *mut __le64, pub dev_base: dma_addr_t,
    pub msix_name: [u8; QDMA_INTR_NAME_MAX_LEN], pub msix_vector: u32, pub msix_id: u16,
    pub ring_size: u32, pub ridx: u16, pub cidx: u16, pub color: u8,
}

pub const QDMA_INTR_MASK_PIDX: u64 = 0xffff;
pub const QDMA_INTR_MASK_CIDX: u64 = 0xffff0000;
pub const QDMA_INTR_MASK_DESC_COLOR: u64 = 1u64 << 32;
pub const QDMA_INTR_MASK_STATE: u64 = 3u64 << 33;
pub const QDMA_INTR_MASK_ERROR: u64 = 3u64 << 35;
pub const QDMA_INTR_MASK_TYPE: u64 = 1u64 << 38;
pub const QDMA_INTR_MASK_QID: u64 = 0xffffffu64 << 39;
pub const QDMA_INTR_MASK_COLOR: u64 = 1u64 << 63;

#[repr(C)]
pub struct qdma_device {
    pub pdev: *mut platform_device, pub dma_dev: dma_device, pub regmap: *mut regmap,
    pub ctxt_lock: mutex, pub rfields: *const qdma_reg_field, pub roffs: *const qdma_reg,
    pub h2c_queues: *mut qdma_queue, pub c2h_queues: *mut qdma_queue, pub qintr_rings: *mut qdma_intr_ring,
    pub qintr_ring_num: u32, pub qintr_ring_idx: u32, pub chan_num: u32, pub queue_irq_start: u32,
    pub queue_irq_num: u32, pub err_irq_idx: u32, pub fid: u32,
}

extern "C" {
    pub static qdma_regos_default: [qdma_reg; QDMA_REGO_MAX as usize];
    pub static qdma_regfs_default: [qdma_reg_field; QDMA_REGF_MAX as usize];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
