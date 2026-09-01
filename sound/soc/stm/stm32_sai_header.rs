/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * STM32 ALSA SoC Digital Audio Interface (SAI) driver.
 *
 * Copyright (C) 2016, STMicroelectronics - All Rights Reserved
 * Author(s): Olivier Moysan <olivier.moysan@st.com> for STMicroelectronics.
 */

/* C dependency: <linux/bitfield.h> */

const fn bit(nr: u32) -> u32 {
    1u32 << nr
}

const fn genmask(h: u32, l: u32) -> u32 {
    (((!0u64) - (1u64 << l) + 1) & (!0u64 >> (32 - 1 - h))) as u32
}

const fn field_get(mask: u32, reg: u32) -> u32 {
    (reg & mask) >> mask.trailing_zeros()
}

/******************** SAI Register Map **************************************/

/* Global configuration register */
pub const STM_SAI_GCR: u32 = 0x00;

/* Sub-block A&B registers offsets, relative to A&B sub-block addresses */
pub const STM_SAI_CR1_REGX: u32 = 0x00; /* A offset: 0x04. B offset: 0x24 */
pub const STM_SAI_CR2_REGX: u32 = 0x04;
pub const STM_SAI_FRCR_REGX: u32 = 0x08;
pub const STM_SAI_SLOTR_REGX: u32 = 0x0C;
pub const STM_SAI_IMR_REGX: u32 = 0x10;
pub const STM_SAI_SR_REGX: u32 = 0x14;
pub const STM_SAI_CLRFR_REGX: u32 = 0x18;
pub const STM_SAI_DR_REGX: u32 = 0x1C;

/* Sub-block A registers, relative to sub-block A address */
pub const STM_SAI_PDMCR_REGX: u32 = 0x40;
pub const STM_SAI_PDMLY_REGX: u32 = 0x44;

/* Hardware configuration registers */
pub const STM_SAI_HWCFGR: u32 = 0x3F0;
pub const STM_SAI_VERR: u32 = 0x3F4;
pub const STM_SAI_IDR: u32 = 0x3F8;
pub const STM_SAI_SIDR: u32 = 0x3FC;

/******************** Bit definition for SAI_GCR register *******************/
pub const SAI_GCR_SYNCIN_SHIFT: u32 = 0;
pub const SAI_GCR_SYNCIN_WDTH: u32 = 2;
pub const SAI_GCR_SYNCIN_MASK: u32 = genmask(1, SAI_GCR_SYNCIN_SHIFT);
pub const SAI_GCR_SYNCIN_MAX: u32 = field_get(SAI_GCR_SYNCIN_MASK, SAI_GCR_SYNCIN_MASK);

pub const SAI_GCR_SYNCOUT_SHIFT: u32 = 4;
pub const SAI_GCR_SYNCOUT_MASK: u32 = genmask(5, SAI_GCR_SYNCOUT_SHIFT);

/******************* Bit definition for SAI_XCR1 register *******************/
pub const SAI_XCR1_RX_TX_SHIFT: u32 = 0;
pub const SAI_XCR1_RX_TX: u32 = bit(SAI_XCR1_RX_TX_SHIFT);
pub const SAI_XCR1_SLAVE_SHIFT: u32 = 1;
pub const SAI_XCR1_SLAVE: u32 = bit(SAI_XCR1_SLAVE_SHIFT);

pub const SAI_XCR1_PRTCFG_SHIFT: u32 = 2;
pub const SAI_XCR1_PRTCFG_MASK: u32 = genmask(3, SAI_XCR1_PRTCFG_SHIFT);
pub const fn SAI_XCR1_PRTCFG_SET(x: u32) -> u32 {
    x << SAI_XCR1_PRTCFG_SHIFT
}

pub const SAI_XCR1_DS_SHIFT: u32 = 5;
pub const SAI_XCR1_DS_MASK: u32 = genmask(7, SAI_XCR1_DS_SHIFT);
pub const fn SAI_XCR1_DS_SET(x: u32) -> u32 {
    x << SAI_XCR1_DS_SHIFT
}

pub const SAI_XCR1_LSBFIRST_SHIFT: u32 = 8;
pub const SAI_XCR1_LSBFIRST: u32 = bit(SAI_XCR1_LSBFIRST_SHIFT);
pub const SAI_XCR1_CKSTR_SHIFT: u32 = 9;
pub const SAI_XCR1_CKSTR: u32 = bit(SAI_XCR1_CKSTR_SHIFT);

