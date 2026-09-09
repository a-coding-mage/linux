/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OMAP3XXX L3 Interconnect Driver header
 *
 * Copyright (C) 2011 Texas Corporation
 *	Felipe Balbi <balbi@ti.com>
 *	Santosh Shilimkar <santosh.shilimkar@ti.com>
 *	sricharan <r.sricharan@ti.com>
 */

/* Register definitions. All 64-bit wide */
pub const L3_COMPONENT: u32 = 0x000;
pub const L3_CORE: u32 = 0x018;
pub const L3_AGENT_CONTROL: u32 = 0x020;
pub const L3_AGENT_STATUS: u32 = 0x028;
pub const L3_ERROR_LOG: u32 = 0x058;
pub const L3_ERROR_LOG_MULTI: u64 = 1u64 << 31;
pub const L3_ERROR_LOG_SECONDARY: u64 = 1u64 << 30;
pub const L3_ERROR_LOG_ADDR: u32 = 0x060;

/* Register definitions for Sideband Interconnect */
pub const L3_SI_CONTROL: u32 = 0x020;
pub const L3_SI_FLAG_STATUS_0: u32 = 0x510;

pub const shift: u64 = 1;

pub const L3_STATUS_0_MPUIA_BRST: u64 = shift << 0;
pub const L3_STATUS_0_MPUIA_RSP: u64 = shift << 1;
pub const L3_STATUS_0_MPUIA_INBAND: u64 = shift << 2;
pub const L3_STATUS_0_IVAIA_BRST: u64 = shift << 6;
pub const L3_STATUS_0_IVAIA_RSP: u64 = shift << 7;
pub const L3_STATUS_0_IVAIA_INBAND: u64 = shift << 8;
pub const L3_STATUS_0_SGXIA_BRST: u64 = shift << 9;
pub const L3_STATUS_0_SGXIA_RSP: u64 = shift << 10;
pub const L3_STATUS_0_SGXIA_MERROR: u64 = shift << 11;
pub const L3_STATUS_0_CAMIA_BRST: u64 = shift << 12;
pub const L3_STATUS_0_CAMIA_RSP: u64 = shift << 13;
pub const L3_STATUS_0_CAMIA_INBAND: u64 = shift << 14;
pub const L3_STATUS_0_DISPIA_BRST: u64 = shift << 15;
pub const L3_STATUS_0_DISPIA_RSP: u64 = shift << 16;
pub const L3_STATUS_0_DMARDIA_BRST: u64 = shift << 18;
pub const L3_STATUS_0_DMARDIA_RSP: u64 = shift << 19;
pub const L3_STATUS_0_DMAWRIA_BRST: u64 = shift << 21;
pub const L3_STATUS_0_DMAWRIA_RSP: u64 = shift << 22;
pub const L3_STATUS_0_USBOTGIA_BRST: u64 = shift << 24;
pub const L3_STATUS_0_USBOTGIA_RSP: u64 = shift << 25;
pub const L3_STATUS_0_USBOTGIA_INBAND: u64 = shift << 26;
pub const L3_STATUS_0_USBHOSTIA_BRST: u64 = shift << 27;
pub const L3_STATUS_0_USBHOSTIA_INBAND: u64 = shift << 28;
pub const L3_STATUS_0_SMSTA_REQ: u64 = shift << 48;
pub const L3_STATUS_0_GPMCTA_REQ: u64 = shift << 49;
pub const L3_STATUS_0_OCMRAMTA_REQ: u64 = shift << 50;
pub const L3_STATUS_0_OCMROMTA_REQ: u64 = shift << 51;
pub const L3_STATUS_0_IVATA_REQ: u64 = shift << 54;
pub const L3_STATUS_0_SGXTA_REQ: u64 = shift << 55;
pub const L3_STATUS_0_SGXTA_SERROR: u64 = shift << 56;
pub const L3_STATUS_0_GPMCTA_SERROR: u64 = shift << 57;
pub const L3_STATUS_0_L4CORETA_REQ: u64 = shift << 58;
pub const L3_STATUS_0_L4PERTA_REQ: u64 = shift << 59;
pub const L3_STATUS_0_L4EMUTA_REQ: u64 = shift << 60;
pub const L3_STATUS_0_MAD2DTA_REQ: u64 = shift << 61;
pub const L3_STATUS_0_TIMEOUT_MASK: u64 =
    L3_STATUS_0_MPUIA_BRST | L3_STATUS_0_MPUIA_RSP | L3_STATUS_0_IVAIA_BRST |
    L3_STATUS_0_IVAIA_RSP | L3_STATUS_0_SGXIA_BRST | L3_STATUS_0_SGXIA_RSP |
    L3_STATUS_0_CAMIA_BRST | L3_STATUS_0_CAMIA_RSP | L3_STATUS_0_DISPIA_BRST |
    L3_STATUS_0_DISPIA_RSP | L3_STATUS_0_DMARDIA_BRST | L3_STATUS_0_DMARDIA_RSP |
    L3_STATUS_0_DMAWRIA_BRST | L3_STATUS_0_DMAWRIA_RSP | L3_STATUS_0_USBOTGIA_BRST |
    L3_STATUS_0_USBOTGIA_RSP | L3_STATUS_0_USBHOSTIA_BRST | L3_STATUS_0_SMSTA_REQ |
    L3_STATUS_0_GPMCTA_REQ | L3_STATUS_0_OCMRAMTA_REQ | L3_STATUS_0_OCMROMTA_REQ |
    L3_STATUS_0_IVATA_REQ | L3_STATUS_0_SGXTA_REQ | L3_STATUS_0_L4CORETA_REQ |
    L3_STATUS_0_L4PERTA_REQ | L3_STATUS_0_L4EMUTA_REQ | L3_STATUS_0_MAD2DTA_REQ;

