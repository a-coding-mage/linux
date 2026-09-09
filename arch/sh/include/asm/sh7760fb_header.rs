/* SPDX-License-Identifier: GPL-2.0 */
/*
 * sh7760fb.h -- platform data for SH7760/SH7763 LCDC framebuffer driver.
 *
 * (c) 2006-2008 MSC Vertriebsges.m.b.H.,
 *              Manuel Lauss <mano@roarinelk.homelinux.net>
 * (c) 2008 Nobuhiro Iwamatsu <iwamatsu.nobuhiro@renesas.com>
 */

/*
 * Some bits of the colormap registers should be written as zero.
 * Create a mask for that.
 */
pub const SH7760FB_PALETTE_MASK: u32 = 0x00f8fcf8;

/* The LCDC dma engine always sets bits 27-26 to 1: this is Area3 */
pub const SH7760FB_DMA_MASK: u32 = 0x0C000000;

/* palette */
#[inline]
pub const fn LDPR(x: u32) -> u32 { x << 2 }

/* framebuffer registers and bits */
pub const LDICKR: u32 = 0x400;
pub const LDMTR: u32 = 0x402;
/* see sh7760fb.h for LDMTR bits */
pub const LDDFR: u32 = 0x404;
pub const LDDFR_PABD: u32 = 1 << 8;
pub const LDDFR_COLOR_MASK: u32 = 0x7F;
pub const LDSMR: u32 = 0x406;
pub const LDSMR_ROT: u32 = 1 << 13;
pub const LDSARU: u32 = 0x408;
pub const LDSARL: u32 = 0x40c;
pub const LDLAOR: u32 = 0x410;
pub const LDPALCR: u32 = 0x412;
pub const LDPALCR_PALS: u32 = 1 << 4;
pub const LDPALCR_PALEN: u32 = 1 << 0;
pub const LDHCNR: u32 = 0x414;
pub const LDHSYNR: u32 = 0x416;
pub const LDVDLNR: u32 = 0x418;
pub const LDVTLNR: u32 = 0x41a;
pub const LDVSYNR: u32 = 0x41c;
pub const LDACLNR: u32 = 0x41e;
pub const LDINTR: u32 = 0x420;
pub const LDPMMR: u32 = 0x424;
pub const LDPSPR: u32 = 0x426;
pub const LDCNTR: u32 = 0x428;
pub const LDCNTR_DON: u32 = 1 << 0;
pub const LDCNTR_DON2: u32 = 1 << 4;

#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const LDLIRNR: u32 = 0x440;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const LDINTR_MINTEN: u32 = 1 << 15;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const LDINTR_FINTEN: u32 = 1 << 14;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const LDINTR_VSINTEN: u32 = 1 << 13;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const LDINTR_VEINTEN: u32 = 1 << 12;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const LDINTR_MINTS: u32 = 1 << 11;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const LDINTR_FINTS: u32 = 1 << 10;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const LDINTR_VSINTS: u32 = 1 << 9;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const LDINTR_VEINTS: u32 = 1 << 8;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const VINT_START: u32 = LDINTR_VSINTEN;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const VINT_CHECK: u32 = LDINTR_VSINTS;

#[cfg(not(feature = "CONFIG_CPU_SUBTYPE_SH7763"))]
pub const LDINTR_VINTSEL: u32 = 1 << 12;
#[cfg(not(feature = "CONFIG_CPU_SUBTYPE_SH7763"))]
pub const LDINTR_VINTE: u32 = 1 << 8;
#[cfg(not(feature = "CONFIG_CPU_SUBTYPE_SH7763"))]
pub const LDINTR_VINTS: u32 = 1 << 0;
#[cfg(not(feature = "CONFIG_CPU_SUBTYPE_SH7763"))]
pub const VINT_START: u32 = LDINTR_VINTSEL;
#[cfg(not(feature = "CONFIG_CPU_SUBTYPE_SH7763"))]
pub const VINT_CHECK: u32 = LDINTR_VINTS;

/* HSYNC polarity inversion */
pub const LDMTR_FLMPOL: u32 = 1 << 15;
/* VSYNC polarity inversion */
pub const LDMTR_CL1POL: u32 = 1 << 14;
/* DISPLAY-ENABLE polarity inversion */
pub const LDMTR_DISPEN_LOWACT: u32 = 1 << 13;
/* DISPLAY DATA BUS polarity inversion */
pub const LDMTR_DPOL_LOWACT: u32 = 1 << 12;
/* AC modulation signal enable */
pub const LDMTR_MCNT: u32 = 1 << 10;
/* Disable output of HSYNC during VSYNC period */
pub const LDMTR_CL1CNT: u32 = 1 << 9;
/* Disable output of VSYNC during VSYNC period */
pub const LDMTR_CL2CNT: u32 = 1 << 8;

/* Display types supported by the LCDC */
pub const LDMTR_STN_MONO_4: u32 = 0x00;
pub const LDMTR_STN_MONO_8: u32 = 0x01;
pub const LDMTR_STN_COLOR_4: u32 = 0x08;
pub const LDMTR_STN_COLOR_8: u32 = 0x09;
pub const LDMTR_STN_COLOR_12: u32 = 0x0A;
pub const LDMTR_STN_COLOR_16: u32 = 0x0B;
pub const LDMTR_DSTN_MONO_8: u32 = 0x11;
pub const LDMTR_DSTN_MONO_16: u32 = 0x13;
pub const LDMTR_DSTN_COLOR_8: u32 = 0x19;
pub const LDMTR_DSTN_COLOR_12: u32 = 0x1A;
pub const LDMTR_DSTN_COLOR_16: u32 = 0x1B;
pub const LDMTR_TFT_COLOR_16: u32 = 0x2B;

/* framebuffer color layout */
pub const LDDFR_1BPP_MONO: u32 = 0x00;
pub const LDDFR_2BPP_MONO: u32 = 0x01;
pub const LDDFR_4BPP_MONO: u32 = 0x02;
pub const LDDFR_6BPP_MONO: u32 = 0x04;
pub const LDDFR_4BPP: u32 = 0x0A;
pub const LDDFR_8BPP: u32 = 0x0C;
pub const LDDFR_16BPP_RGB555: u32 = 0x1D;
pub const LDDFR_16BPP_RGB565: u32 = 0x2D;

/* LCDC Pixclock sources */
pub const LCDC_CLKSRC_BUSCLOCK: u32 = 0;
pub const LCDC_CLKSRC_PERIPHERAL: u32 = 1;
pub const LCDC_CLKSRC_EXTERNAL: u32 = 2;

#[inline]
pub const fn LDICKR_CLKSRC(x: u32) -> u32 { (x & 3) << 12 }
#[inline]
pub const fn LDICKR_CLKDIV(x: u32) -> u32 { x & 0x1f }

#[repr(C)]
pub struct sh7760fb_platdata {
    pub def_mode: *mut fb_videomode,
    pub ldmtr: u16,
    pub lddfr: u16,
    pub ldpmmr: u16,
    pub ldpspr: u16,
    pub ldaclnr: u16,
    pub ldickr: u16,
    pub rotate: i32,
    pub novsync: i32,
    pub blank: Option<unsafe extern "C" fn(i32)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
