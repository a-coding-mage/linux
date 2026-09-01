// SPDX-License-Identifier: GPL-2.0

// register 1
pub const CS4398_REV_MASK: u32 = 0x07;
pub const CS4398_PART_MASK: u32 = 0xf8;
pub const CS4398_PART_CS4398: u32 = 0x70;

// register 2
pub const CS4398_FM_MASK: u32 = 0x03;
pub const CS4398_FM_SINGLE: u32 = 0x00;
pub const CS4398_FM_DOUBLE: u32 = 0x01;
pub const CS4398_FM_QUAD: u32 = 0x02;
pub const CS4398_FM_DSD: u32 = 0x03;
pub const CS4398_DEM_MASK: u32 = 0x0c;
pub const CS4398_DEM_NONE: u32 = 0x00;
pub const CS4398_DEM_44100: u32 = 0x04;
pub const CS4398_DEM_48000: u32 = 0x08;
pub const CS4398_DEM_32000: u32 = 0x0c;
pub const CS4398_DIF_MASK: u32 = 0x70;
pub const CS4398_DIF_LJUST: u32 = 0x00;
pub const CS4398_DIF_I2S: u32 = 0x10;
pub const CS4398_DIF_RJUST_16: u32 = 0x20;
pub const CS4398_DIF_RJUST_24: u32 = 0x30;
pub const CS4398_DIF_RJUST_20: u32 = 0x40;
pub const CS4398_DIF_RJUST_18: u32 = 0x50;
pub const CS4398_DSD_SRC: u32 = 0x80;

// register 3
pub const CS4398_ATAPI_MASK: u32 = 0x1f;
pub const CS4398_ATAPI_B_MUTE: u32 = 0x00;
pub const CS4398_ATAPI_B_R: u32 = 0x01;
pub const CS4398_ATAPI_B_L: u32 = 0x02;
pub const CS4398_ATAPI_B_LR: u32 = 0x03;
pub const CS4398_ATAPI_A_MUTE: u32 = 0x00;
pub const CS4398_ATAPI_A_R: u32 = 0x04;
pub const CS4398_ATAPI_A_L: u32 = 0x08;
pub const CS4398_ATAPI_A_LR: u32 = 0x0c;
pub const CS4398_ATAPI_MIX_LR_VOL: u32 = 0x10;
pub const CS4398_INVERT_B: u32 = 0x20;
pub const CS4398_INVERT_A: u32 = 0x40;
pub const CS4398_VOL_B_EQ_A: u32 = 0x80;

// register 4
pub const CS4398_MUTEP_MASK: u32 = 0x03;
pub const CS4398_MUTEP_AUTO: u32 = 0x00;
pub const CS4398_MUTEP_LOW: u32 = 0x02;
pub const CS4398_MUTEP_HIGH: u32 = 0x03;
pub const CS4398_MUTE_B: u32 = 0x08;
pub const CS4398_MUTE_A: u32 = 0x10;
pub const CS4398_MUTEC_A_EQ_B: u32 = 0x20;
pub const CS4398_DAMUTE: u32 = 0x40;
pub const CS4398_PAMUTE: u32 = 0x80;

// register 5
pub const CS4398_VOL_A_MASK: u32 = 0xff;

// register 6
pub const CS4398_VOL_B_MASK: u32 = 0xff;

// register 7
pub const CS4398_DIR_DSD: u32 = 0x01;
pub const CS4398_FILT_SEL: u32 = 0x04;
pub const CS4398_RMP_DN: u32 = 0x10;
pub const CS4398_RMP_UP: u32 = 0x20;
pub const CS4398_ZERO_CROSS: u32 = 0x40;
pub const CS4398_SOFT_RAMP: u32 = 0x80;

// register 8
pub const CS4398_MCLKDIV3: u32 = 0x08;
pub const CS4398_MCLKDIV2: u32 = 0x10;
pub const CS4398_FREEZE: u32 = 0x20;
pub const CS4398_CPEN: u32 = 0x40;
pub const CS4398_PDN: u32 = 0x80;

// register 9
pub const CS4398_DSD_PM_EN: u32 = 0x01;
pub const CS4398_DSD_PM_MODE: u32 = 0x02;
pub const CS4398_INVALID_DSD: u32 = 0x04;
pub const CS4398_STATIC_DSD: u32 = 0x08;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