pub const L3_SI_FLAG_STATUS_1: u32 = 0x530;
pub const L3_STATUS_1_MPU_DATAIA: u32 = 1 << 0;
pub const L3_STATUS_1_DAPIA0: u32 = 1 << 3;
pub const L3_STATUS_1_DAPIA1: u32 = 1 << 4;
pub const L3_STATUS_1_IVAIA: u32 = 1 << 6;
pub const L3_PM_ERROR_LOG: u32 = 0x020;
pub const L3_PM_CONTROL: u32 = 0x028;
pub const L3_PM_ERROR_CLEAR_SINGLE: u32 = 0x030;
pub const L3_PM_ERROR_CLEAR_MULTI: u32 = 0x038;
pub const L3_PM_REQ_INFO_PERMISSION: fn(u32) -> u32 = |n| 0x048 + (0x020 * n);
pub const L3_PM_READ_PERMISSION: fn(u32) -> u32 = |n| 0x050 + (0x020 * n);
pub const L3_PM_WRITE_PERMISSION: fn(u32) -> u32 = |n| 0x058 + (0x020 * n);
pub const L3_PM_ADDR_MATCH: fn(u32) -> u32 = |n| 0x060 + (0x020 * n);

pub const L3_ERROR_LOG_CODE: u32 = 24;
pub const L3_ERROR_LOG_INITID: u32 = 8;
pub const L3_ERROR_LOG_CMD: u32 = 0;
pub const L3_AGENT_STATUS_CLEAR_IA: u32 = 0x10000000;
pub const L3_AGENT_STATUS_CLEAR_TA: u32 = 0x01000000;
pub const OMAP34xx_IRQ_L3_APP: i32 = 10;
pub const L3_APPLICATION_ERROR: i32 = 0x0;
pub const L3_DEBUG_ERROR: i32 = 0x1;

