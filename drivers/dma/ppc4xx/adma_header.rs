/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * 2006-2009 (C) DENX Software Engineering.
 *
 * Author: Yuri Tikhonov <yur@emcraft.com>
 */

// Dependencies supplied by the surrounding translation unit: dma.h, xor.h,
// and the Linux/kernel type declarations.

macro_rules! to_ppc440spe_adma_chan {
    ($chan:expr) => { container_of!($chan, ppc440spe_adma_chan, common) };
}
macro_rules! to_ppc440spe_adma_device {
    ($dev:expr) => { container_of!($dev, ppc440spe_adma_device, common) };
}
macro_rules! tx_to_ppc440spe_adma_slot {
    ($tx:expr) => { container_of!($tx, ppc440spe_adma_desc_slot, async_tx) };
}

/* Default polynomial (for 440SP is only available) */
pub const PPC440SPE_DEFAULT_POLY: u32 = 0x4d;
pub const PPC440SPE_ADMA_ENGINES_NUM: u32 = XOR_ENGINES_NUM + DMA_ENGINES_NUM;
pub const PPC440SPE_ADMA_WATCHDOG_MSEC: u32 = 3;
pub const PPC440SPE_ADMA_THRESHOLD: u32 = 1;
pub const PPC440SPE_DMA0_ID: u32 = 0;
pub const PPC440SPE_DMA1_ID: u32 = 1;
pub const PPC440SPE_XOR_ID: u32 = 2;
pub const PPC440SPE_ADMA_DMA_MAX_BYTE_COUNT: u32 = 0xFFFFFF;
/* this is the XOR_CBBCR width */
pub const PPC440SPE_ADMA_XOR_MAX_BYTE_COUNT: u32 = 1 << 31;
pub const PPC440SPE_ADMA_ZERO_SUM_MAX_BYTE_COUNT: u32 = PPC440SPE_ADMA_XOR_MAX_BYTE_COUNT;
pub const PPC440SPE_RXOR_RUN: u32 = 0;
pub const MQ0_CF2H_RXOR_BS_MASK: u32 = 0x1FF;

#[repr(C)]
pub struct ppc440spe_adma_device {
    pub dev: *mut device,
    pub dma_reg: *mut dma_regs,
    pub xor_reg: *mut xor_regs,
    pub i2o_reg: *mut i2o_regs,
    pub id: i32,
    pub dma_desc_pool_virt: *mut core::ffi::c_void,
    pub dma_desc_pool: dma_addr_t,
    pub pool_size: usize,
    pub irq: i32,
    pub err_irq: i32,
    pub common: dma_device,
}

#[repr(C)]
pub struct ppc440spe_adma_chan {
    pub lock: spinlock_t,
    pub device: *mut ppc440spe_adma_device,
    pub chain: list_head,
    pub common: dma_chan,
    pub all_slots: list_head,
    pub last_used: *mut ppc440spe_adma_desc_slot,
    pub pending: i32,
    pub slots_allocated: i32,
    pub hw_chain_inited: i32,
    pub irq_tasklet: tasklet_struct,
    pub needs_unmap: u8,
    pub pdest_page: *mut page,
    pub qdest_page: *mut page,
    pub pdest: dma_addr_t,
    pub qdest: dma_addr_t,
}

#[repr(C)]
pub struct ppc440spe_rxor {
    pub addrl: u32,
    pub addrh: u32,
    pub len: i32,
    pub xor_count: i32,
    pub addr_count: i32,
    pub desc_count: i32,
    pub state: i32,
}

#[repr(C)]
pub struct ppc440spe_adma_desc_slot {
    pub phys: dma_addr_t,
    pub group_head: *mut ppc440spe_adma_desc_slot,
    pub hw_next: *mut ppc440spe_adma_desc_slot,
    pub async_tx: dma_async_tx_descriptor,
    pub slot_node: list_head,
    pub chain_node: list_head, /* node in channel ops list */
    pub group_list: list_head, /* list */
    pub unmap_len: u32,
    pub hw_desc: *mut core::ffi::c_void,
    pub stride: u16,
    pub idx: u16,
    pub slot_cnt: u16,
    pub src_cnt: u8,
    pub dst_cnt: u8,
    pub slots_per_op: u8,
    pub descs_per_op: u8,
    pub flags: usize,
    pub reverse_flags: [usize; 8],
    pub rxor_cursor: ppc440spe_rxor,
    pub results: ppc440spe_adma_desc_slot_results,
}

#[repr(C)]
pub union ppc440spe_adma_desc_slot_results {
    pub xor_check_result: *mut u32,
    pub crc32_result: *mut u32,
}

pub const PPC440SPE_DESC_INT: u32 = 0; /* generate interrupt on complete */
pub const PPC440SPE_ZERO_P: u32 = 1; /* clear P destionaion */
pub const PPC440SPE_ZERO_Q: u32 = 2; /* clear Q destination */
pub const PPC440SPE_COHERENT: u32 = 3; /* src/dst are coherent */
pub const PPC440SPE_DESC_WXOR: u32 = 4; /* WXORs are in chain */
pub const PPC440SPE_DESC_RXOR: u32 = 5; /* RXOR is in chain */
pub const PPC440SPE_DESC_RXOR123: u32 = 8; /* CDB for RXOR123 operation */
pub const PPC440SPE_DESC_RXOR124: u32 = 9; /* CDB for RXOR124 operation */
pub const PPC440SPE_DESC_RXOR125: u32 = 10; /* CDB for RXOR125 operation */
pub const PPC440SPE_DESC_RXOR12: u32 = 11; /* CDB for RXOR12 operation */
pub const PPC440SPE_DESC_RXOR_REV: u32 = 12; /* CDB has srcs in reversed order */
pub const PPC440SPE_DESC_PCHECK: u32 = 13;
pub const PPC440SPE_DESC_QCHECK: u32 = 14;
pub const PPC440SPE_DESC_RXOR_MSK: u32 = 0x3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
