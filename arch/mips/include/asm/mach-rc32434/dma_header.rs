/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2002 Integrated Device Technology, Inc.
 *		All rights reserved.
 *
 * DMA register definition.
 *
 * Author : ryan.holmQVist@idt.com
 * Date	  : 20011005
 */

// Dependency supplied by the surrounding translated sources: `u32`.

pub const DMA0_BASE_ADDR: usize = 0x18040000;

/*
 * DMA descriptor (in physical memory).
 */

#[repr(C)]
pub struct dma_desc {
    pub control: u32, // Control. use DMAD_*
    pub ca: u32,      // Current Address.
    pub devcs: u32,   // Device control and status.
    pub link: u32,    // Next descriptor in chain.
}

pub const DMA_DESC_SIZ: usize = core::mem::size_of::<dma_desc>();
pub const DMA_DESC_COUNT_BIT: u32 = 0;
pub const DMA_DESC_COUNT_MSK: u32 = 0x0003ffff;
pub const DMA_DESC_DS_BIT: u32 = 20;
pub const DMA_DESC_DS_MSK: u32 = 0x00300000;

pub const DMA_DESC_DEV_CMD_BIT: u32 = 22;
pub const DMA_DESC_DEV_CMD_MSK: u32 = 0x01c00000;

/* DMA command sizes */
pub const DMA_DESC_DEV_CMD_BYTE: u32 = 0;
pub const DMA_DESC_DEV_CMD_HLF_WD: u32 = 1;
pub const DMA_DESC_DEV_CMD_WORD: u32 = 2;
pub const DMA_DESC_DEV_CMD_2WORDS: u32 = 3;
pub const DMA_DESC_DEV_CMD_4WORDS: u32 = 4;
pub const DMA_DESC_DEV_CMD_6WORDS: u32 = 5;
pub const DMA_DESC_DEV_CMD_8WORDS: u32 = 6;
pub const DMA_DESC_DEV_CMD_16WORDS: u32 = 7;

/* DMA descriptors interrupts */
pub const DMA_DESC_COF: u32 = 1 << 25; // Chain on finished
pub const DMA_DESC_COD: u32 = 1 << 26; // Chain on done
pub const DMA_DESC_IOF: u32 = 1 << 27; // Interrupt on finished
pub const DMA_DESC_IOD: u32 = 1 << 28; // Interrupt on done
pub const DMA_DESC_TERM: u32 = 1 << 29; // Terminated
pub const DMA_DESC_DONE: u32 = 1 << 30; // Done
pub const DMA_DESC_FINI: u32 = 1 << 31; // Finished

/*
 * DMA register (within Internal Register Map).
 */

#[repr(C)]
pub struct dma_reg {
    pub dmac: u32,    // Control.
    pub dmas: u32,    // Status.
    pub dmasm: u32,   // Mask.
    pub dmadptr: u32, // Descriptor pointer.
    pub dmandptr: u32, // Next descriptor pointer.
}

/* DMA channels specific registers */
pub const DMA_CHAN_RUN_BIT: u32 = 1 << 0;
pub const DMA_CHAN_DONE_BIT: u32 = 1 << 1;
pub const DMA_CHAN_MODE_BIT: u32 = 1 << 2;
pub const DMA_CHAN_MODE_MSK: u32 = 0x0000000c;
pub const DMA_CHAN_MODE_AUTO: u32 = 0;
pub const DMA_CHAN_MODE_BURST: u32 = 1;
pub const DMA_CHAN_MODE_XFRT: u32 = 2;
pub const DMA_CHAN_MODE_RSVD: u32 = 3;
pub const DMA_CHAN_ACT_BIT: u32 = 1 << 4;

/* DMA status registers */
pub const DMA_STAT_FINI: u32 = 1 << 0;
pub const DMA_STAT_DONE: u32 = 1 << 1;
pub const DMA_STAT_CHAIN: u32 = 1 << 2;
pub const DMA_STAT_ERR: u32 = 1 << 3;
pub const DMA_STAT_HALT: u32 = 1 << 4;

/*
 * DMA channel definitions
 */

pub const DMA_CHAN_ETH_RCV: usize = 0;
pub const DMA_CHAN_ETH_XMT: usize = 1;
pub const DMA_CHAN_MEM_TO_FIFO: usize = 2;
pub const DMA_CHAN_FIFO_TO_MEM: usize = 3;
pub const DMA_CHAN_PCI_TO_MEM: usize = 4;
pub const DMA_CHAN_MEM_TO_PCI: usize = 5;
pub const DMA_CHAN_COUNT: usize = 6;

#[repr(C)]
pub struct dma_channel {
    pub ch: [dma_reg; DMA_CHAN_COUNT],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
