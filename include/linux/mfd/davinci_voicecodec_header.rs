/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * DaVinci Voice Codec Core Interface for TI platforms
 *
 * Copyright (C) 2010 Texas Instruments, Inc
 *
 * Author: Miguel Aguilar <miguel.aguilar@ridgerun.com>
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[allow(non_camel_case_types)]
pub type u32 = ::core::primitive::u32;

#[allow(non_camel_case_types)]
pub type dma_addr_t = u64;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mfd_cell {
    _private: [u8; 0],
}

pub const DAVINCI_VC_PID: u32 = 0x00;
pub const DAVINCI_VC_CTRL: u32 = 0x04;
pub const DAVINCI_VC_INTEN: u32 = 0x08;
pub const DAVINCI_VC_INTSTATUS: u32 = 0x0c;
pub const DAVINCI_VC_INTCLR: u32 = 0x10;
pub const DAVINCI_VC_EMUL_CTRL: u32 = 0x14;
pub const DAVINCI_VC_RFIFO: u32 = 0x20;
pub const DAVINCI_VC_WFIFO: u32 = 0x24;
pub const DAVINCI_VC_FIFOSTAT: u32 = 0x28;
pub const DAVINCI_VC_TST_CTRL: u32 = 0x2C;
pub const DAVINCI_VC_REG05: u32 = 0x94;
pub const DAVINCI_VC_REG09: u32 = 0xA4;
pub const DAVINCI_VC_REG12: u32 = 0xB0;

pub const DAVINCI_VC_CTRL_MASK: u32 = 0x5500;
pub const DAVINCI_VC_CTRL_RSTADC: u32 = 1u32 << 0;
pub const DAVINCI_VC_CTRL_RSTDAC: u32 = 1u32 << 1;
pub const DAVINCI_VC_CTRL_RD_BITS_8: u32 = 1u32 << 4;
pub const DAVINCI_VC_CTRL_RD_UNSIGNED: u32 = 1u32 << 5;
pub const DAVINCI_VC_CTRL_WD_BITS_8: u32 = 1u32 << 6;
pub const DAVINCI_VC_CTRL_WD_UNSIGNED: u32 = 1u32 << 7;
pub const DAVINCI_VC_CTRL_RFIFOEN: u32 = 1u32 << 8;
pub const DAVINCI_VC_CTRL_RFIFOCL: u32 = 1u32 << 9;
pub const DAVINCI_VC_CTRL_RFIFOMD_WORD_1: u32 = 1u32 << 10;
pub const DAVINCI_VC_CTRL_WFIFOEN: u32 = 1u32 << 12;
pub const DAVINCI_VC_CTRL_WFIFOCL: u32 = 1u32 << 13;
pub const DAVINCI_VC_CTRL_WFIFOMD_WORD_1: u32 = 1u32 << 14;

pub const DAVINCI_VC_INT_MASK: u32 = 0x3F;
pub const DAVINCI_VC_INT_RDRDY_MASK: u32 = 1u32 << 0;
pub const DAVINCI_VC_INT_RERROVF_MASK: u32 = 1u32 << 1;
pub const DAVINCI_VC_INT_RERRUDR_MASK: u32 = 1u32 << 2;
pub const DAVINCI_VC_INT_WDREQ_MASK: u32 = 1u32 << 3;
pub const DAVINCI_VC_INT_WERROVF_MASKBIT: u32 = 1u32 << 4;
pub const DAVINCI_VC_INT_WERRUDR_MASK: u32 = 1u32 << 5;

pub const DAVINCI_VC_REG05_PGA_GAIN: u32 = 0x07;
pub const DAVINCI_VC_REG09_MUTE: u32 = 0x40;
pub const DAVINCI_VC_REG09_DIG_ATTEN: u32 = 0x3F;
pub const DAVINCI_VC_REG12_POWER_ALL_ON: u32 = 0xFD;
pub const DAVINCI_VC_REG12_POWER_ALL_OFF: u32 = 0x00;

pub const DAVINCI_VC_CELLS: usize = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum davinci_vc_cells {
    DAVINCI_VC_VCIF_CELL,
    DAVINCI_VC_CQ93VC_CELL,
}

#[repr(C)]
pub struct davinci_vcif {
    pub pdev: *mut platform_device,
    pub dma_tx_channel: u32,
    pub dma_rx_channel: u32,
    pub dma_tx_addr: dma_addr_t,
    pub dma_rx_addr: dma_addr_t,
}

#[repr(C)]
pub struct davinci_vc {
    /* Device data */
    pub dev: *mut device,
    pub pdev: *mut platform_device,
    pub clk: *mut clk,

    /* Memory resources */
    pub base: *mut core::ffi::c_void,
    pub regmap: *mut regmap,

    /* MFD cells */
    pub cells: [mfd_cell; DAVINCI_VC_CELLS],

    /* Client devices */
    pub davinci_vcif: davinci_vcif,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
