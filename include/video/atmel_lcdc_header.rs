/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Header file for AT91/AT32 LCD Controller
 *
 * Data structure and register user interface
 *
 * Copyright (C) 2007 Atmel Corporation
 */

// Dependency supplied by the surrounding kernel bindings.

/* Way LCD wires are connected to the chip:
 * Some Atmel chips use BGR color mode (instead of standard RGB)
 * A swapped wiring onboard can bring to RGB mode.
 */
pub const ATMEL_LCDC_WIRING_BGR: u32 = 0;
pub const ATMEL_LCDC_WIRING_RGB: u32 = 1;

/* LCD Controller info data structure, stored in device platform_data */
#[repr(C)]
pub struct atmel_lcdfb_pdata {
    pub guard_time: u32,
    pub lcdcon_is_backlight: bool,
    pub lcdcon_pol_negative: bool,
    pub default_bpp: u8,
    pub lcd_wiring_mode: u8,
    pub default_lcdcon2: u32,
    pub default_dmacon: u32,
    pub atmel_lcdfb_power_control:
        Option<unsafe extern "C" fn(pdata: *mut atmel_lcdfb_pdata, on: i32)>,
    pub default_monspecs: *mut fb_monspecs,
    pub pwr_gpios: list_head,
}

pub const ATMEL_LCDC_DMABADDR1: u32 = 0x00;
pub const ATMEL_LCDC_DMABADDR2: u32 = 0x04;
pub const ATMEL_LCDC_DMAFRMPT1: u32 = 0x08;
pub const ATMEL_LCDC_DMAFRMPT2: u32 = 0x0c;
pub const ATMEL_LCDC_DMAFRMADD1: u32 = 0x10;
pub const ATMEL_LCDC_DMAFRMADD2: u32 = 0x14;
pub const ATMEL_LCDC_DMAFRMCFG: u32 = 0x18;
pub const ATMEL_LCDC_FRSIZE: u32 = 0x7fffff << 0;
pub const ATMEL_LCDC_BLENGTH_OFFSET: u32 = 24;
pub const ATMEL_LCDC_BLENGTH: u32 = 0x7f << ATMEL_LCDC_BLENGTH_OFFSET;
pub const ATMEL_LCDC_DMACON: u32 = 0x1c;
pub const ATMEL_LCDC_DMAEN: u32 = 1 << 0;
pub const ATMEL_LCDC_DMARST: u32 = 1 << 1;
pub const ATMEL_LCDC_DMABUSY: u32 = 1 << 2;
pub const ATMEL_LCDC_DMAUPDT: u32 = 1 << 3;
pub const ATMEL_LCDC_DMA2DEN: u32 = 1 << 4;
pub const ATMEL_LCDC_DMA2DCFG: u32 = 0x20;
pub const ATMEL_LCDC_ADDRINC_OFFSET: u32 = 0;
pub const ATMEL_LCDC_ADDRINC: u32 = 0xffff;
pub const ATMEL_LCDC_PIXELOFF_OFFSET: u32 = 24;
pub const ATMEL_LCDC_PIXELOFF: u32 = 0x1f << 24;
pub const ATMEL_LCDC_LCDCON1: u32 = 0x0800;
pub const ATMEL_LCDC_BYPASS: u32 = 1 << 0;
pub const ATMEL_LCDC_CLKVAL_OFFSET: u32 = 12;
pub const ATMEL_LCDC_CLKVAL: u32 = 0x1ff << ATMEL_LCDC_CLKVAL_OFFSET;
pub const ATMEL_LCDC_LINCNT: u32 = 0x7ff << 21;
pub const ATMEL_LCDC_LCDCON2: u32 = 0x0804;
pub const ATMEL_LCDC_DISTYPE: u32 = 3 << 0;
pub const ATMEL_LCDC_DISTYPE_STNMONO: u32 = 0 << 0;
pub const ATMEL_LCDC_DISTYPE_STNCOLOR: u32 = 1 << 0;
pub const ATMEL_LCDC_DISTYPE_TFT: u32 = 2 << 0;
pub const ATMEL_LCDC_SCANMOD: u32 = 1 << 2;
pub const ATMEL_LCDC_SCANMOD_SINGLE: u32 = 0 << 2;
pub const ATMEL_LCDC_SCANMOD_DUAL: u32 = 1 << 2;
pub const ATMEL_LCDC_IFWIDTH: u32 = 3 << 3;
pub const ATMEL_LCDC_IFWIDTH_4: u32 = 0 << 3;
pub const ATMEL_LCDC_IFWIDTH_8: u32 = 1 << 3;
pub const ATMEL_LCDC_IFWIDTH_16: u32 = 2 << 3;
pub const ATMEL_LCDC_PIXELSIZE: u32 = 7 << 5;
pub const ATMEL_LCDC_PIXELSIZE_1: u32 = 0 << 5;
pub const ATMEL_LCDC_PIXELSIZE_2: u32 = 1 << 5;
pub const ATMEL_LCDC_PIXELSIZE_4: u32 = 2 << 5;
pub const ATMEL_LCDC_PIXELSIZE_8: u32 = 3 << 5;
pub const ATMEL_LCDC_PIXELSIZE_16: u32 = 4 << 5;
pub const ATMEL_LCDC_PIXELSIZE_24: u32 = 5 << 5;
pub const ATMEL_LCDC_PIXELSIZE_32: u32 = 6 << 5;
pub const ATMEL_LCDC_INVVD: u32 = 1 << 8;
pub const ATMEL_LCDC_INVVD_NORMAL: u32 = 0 << 8;
pub const ATMEL_LCDC_INVVD_INVERTED: u32 = 1 << 8;
pub const ATMEL_LCDC_INVFRAME: u32 = 1 << 9;
pub const ATMEL_LCDC_INVFRAME_NORMAL: u32 = 0 << 9;
pub const ATMEL_LCDC_INVFRAME_INVERTED: u32 = 1 << 9;
pub const ATMEL_LCDC_INVLINE: u32 = 1 << 10;
pub const ATMEL_LCDC_INVLINE_NORMAL: u32 = 0 << 10;
pub const ATMEL_LCDC_INVLINE_INVERTED: u32 = 1 << 10;
pub const ATMEL_LCDC_INVCLK: u32 = 1 << 11;
pub const ATMEL_LCDC_INVCLK_NORMAL: u32 = 0 << 11;
pub const ATMEL_LCDC_INVCLK_INVERTED: u32 = 1 << 11;
pub const ATMEL_LCDC_INVDVAL: u32 = 1 << 12;
pub const ATMEL_LCDC_INVDVAL_NORMAL: u32 = 0 << 12;
pub const ATMEL_LCDC_INVDVAL_INVERTED: u32 = 1 << 12;
pub const ATMEL_LCDC_CLKMOD: u32 = 1 << 15;
pub const ATMEL_LCDC_CLKMOD_ACTIVEDISPLAY: u32 = 0 << 15;
pub const ATMEL_LCDC_CLKMOD_ALWAYSACTIVE: u32 = 1 << 15;
pub const ATMEL_LCDC_MEMOR: u32 = 1 << 31;
pub const ATMEL_LCDC_MEMOR_BIG: u32 = 0 << 31;
pub const ATMEL_LCDC_MEMOR_LITTLE: u32 = 1 << 31;