pub const SAI_XCR1_SYNCEN_SHIFT: u32 = 10;
pub const SAI_XCR1_SYNCEN_MASK: u32 = genmask(11, SAI_XCR1_SYNCEN_SHIFT);
pub const fn SAI_XCR1_SYNCEN_SET(x: u32) -> u32 {
    x << SAI_XCR1_SYNCEN_SHIFT
}

pub const SAI_XCR1_MONO_SHIFT: u32 = 12;
pub const SAI_XCR1_MONO: u32 = bit(SAI_XCR1_MONO_SHIFT);
pub const SAI_XCR1_OUTDRIV_SHIFT: u32 = 13;
pub const SAI_XCR1_OUTDRIV: u32 = bit(SAI_XCR1_OUTDRIV_SHIFT);
pub const SAI_XCR1_SAIEN_SHIFT: u32 = 16;
pub const SAI_XCR1_SAIEN: u32 = bit(SAI_XCR1_SAIEN_SHIFT);
pub const SAI_XCR1_DMAEN_SHIFT: u32 = 17;
pub const SAI_XCR1_DMAEN: u32 = bit(SAI_XCR1_DMAEN_SHIFT);
pub const SAI_XCR1_NODIV_SHIFT: u32 = 19;
pub const SAI_XCR1_NODIV: u32 = bit(SAI_XCR1_NODIV_SHIFT);

pub const SAI_XCR1_MCKDIV_SHIFT: u32 = 20;
pub const fn SAI_XCR1_MCKDIV_WIDTH(x: u32) -> u32 {
    if x == STM_SAI_STM32F4 { 4 } else { 6 }
}
pub const fn SAI_XCR1_MCKDIV_MASK(x: u32) -> u32 {
    genmask(SAI_XCR1_MCKDIV_SHIFT + x - 1, SAI_XCR1_MCKDIV_SHIFT)
}
pub const fn SAI_XCR1_MCKDIV_SET(x: u32) -> u32 {
    x << SAI_XCR1_MCKDIV_SHIFT
}
pub const fn SAI_XCR1_MCKDIV_MAX(x: u32) -> u32 {
    (1 << SAI_XCR1_MCKDIV_WIDTH(x)) - 1
}

pub const SAI_XCR1_OSR_SHIFT: u32 = 26;
pub const SAI_XCR1_OSR: u32 = bit(SAI_XCR1_OSR_SHIFT);

pub const SAI_XCR1_MCKEN_SHIFT: u32 = 27;
pub const SAI_XCR1_MCKEN: u32 = bit(SAI_XCR1_MCKEN_SHIFT);

/******************* Bit definition for SAI_XCR2 register *******************/
pub const SAI_XCR2_FTH_SHIFT: u32 = 0;
pub const SAI_XCR2_FTH_MASK: u32 = genmask(2, SAI_XCR2_FTH_SHIFT);
pub const fn SAI_XCR2_FTH_SET(x: u32) -> u32 {
    x << SAI_XCR2_FTH_SHIFT
}

pub const SAI_XCR2_FFLUSH_SHIFT: u32 = 3;
pub const SAI_XCR2_FFLUSH: u32 = bit(SAI_XCR2_FFLUSH_SHIFT);
pub const SAI_XCR2_TRIS_SHIFT: u32 = 4;
pub const SAI_XCR2_TRIS: u32 = bit(SAI_XCR2_TRIS_SHIFT);
pub const SAI_XCR2_MUTE_SHIFT: u32 = 5;
pub const SAI_XCR2_MUTE: u32 = bit(SAI_XCR2_MUTE_SHIFT);
pub const SAI_XCR2_MUTEVAL_SHIFT: u32 = 6;
pub const SAI_XCR2_MUTEVAL: u32 = bit(SAI_XCR2_MUTEVAL_SHIFT);

pub const SAI_XCR2_MUTECNT_SHIFT: u32 = 7;
pub const SAI_XCR2_MUTECNT_MASK: u32 = genmask(12, SAI_XCR2_MUTECNT_SHIFT);
pub const fn SAI_XCR2_MUTECNT_SET(x: u32) -> u32 {
    x << SAI_XCR2_MUTECNT_SHIFT
}

