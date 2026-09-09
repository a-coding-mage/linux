/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2017-2020 Xilinx, Inc. All rights reserved.
 * Copyright (C) 2022, Advanced Micro Devices, Inc.
 */

// The original header guard was: __DMA_XDMA_REGS_H

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(high: u32, low: u32) -> u32 {
    ((1u32 << (high - low + 1)) - 1) << low
}
const fn field_prep(mask: u32, val: u32) -> u32 {
    (val << mask.trailing_zeros()) & mask
}

pub const XDMA_REG_SPACE_LEN: usize = 65536;
pub const XDMA_MAX_REG_OFFSET: usize = XDMA_REG_SPACE_LEN - 4;
pub const XDMA_MAX_CHANNELS: usize = 4;

pub const XDMA_DESC_BLOCK_NUM: u32 = bit(7);
pub const XDMA_DESC_BLOCK_MASK: u32 = XDMA_DESC_BLOCK_NUM - 1;

pub const XDMA_DESC_ADJACENT: u32 = 32;
pub const XDMA_DESC_ADJACENT_MASK: u32 = XDMA_DESC_ADJACENT - 1;
pub const XDMA_DESC_ADJACENT_BITS: u32 = genmask(13, 8);
pub const XDMA_DESC_MAGIC: u32 = 0xad4b;
pub const XDMA_DESC_MAGIC_BITS: u32 = genmask(31, 16);
pub const XDMA_DESC_FLAGS_BITS: u32 = genmask(7, 0);
pub const XDMA_DESC_STOPPED: u32 = bit(0);
pub const XDMA_DESC_COMPLETED: u32 = bit(1);
pub const XDMA_DESC_BLEN_BITS: u32 = 28;
pub const XDMA_DESC_BLEN_MAX: u32 = bit(XDMA_DESC_BLEN_BITS) - PAGE_SIZE as u32;

pub const fn XDMA_DESC_CONTROL(adjacent: u32, flag: u32) -> u32 {
    field_prep(XDMA_DESC_MAGIC_BITS, XDMA_DESC_MAGIC)
        | field_prep(XDMA_DESC_ADJACENT_BITS, adjacent - 1)
        | field_prep(XDMA_DESC_FLAGS_BITS, flag)
}
pub const XDMA_DESC_CONTROL_LAST: u32 =
    XDMA_DESC_CONTROL(1, XDMA_DESC_STOPPED | XDMA_DESC_COMPLETED);
pub const XDMA_DESC_CONTROL_CYCLIC: u32 = XDMA_DESC_CONTROL(1, XDMA_DESC_COMPLETED);

#[repr(C)]
pub struct xdma_hw_desc {
    pub control: u32,
    pub bytes: u32,
    pub src_addr: u64,
    pub dst_addr: u64,
    pub next_desc: u64,
}

pub const XDMA_DESC_SIZE: usize = core::mem::size_of::<xdma_hw_desc>();
pub const XDMA_DESC_BLOCK_SIZE: usize = XDMA_DESC_SIZE * XDMA_DESC_ADJACENT as usize;
pub const XDMA_DESC_BLOCK_ALIGN: usize = 32;
pub const XDMA_DESC_BLOCK_BOUNDARY: usize = 4096;

pub const XDMA_CHAN_IDENTIFIER: u32 = 0x0;
pub const XDMA_CHAN_CONTROL: u32 = 0x4;
pub const XDMA_CHAN_CONTROL_W1S: u32 = 0x8;
pub const XDMA_CHAN_CONTROL_W1C: u32 = 0xc;
pub const XDMA_CHAN_STATUS: u32 = 0x40;
pub const XDMA_CHAN_STATUS_RC: u32 = 0x44;
pub const XDMA_CHAN_COMPLETED_DESC: u32 = 0x48;
pub const XDMA_CHAN_ALIGNMENTS: u32 = 0x4c;
pub const XDMA_CHAN_INTR_ENABLE: u32 = 0x90;
pub const XDMA_CHAN_INTR_ENABLE_W1S: u32 = 0x94;
pub const XDMA_CHAN_INTR_ENABLE_W1C: u32 = 0x9c;
pub const XDMA_CHAN_STRIDE: u32 = 0x100;
pub const XDMA_CHAN_H2C_OFFSET: u32 = 0x0;
pub const XDMA_CHAN_C2H_OFFSET: u32 = 0x1000;
pub const XDMA_CHAN_H2C_TARGET: u32 = 0x0;
pub const XDMA_CHAN_C2H_TARGET: u32 = 0x1;
pub const XDMA_CHAN_MAGIC: u32 = 0x1fc0;
pub const fn XDMA_CHAN_CHECK_TARGET(id: u32, target: u32) -> bool {
    (id >> 16) == XDMA_CHAN_MAGIC + target
}

