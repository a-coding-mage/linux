/* SPDX-License-Identifier: GPL-2.0 */
/* register 01h */
pub const CS4362A_PDN: u8 = 0x01;
pub const CS4362A_DAC1_DIS: u8 = 0x02;
pub const CS4362A_DAC2_DIS: u8 = 0x04;
pub const CS4362A_DAC3_DIS: u8 = 0x08;
pub const CS4362A_MCLKDIV: u8 = 0x20;
pub const CS4362A_FREEZE: u8 = 0x40;
pub const CS4362A_CPEN: u8 = 0x80;
/* register 02h */
pub const CS4362A_DIF_MASK: u8 = 0x70;
pub const CS4362A_DIF_LJUST: u8 = 0x00;
pub const CS4362A_DIF_I2S: u8 = 0x10;
pub const CS4362A_DIF_RJUST_16: u8 = 0x20;
pub const CS4362A_DIF_RJUST_24: u8 = 0x30;
pub const CS4362A_DIF_RJUST_20: u8 = 0x40;
pub const CS4362A_DIF_RJUST_18: u8 = 0x50;
/* register 03h */
pub const CS4362A_MUTEC_MASK: u8 = 0x03;
pub const CS4362A_MUTEC_6: u8 = 0x00;
pub const CS4362A_MUTEC_1: u8 = 0x01;
pub const CS4362A_MUTEC_3: u8 = 0x03;
pub const CS4362A_AMUTE: u8 = 0x04;
pub const CS4362A_MUTEC_POL: u8 = 0x08;
pub const CS4362A_RMP_UP: u8 = 0x10;
pub const CS4362A_SNGLVOL: u8 = 0x20;
pub const CS4362A_ZERO_CROSS: u8 = 0x40;
pub const CS4362A_SOFT_RAMP: u8 = 0x80;
/* register 04h */
pub const CS4362A_RMP_DN: u8 = 0x01;
pub const CS4362A_DEM_MASK: u8 = 0x06;
pub const CS4362A_DEM_NONE: u8 = 0x00;
pub const CS4362A_DEM_44100: u8 = 0x02;
pub const CS4362A_DEM_48000: u8 = 0x04;
pub const CS4362A_DEM_32000: u8 = 0x06;
pub const CS4362A_FILT_SEL: u8 = 0x10;
/* register 05h */
pub const CS4362A_INV_A1: u8 = 0x01;
pub const CS4362A_INV_B1: u8 = 0x02;
pub const CS4362A_INV_A2: u8 = 0x04;
pub const CS4362A_INV_B2: u8 = 0x08;
pub const CS4362A_INV_A3: u8 = 0x10;
pub const CS4362A_INV_B3: u8 = 0x20;
/* register 06h */
pub const CS4362A_FM_MASK: u8 = 0x03;
pub const CS4362A_FM_SINGLE: u8 = 0x00;
pub const CS4362A_FM_DOUBLE: u8 = 0x01;
pub const CS4362A_FM_QUAD: u8 = 0x02;
pub const CS4362A_FM_DSD: u8 = 0x03;
pub const CS4362A_ATAPI_MASK: u8 = 0x7c;
pub const CS4362A_ATAPI_B_MUTE: u8 = 0x00;
pub const CS4362A_ATAPI_B_R: u8 = 0x04;
pub const CS4362A_ATAPI_B_L: u8 = 0x08;
pub const CS4362A_ATAPI_B_LR: u8 = 0x0c;
pub const CS4362A_ATAPI_A_MUTE: u8 = 0x00;
pub const CS4362A_ATAPI_A_R: u8 = 0x10;
pub const CS4362A_ATAPI_A_L: u8 = 0x20;
pub const CS4362A_ATAPI_A_LR: u8 = 0x30;
pub const CS4362A_ATAPI_MIX_LR_VOL: u8 = 0x40;
pub const CS4362A_A_EQ_B: u8 = 0x80;
/* register 07h */
pub const CS4362A_VOL_MASK: u8 = 0x7f;
pub const CS4362A_MUTE: u8 = 0x80;
/* register 08h: like 07h */
/* registers 09h..0Bh: like 06h..08h */
/* registers 0Ch..0Eh: like 06h..08h */
/* register 12h */
pub const CS4362A_REV_MASK: u8 = 0x07;
pub const CS4362A_PART_MASK: u8 = 0xf8;
pub const CS4362A_PART_CS4362A: u8 = 0x50;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