pub const SAI_XCR2_CPL_SHIFT: u32 = 13;
pub const SAI_XCR2_CPL: u32 = bit(SAI_XCR2_CPL_SHIFT);

pub const SAI_XCR2_COMP_SHIFT: u32 = 14;
pub const SAI_XCR2_COMP_MASK: u32 = genmask(15, SAI_XCR2_COMP_SHIFT);
pub const fn SAI_XCR2_COMP_SET(x: u32) -> u32 {
    x << SAI_XCR2_COMP_SHIFT
}

/****************** Bit definition for SAI_XFRCR register *******************/
pub const SAI_XFRCR_FRL_SHIFT: u32 = 0;
pub const SAI_XFRCR_FRL_MASK: u32 = genmask(7, SAI_XFRCR_FRL_SHIFT);
pub const fn SAI_XFRCR_FRL_SET(x: u32) -> u32 {
    x << SAI_XFRCR_FRL_SHIFT
}

pub const SAI_XFRCR_FSALL_SHIFT: u32 = 8;
pub const SAI_XFRCR_FSALL_MASK: u32 = genmask(14, SAI_XFRCR_FSALL_SHIFT);
pub const fn SAI_XFRCR_FSALL_SET(x: u32) -> u32 {
    x << SAI_XFRCR_FSALL_SHIFT
}

pub const SAI_XFRCR_FSDEF_SHIFT: u32 = 16;
pub const SAI_XFRCR_FSDEF: u32 = bit(SAI_XFRCR_FSDEF_SHIFT);
pub const SAI_XFRCR_FSPOL_SHIFT: u32 = 17;
pub const SAI_XFRCR_FSPOL: u32 = bit(SAI_XFRCR_FSPOL_SHIFT);
pub const SAI_XFRCR_FSOFF_SHIFT: u32 = 18;
pub const SAI_XFRCR_FSOFF: u32 = bit(SAI_XFRCR_FSOFF_SHIFT);

/****************** Bit definition for SAI_XSLOTR register ******************/
pub const SAI_XSLOTR_FBOFF_SHIFT: u32 = 0;
pub const SAI_XSLOTR_FBOFF_MASK: u32 = genmask(4, SAI_XSLOTR_FBOFF_SHIFT);
pub const fn SAI_XSLOTR_FBOFF_SET(x: u32) -> u32 {
    x << SAI_XSLOTR_FBOFF_SHIFT
}

pub const SAI_XSLOTR_SLOTSZ_SHIFT: u32 = 6;
pub const SAI_XSLOTR_SLOTSZ_MASK: u32 = genmask(7, SAI_XSLOTR_SLOTSZ_SHIFT);
pub const fn SAI_XSLOTR_SLOTSZ_SET(x: u32) -> u32 {
    x << SAI_XSLOTR_SLOTSZ_SHIFT
}

pub const SAI_XSLOTR_NBSLOT_SHIFT: u32 = 8;
pub const SAI_XSLOTR_NBSLOT_MASK: u32 = genmask(11, SAI_XSLOTR_NBSLOT_SHIFT);
pub const fn SAI_XSLOTR_NBSLOT_SET(x: u32) -> u32 {
    x << SAI_XSLOTR_NBSLOT_SHIFT
}

pub const SAI_XSLOTR_SLOTEN_SHIFT: u32 = 16;
pub const SAI_XSLOTR_SLOTEN_WIDTH: u32 = 16;
pub const SAI_XSLOTR_SLOTEN_MASK: u32 = genmask(31, SAI_XSLOTR_SLOTEN_SHIFT);
pub const fn SAI_XSLOTR_SLOTEN_SET(x: u32) -> u32 {
    x << SAI_XSLOTR_SLOTEN_SHIFT
}

/******************* Bit definition for SAI_XIMR register *******************/
pub const SAI_XIMR_OVRUDRIE: u32 = bit(0);
pub const SAI_XIMR_MUTEDETIE: u32 = bit(1);
pub const SAI_XIMR_WCKCFGIE: u32 = bit(2);
pub const SAI_XIMR_FREQIE: u32 = bit(3);
pub const SAI_XIMR_CNRDYIE: u32 = bit(4);
pub const SAI_XIMR_AFSDETIE: u32 = bit(5);
pub const SAI_XIMR_LFSDETIE: u32 = bit(6);

