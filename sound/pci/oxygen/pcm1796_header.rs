// SPDX-License-Identifier: GPL-2.0

/* register 16 */
pub const PCM1796_ATL_MASK: u32 = 0xff;
/* register 17 */
pub const PCM1796_ATR_MASK: u32 = 0xff;
/* register 18 */
pub const PCM1796_MUTE: u32 = 0x01;
pub const PCM1796_DME: u32 = 0x02;
pub const PCM1796_DMF_MASK: u32 = 0x0c;
pub const PCM1796_DMF_48: u32 = 0x04;
pub const PCM1796_DMF_441: u32 = 0x08;
pub const PCM1796_DMF_32: u32 = 0x0c;
pub const PCM1796_FMT_MASK: u32 = 0x70;
pub const PCM1796_FMT_16_RJUST: u32 = 0x00;
pub const PCM1796_FMT_20_RJUST: u32 = 0x10;
pub const PCM1796_FMT_24_RJUST: u32 = 0x20;
pub const PCM1796_FMT_24_LJUST: u32 = 0x30;
pub const PCM1796_FMT_16_I2S: u32 = 0x40;
pub const PCM1796_FMT_24_I2S: u32 = 0x50;
pub const PCM1796_ATLD: u32 = 0x80;
/* register 19 */
pub const PCM1796_INZD: u32 = 0x01;
pub const PCM1796_FLT_MASK: u32 = 0x02;
pub const PCM1796_FLT_SHARP: u32 = 0x00;
pub const PCM1796_FLT_SLOW: u32 = 0x02;
pub const PCM1796_DFMS: u32 = 0x04;
pub const PCM1796_OPE: u32 = 0x10;
pub const PCM1796_ATS_MASK: u32 = 0x60;
pub const PCM1796_ATS_1: u32 = 0x00;
pub const PCM1796_ATS_2: u32 = 0x20;
pub const PCM1796_ATS_4: u32 = 0x40;
pub const PCM1796_ATS_8: u32 = 0x60;
pub const PCM1796_REV: u32 = 0x80;
/* register 20 */
pub const PCM1796_OS_MASK: u32 = 0x03;
pub const PCM1796_OS_64: u32 = 0x00;
pub const PCM1796_OS_32: u32 = 0x01;
pub const PCM1796_OS_128: u32 = 0x02;
pub const PCM1796_CHSL_MASK: u32 = 0x04;
pub const PCM1796_CHSL_LEFT: u32 = 0x00;
pub const PCM1796_CHSL_RIGHT: u32 = 0x04;
pub const PCM1796_MONO: u32 = 0x08;
pub const PCM1796_DFTH: u32 = 0x10;
pub const PCM1796_DSD: u32 = 0x20;
pub const PCM1796_SRST: u32 = 0x40;
/* register 21 */
pub const PCM1796_PCMZ: u32 = 0x01;
pub const PCM1796_DZ_MASK: u32 = 0x06;
/* register 22 */
pub const PCM1796_ZFGL: u32 = 0x01;
pub const PCM1796_ZFGR: u32 = 0x02;
/* register 23 */
pub const PCM1796_ID_MASK: u32 = 0x1f;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
