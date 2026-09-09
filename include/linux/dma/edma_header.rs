/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018-2019 Synopsys, Inc. and/or its affiliates.
 * Synopsys DesignWare eDMA core driver
 *
 * Author: Gustavo Pimentel <gustavo.pimentel@synopsys.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub const EDMA_MAX_WR_CH: usize = 8;
pub const EDMA_MAX_RD_CH: usize = 8;
pub const HDMA_MAX_WR_CH: usize = 64;
pub const HDMA_MAX_RD_CH: usize = 64;

pub struct dw_edma;

#[repr(C)]
pub union dw_edma_region_vaddr {
    pub mem: *mut core::ffi::c_void,
    pub io: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct dw_edma_region {
    pub paddr: u64,
    pub vaddr: dw_edma_region_vaddr,
    pub sz: usize,
}

/**
 * struct dw_edma_plat_ops - platform-specific eDMA methods
 * @irq_vector: Get IRQ number of the passed eDMA channel.
 * @pci_address: Get PCIe bus address corresponding to the passed CPU address.
 */
#[repr(C)]
pub struct dw_edma_plat_ops {
    pub irq_vector: Option<unsafe extern "C" fn(dev: *mut crate::device::device, nr: u32) -> i32>,
    pub pci_address: Option<unsafe extern "C" fn(dev: *mut crate::device::device, cpu_addr: u64) -> u64>,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum dw_edma_map_format {
    EDMA_MF_EDMA_LEGACY = 0x0,
    EDMA_MF_EDMA_UNROLL = 0x1,
    EDMA_MF_HDMA_COMPAT = 0x5,
    EDMA_MF_HDMA_NATIVE = 0x7,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum dw_edma_chip_flags {
    DW_EDMA_CHIP_LOCAL = 1u32 << 0,
    DW_EDMA_CHIP_PARTIAL = 1u32 << 1,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum dw_edma_ch_irq_mode {
    DW_EDMA_CH_IRQ_LOCAL = 0,
    DW_EDMA_CH_IRQ_REMOTE,
}

#[repr(C)]
pub struct dw_edma_chip {
    pub dev: *mut crate::device::device,
    pub nr_irqs: i32,
    pub ops: *const dw_edma_plat_ops,
    pub flags: u32,

    pub reg_base: *mut core::ffi::c_void,

    pub ll_wr_cnt: u16,
    pub ll_rd_cnt: u16,
    /* link list address */
    pub ll_region_wr: [dw_edma_region; HDMA_MAX_WR_CH],
    pub ll_region_rd: [dw_edma_region; HDMA_MAX_RD_CH],

    /* data region */
    pub dt_region_wr: [dw_edma_region; HDMA_MAX_WR_CH],
    pub dt_region_rd: [dw_edma_region; HDMA_MAX_RD_CH],

    /* interrupt emulation */
    pub db_irq: i32,
    pub db_offset: u64,

    pub mf: dw_edma_map_format,
    pub func_no: u8,

    pub dw: *mut dw_edma,
    pub cfg_non_ll: bool,
}

// CONFIG_DW_EDMA build-time condition from the original header.
#[cfg(feature = "CONFIG_DW_EDMA")]
unsafe extern "C" {
    pub fn dw_edma_probe(chip: *mut dw_edma_chip) -> i32;
    pub fn dw_edma_remove(chip: *mut dw_edma_chip) -> i32;
}

#[cfg(not(feature = "CONFIG_DW_EDMA"))]
#[inline]
pub unsafe fn dw_edma_probe(_chip: *mut dw_edma_chip) -> i32 {
    -19 // -ENODEV
}

#[cfg(not(feature = "CONFIG_DW_EDMA"))]
#[inline]
pub unsafe fn dw_edma_remove(_chip: *mut dw_edma_chip) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