pub const SAI_XIMR_SHIFT: u32 = 0;
pub const SAI_XIMR_MASK: u32 = genmask(6, SAI_XIMR_SHIFT);

/******************** Bit definition for SAI_XSR register *******************/
pub const SAI_XSR_OVRUDR: u32 = bit(0);
pub const SAI_XSR_MUTEDET: u32 = bit(1);
pub const SAI_XSR_WCKCFG: u32 = bit(2);
pub const SAI_XSR_FREQ: u32 = bit(3);
pub const SAI_XSR_CNRDY: u32 = bit(4);
pub const SAI_XSR_AFSDET: u32 = bit(5);
pub const SAI_XSR_LFSDET: u32 = bit(6);

pub const SAI_XSR_SHIFT: u32 = 0;
pub const SAI_XSR_MASK: u32 = genmask(6, SAI_XSR_SHIFT);

/****************** Bit definition for SAI_XCLRFR register ******************/
pub const SAI_XCLRFR_COVRUDR: u32 = bit(0);
pub const SAI_XCLRFR_CMUTEDET: u32 = bit(1);
pub const SAI_XCLRFR_CWCKCFG: u32 = bit(2);
pub const SAI_XCLRFR_CFREQ: u32 = bit(3);
pub const SAI_XCLRFR_CCNRDY: u32 = bit(4);
pub const SAI_XCLRFR_CAFSDET: u32 = bit(5);
pub const SAI_XCLRFR_CLFSDET: u32 = bit(6);

pub const SAI_XCLRFR_SHIFT: u32 = 0;
pub const SAI_XCLRFR_MASK: u32 = genmask(6, SAI_XCLRFR_SHIFT);

/****************** Bit definition for SAI_PDMCR register ******************/
pub const SAI_PDMCR_PDMEN: u32 = bit(0);

pub const SAI_PDMCR_MICNBR_SHIFT: u32 = 4;
pub const SAI_PDMCR_MICNBR_MASK: u32 = genmask(5, SAI_PDMCR_MICNBR_SHIFT);
pub const fn SAI_PDMCR_MICNBR_SET(x: u32) -> u32 {
    x << SAI_PDMCR_MICNBR_SHIFT
}

pub const SAI_PDMCR_CKEN1: u32 = bit(8);
pub const SAI_PDMCR_CKEN2: u32 = bit(9);
pub const SAI_PDMCR_CKEN3: u32 = bit(10);
pub const SAI_PDMCR_CKEN4: u32 = bit(11);

/****************** Bit definition for (SAI_PDMDLY register ****************/
pub const SAI_PDMDLY_1L_SHIFT: u32 = 0;
pub const SAI_PDMDLY_1L_MASK: u32 = genmask(2, SAI_PDMDLY_1L_SHIFT);
pub const SAI_PDMDLY_1L_WIDTH: u32 = 3;

pub const SAI_PDMDLY_1R_SHIFT: u32 = 4;
pub const SAI_PDMDLY_1R_MASK: u32 = genmask(6, SAI_PDMDLY_1R_SHIFT);
pub const SAI_PDMDLY_1R_WIDTH: u32 = 3;

pub const SAI_PDMDLY_2L_SHIFT: u32 = 8;
pub const SAI_PDMDLY_2L_MASK: u32 = genmask(10, SAI_PDMDLY_2L_SHIFT);
pub const SAI_PDMDLY_2L_WIDTH: u32 = 3;

pub const SAI_PDMDLY_2R_SHIFT: u32 = 12;
pub const SAI_PDMDLY_2R_MASK: u32 = genmask(14, SAI_PDMDLY_2R_SHIFT);
pub const SAI_PDMDLY_2R_WIDTH: u32 = 3;

pub const SAI_PDMDLY_3L_SHIFT: u32 = 16;
pub const SAI_PDMDLY_3L_MASK: u32 = genmask(18, SAI_PDMDLY_3L_SHIFT);
pub const SAI_PDMDLY_3L_WIDTH: u32 = 3;

pub const SAI_PDMDLY_3R_SHIFT: u32 = 20;
pub const SAI_PDMDLY_3R_MASK: u32 = genmask(22, SAI_PDMDLY_3R_SHIFT);
pub const SAI_PDMDLY_3R_WIDTH: u32 = 3;