#[repr(i32)]
pub enum omap3_l3_initiator_id {
    OMAP_L3_LCD = 29, OMAP_L3_SAD2D = 28,
    OMAP_L3_IA_MPU_SS_1 = 27, OMAP_L3_IA_MPU_SS_2 = 26, OMAP_L3_IA_MPU_SS_3 = 25,
    OMAP_L3_IA_MPU_SS_4 = 24, OMAP_L3_IA_MPU_SS_5 = 23,
    OMAP_L3_IA_IVA_SS_1 = 22, OMAP_L3_IA_IVA_SS_2 = 21, OMAP_L3_IA_IVA_SS_3 = 20,
    OMAP_L3_IA_IVA_SS_DMA_1 = 19, OMAP_L3_IA_IVA_SS_DMA_2 = 18, OMAP_L3_IA_IVA_SS_DMA_3 = 17,
    OMAP_L3_IA_IVA_SS_DMA_4 = 16, OMAP_L3_IA_IVA_SS_DMA_5 = 15, OMAP_L3_IA_IVA_SS_DMA_6 = 14,
    OMAP_L3_IA_SGX = 13, OMAP_L3_IA_CAM_1 = 12, OMAP_L3_IA_CAM_2 = 11, OMAP_L3_IA_CAM_3 = 10,
    OMAP_L3_IA_DAP = 9, OMAP_L3_SDMA_WR_1 = 8, OMAP_L3_SDMA_WR_2 = 7,
    OMAP_L3_SDMA_RD_1 = 6, OMAP_L3_SDMA_RD_2 = 5, OMAP_L3_SDMA_RD_3 = 4,
    OMAP_L3_SDMA_RD_4 = 3, OMAP_L3_USBOTG = 2, OMAP_L3_USBHOST = 1,
}

#[repr(i32)]
pub enum omap3_l3_code {
    OMAP_L3_CODE_NOERROR = 0, OMAP_L3_CODE_UNSUP_CMD = 1, OMAP_L3_CODE_ADDR_HOLE = 2,
    OMAP_L3_CODE_PROTECT_VIOLATION = 3, OMAP_L3_CODE_IN_BAND_ERR = 4,
    /* codes 5 and 6 are reserved */
    OMAP_L3_CODE_REQ_TOUT_NOT_ACCEPT = 7, OMAP_L3_CODE_REQ_TOUT_NO_RESP = 8,
    /* codes 9 - 15 are also reserved */
}

/* External types are supplied by the surrounding translation. */
#[repr(C)]
pub struct omap3_l3 {
    pub dev: *mut device,
    pub ick: *mut clk,
    /* memory base*/
    pub rt: *mut core::ffi::c_void,
    pub debug_irq: i32,
    pub app_irq: i32,
    /* true when and inband functional error occurs */
    pub inband: u32,
}

pub static mut omap3_l3_app_bases: [u32; 64] = [
    0x1400, 0x1400, 0x1400, 0, 0, 0, 0x1800, 0x1800, 0x1800, 0x1c00, 0x1c00, 0,
    0x5800, 0x5800, 0x5800, 0x5400, 0x5400, 0, 0x4c00, 0x4c00, 0, 0x5000, 0x5000, 0,
    0x4400, 0x4400, 0x4400, 0x4000, 0x4000, 0, 0, 0, 0x3000, 0x3000, 0x3000,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x2000, 0x2400, 0x2800, 0x2c00,
    0x6800, 0x6c00, 0x6000, 0x6400, 0x7000, 0x2400, 0x6800, 0x6c00, 0x7000, 0x3400, 0, 0,
];

pub static mut omap3_l3_debug_bases: [u32; 9] = [0x1400, 0, 0, 0x5c00, 0x5c00, 0, 0x1800, 0, 0];

pub static mut omap3_l3_bases: [*mut u32; 2] = unsafe {
    [omap3_l3_app_bases.as_mut_ptr(), omap3_l3_debug_bases.as_mut_ptr()]
};

/*
 * REVISIT define __raw_readll/__raw_writell here, but move them to
 * <asm/io.h> at some point
 */
#[inline]
pub unsafe fn __raw_writell(v: u64, a: *mut u64) {
    core::ptr::write_volatile(a, v);
}

#[inline]
pub unsafe fn __raw_readll(a: *const u64) -> u64 {
    core::ptr::read_volatile(a)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