pub const CHAN_CTRL_RUN_STOP: u32 = bit(0);
pub const CHAN_CTRL_IE_DESC_STOPPED: u32 = bit(1);
pub const CHAN_CTRL_IE_DESC_COMPLETED: u32 = bit(2);
pub const CHAN_CTRL_IE_DESC_ALIGN_MISMATCH: u32 = bit(3);
pub const CHAN_CTRL_IE_MAGIC_STOPPED: u32 = bit(4);
pub const CHAN_CTRL_IE_IDLE_STOPPED: u32 = bit(6);
pub const CHAN_CTRL_IE_READ_ERROR: u32 = genmask(13, 9);
pub const CHAN_CTRL_IE_WRITE_ERROR: u32 = genmask(18, 14);
pub const CHAN_CTRL_IE_DESC_ERROR: u32 = genmask(23, 19);
pub const CHAN_CTRL_NON_INCR_ADDR: u32 = bit(25);
pub const CHAN_CTRL_POLL_MODE_WB: u32 = bit(26);
pub const CHAN_CTRL_START: u32 = CHAN_CTRL_RUN_STOP | CHAN_CTRL_IE_DESC_STOPPED
    | CHAN_CTRL_IE_DESC_COMPLETED | CHAN_CTRL_IE_DESC_ALIGN_MISMATCH
    | CHAN_CTRL_IE_MAGIC_STOPPED | CHAN_CTRL_IE_READ_ERROR
    | CHAN_CTRL_IE_WRITE_ERROR | CHAN_CTRL_IE_DESC_ERROR;
pub const XDMA_CHAN_STATUS_BUSY: u32 = bit(0);
pub const XDMA_CHAN_STATUS_MASK: u32 = CHAN_CTRL_START;
pub const XDMA_CHAN_ERROR_MASK: u32 = CHAN_CTRL_IE_DESC_ALIGN_MISMATCH
    | CHAN_CTRL_IE_MAGIC_STOPPED | CHAN_CTRL_IE_READ_ERROR
    | CHAN_CTRL_IE_WRITE_ERROR | CHAN_CTRL_IE_DESC_ERROR;

pub const CHAN_IM_DESC_ERROR: u32 = bit(19);
pub const CHAN_IM_READ_ERROR: u32 = bit(9);
pub const CHAN_IM_IDLE_STOPPED: u32 = bit(6);
pub const CHAN_IM_MAGIC_STOPPED: u32 = bit(4);
pub const CHAN_IM_DESC_COMPLETED: u32 = bit(2);
pub const CHAN_IM_DESC_STOPPED: u32 = bit(1);
pub const CHAN_IM_ALL: u32 = CHAN_IM_DESC_ERROR | CHAN_IM_READ_ERROR
    | CHAN_IM_IDLE_STOPPED | CHAN_IM_MAGIC_STOPPED
    | CHAN_IM_DESC_COMPLETED | CHAN_IM_DESC_STOPPED;

pub const XDMA_SGDMA_IDENTIFIER: u32 = 0x4000;
pub const XDMA_SGDMA_DESC_LO: u32 = 0x4080;
pub const XDMA_SGDMA_DESC_HI: u32 = 0x4084;
pub const XDMA_SGDMA_DESC_ADJ: u32 = 0x4088;
pub const XDMA_SGDMA_DESC_CREDIT: u32 = 0x408c;

pub const XDMA_IRQ_IDENTIFIER: u32 = 0x2000;
pub const XDMA_IRQ_USER_INT_EN: u32 = 0x2004;
pub const XDMA_IRQ_USER_INT_EN_W1S: u32 = 0x2008;
pub const XDMA_IRQ_USER_INT_EN_W1C: u32 = 0x200c;
pub const XDMA_IRQ_CHAN_INT_EN: u32 = 0x2010;
pub const XDMA_IRQ_CHAN_INT_EN_W1S: u32 = 0x2014;
pub const XDMA_IRQ_CHAN_INT_EN_W1C: u32 = 0x2018;
pub const XDMA_IRQ_USER_INT_REQ: u32 = 0x2040;
pub const XDMA_IRQ_CHAN_INT_REQ: u32 = 0x2044;
pub const XDMA_IRQ_USER_INT_PEND: u32 = 0x2048;
pub const XDMA_IRQ_CHAN_INT_PEND: u32 = 0x204c;
pub const XDMA_IRQ_USER_VEC_NUM: u32 = 0x2080;
pub const XDMA_IRQ_CHAN_VEC_NUM: u32 = 0x20a0;
pub const XDMA_IRQ_VEC_SHIFT: u32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