pub const SAI_PDMDLY_4L_SHIFT: u32 = 24;
pub const SAI_PDMDLY_4L_MASK: u32 = genmask(26, SAI_PDMDLY_4L_SHIFT);
pub const SAI_PDMDLY_4L_WIDTH: u32 = 3;

pub const SAI_PDMDLY_4R_SHIFT: u32 = 28;
pub const SAI_PDMDLY_4R_MASK: u32 = genmask(30, SAI_PDMDLY_4R_SHIFT);
pub const SAI_PDMDLY_4R_WIDTH: u32 = 3;

/* Registers below apply to SAI version 2.1 and more */

/* Bit definition for SAI_HWCFGR register */
pub const SAI_HWCFGR_FIFO_SIZE: u32 = genmask(7, 0);
pub const SAI_HWCFGR_SPDIF_PDM: u32 = genmask(11, 8);
pub const SAI_HWCFGR_REGOUT: u32 = genmask(19, 12);

/* Bit definition for SAI_VERR register */
pub const SAI_VERR_MIN_MASK: u32 = genmask(3, 0);
pub const SAI_VERR_MAJ_MASK: u32 = genmask(7, 4);

/* Bit definition for SAI_IDR register */
pub const SAI_IDR_ID_MASK: u32 = genmask(31, 0);

/* Bit definition for SAI_SIDR register */
pub const SAI_SIDR_ID_MASK: u32 = genmask(31, 0);

pub const SAI_IPIDR_NUMBER: u32 = 0x00130031;

/* SAI version numbers are 1.x for F4. Major version number set to 1 for F4 */
pub const STM_SAI_STM32F4: u32 = bit(4);
/* Dummy version number for H7 socs and next */
pub const STM_SAI_STM32H7: u32 = 0x0;

pub unsafe fn STM_SAI_IS_F4(ip: *const stm32_sai_data) -> bool {
    unsafe { (*ip).conf.version == STM_SAI_STM32F4 }
}

pub unsafe fn STM_SAI_HAS_SPDIF_PDM(ip: *const stm32_sai_data) -> bool {
    unsafe { (*(*ip).pdata).conf.has_spdif_pdm }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum stm32_sai_syncout {
    STM_SAI_SYNC_OUT_NONE,
    STM_SAI_SYNC_OUT_A,
    STM_SAI_SYNC_OUT_B,
}

/* External C dependency types: struct platform_device, struct clk, struct device_node. */

/**
 * struct stm32_sai_conf - SAI configuration
 * @get_sai_ck_parent: get parent clock of SAI kernel clock
 * @version: SAI version
 * @fifo_size: SAI fifo size as words number
 * @has_spdif_pdm: SAI S/PDIF and PDM features support flag
 * @no_dma_burst: Support only DMA single transfers if set
 */
#[repr(C)]
pub struct stm32_sai_conf {
    pub get_sai_ck_parent: Option<unsafe extern "C" fn(sai: *mut stm32_sai_data) -> i32>,
    pub version: u32,
    pub fifo_size: u32,
    pub has_spdif_pdm: bool,
    pub no_dma_burst: bool,
}

/**
 * struct stm32_sai_data - private data of SAI instance driver
 * @pdev: device data pointer
 * @base: common register bank virtual base address
 * @pclk: SAI bus clock
 * @clk_x8k: SAI parent clock for sampling frequencies multiple of 8kHz
 * @clk_x11k: SAI parent clock for sampling frequencies multiple of 11kHz
 * @conf: SAI hardware capabitilites
 * @irq: SAI interrupt line
 * @set_sync: pointer to synchro mode configuration callback
 * @gcr: SAI Global Configuration Register
 */
#[repr(C)]
pub struct stm32_sai_data {
    pub pdev: *mut platform_device,
    pub base: *mut core::ffi::c_void,
    pub pclk: *mut clk,
    pub clk_x8k: *mut clk,
    pub clk_x11k: *mut clk,
    pub conf: stm32_sai_conf,
    pub irq: i32,
    pub set_sync: Option<
        unsafe extern "C" fn(
            sai: *mut stm32_sai_data,
            np_provider: *mut device_node,
            synco: i32,
            synci: i32,
        ) -> i32,
    >,
    pub gcr: u32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