pub const ATMEL_LCDC_TIM1: u32 = 0x0808;
pub const ATMEL_LCDC_VFP: u32 = 0xff << 0;
pub const ATMEL_LCDC_VBP_OFFSET: u32 = 8;
pub const ATMEL_LCDC_VBP: u32 = 0xff << ATMEL_LCDC_VBP_OFFSET;
pub const ATMEL_LCDC_VPW_OFFSET: u32 = 16;
pub const ATMEL_LCDC_VPW: u32 = 0x3f << ATMEL_LCDC_VPW_OFFSET;
pub const ATMEL_LCDC_VHDLY_OFFSET: u32 = 24;
pub const ATMEL_LCDC_VHDLY: u32 = 0xf << ATMEL_LCDC_VHDLY_OFFSET;
pub const ATMEL_LCDC_TIM2: u32 = 0x080c;
pub const ATMEL_LCDC_HBP: u32 = 0xff;
pub const ATMEL_LCDC_HPW_OFFSET: u32 = 8;
pub const ATMEL_LCDC_HPW: u32 = 0x3f << ATMEL_LCDC_HPW_OFFSET;
pub const ATMEL_LCDC_HFP_OFFSET: u32 = 21;
pub const ATMEL_LCDC_HFP: u32 = 0x7ff << ATMEL_LCDC_HFP_OFFSET;
pub const ATMEL_LCDC_LCDFRMCFG: u32 = 0x0810;
pub const ATMEL_LCDC_LINEVAL: u32 = 0x7ff;
pub const ATMEL_LCDC_HOZVAL_OFFSET: u32 = 21;
pub const ATMEL_LCDC_HOZVAL: u32 = 0x7ff << ATMEL_LCDC_HOZVAL_OFFSET;
pub const ATMEL_LCDC_FIFO: u32 = 0x0814;
pub const ATMEL_LCDC_FIFOTH: u32 = 0xffff;
pub const ATMEL_LCDC_MVAL: u32 = 0x0818;
pub const ATMEL_LCDC_DP1_2: u32 = 0x081c;
pub const ATMEL_LCDC_DP4_7: u32 = 0x0820;
pub const ATMEL_LCDC_DP3_5: u32 = 0x0824;
pub const ATMEL_LCDC_DP2_3: u32 = 0x0828;
pub const ATMEL_LCDC_DP5_7: u32 = 0x082c;
pub const ATMEL_LCDC_DP3_4: u32 = 0x0830;
pub const ATMEL_LCDC_DP4_5: u32 = 0x0834;
pub const ATMEL_LCDC_DP6_7: u32 = 0x0838;
pub const ATMEL_LCDC_DP1_2_VAL: u32 = 0xff;
pub const ATMEL_LCDC_DP4_7_VAL: u32 = 0xfffffff;
pub const ATMEL_LCDC_DP3_5_VAL: u32 = 0xfffff;
pub const ATMEL_LCDC_DP2_3_VAL: u32 = 0xfff;
pub const ATMEL_LCDC_DP5_7_VAL: u32 = 0xfffffff;
pub const ATMEL_LCDC_DP3_4_VAL: u32 = 0xffff;
pub const ATMEL_LCDC_DP4_5_VAL: u32 = 0xfffff;
pub const ATMEL_LCDC_DP6_7_VAL: u32 = 0xfffffff;
pub const ATMEL_LCDC_PWRCON: u32 = 0x083c;
pub const ATMEL_LCDC_PWR: u32 = 1 << 0;
pub const ATMEL_LCDC_GUARDT_OFFSET: u32 = 1;
pub const ATMEL_LCDC_GUARDT: u32 = 0x7f << ATMEL_LCDC_GUARDT_OFFSET;
pub const ATMEL_LCDC_BUSY: u32 = 1 << 31;
pub const ATMEL_LCDC_CONTRAST_CTR: u32 = 0x0840;
pub const ATMEL_LCDC_PS: u32 = 3;
pub const ATMEL_LCDC_PS_DIV1: u32 = 0;
pub const ATMEL_LCDC_PS_DIV2: u32 = 1;
pub const ATMEL_LCDC_PS_DIV4: u32 = 2;
pub const ATMEL_LCDC_PS_DIV8: u32 = 3;
pub const ATMEL_LCDC_POL: u32 = 1 << 2;
pub const ATMEL_LCDC_POL_NEGATIVE: u32 = 0 << 2;
pub const ATMEL_LCDC_POL_POSITIVE: u32 = 1 << 2;
pub const ATMEL_LCDC_ENA: u32 = 1 << 3;
pub const ATMEL_LCDC_ENA_PWMDISABLE: u32 = 0 << 3;
pub const ATMEL_LCDC_ENA_PWMENABLE: u32 = 1 << 3;
pub const ATMEL_LCDC_CONTRAST_VAL: u32 = 0x0844;
pub const ATMEL_LCDC_CVAL: u32 = 0xff;
pub const ATMEL_LCDC_IER: u32 = 0x0848;
pub const ATMEL_LCDC_IDR: u32 = 0x084c;
pub const ATMEL_LCDC_IMR: u32 = 0x0850;
pub const ATMEL_LCDC_ISR: u32 = 0x0854;
pub const ATMEL_LCDC_ICR: u32 = 0x0858;
pub const ATMEL_LCDC_LNI: u32 = 1 << 0;
pub const ATMEL_LCDC_LSTLNI: u32 = 1 << 1;
pub const ATMEL_LCDC_EOFI: u32 = 1 << 2;
pub const ATMEL_LCDC_UFLWI: u32 = 1 << 4;
pub const ATMEL_LCDC_OWRI: u32 = 1 << 5;
pub const ATMEL_LCDC_MERI: u32 = 1 << 6;

pub const fn ATMEL_LCDC_LUT(n: u32) -> u32 { 0x0c00 + n * 4 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
