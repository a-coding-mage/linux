// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC CS4349 codec driver
 *
 * Copyright 2015 Cirrus Logic, Inc.
 *
 * Author: Tim Howe <Tim.Howe@cirrus.com>
 */

/* CS4349 registers addresses */
pub const CS4349_CHIPID: u8 = 0x01; /* Device and Rev ID, Read Only */
pub const CS4349_MODE: u8 = 0x02; /* Mode Control */
pub const CS4349_VMI: u8 = 0x03; /* Volume, Mixing, Inversion Control */
pub const CS4349_MUTE: u8 = 0x04; /* Mute Control */
pub const CS4349_VOLA: u8 = 0x05; /* DAC Channel A Volume Control */
pub const CS4349_VOLB: u8 = 0x06; /* DAC Channel B Volume Control */
pub const CS4349_RMPFLT: u8 = 0x07; /* Ramp and Filter Control */
pub const CS4349_MISC: u8 = 0x08; /* Power Down,Freeze Control,Pop Stop*/

pub const CS4349_I2C_INCR: u8 = 0x80;

/* Device and Revision ID */
pub const CS4349_REVA: u8 = 0xF0; /* Rev A */
pub const CS4349_REVB: u8 = 0xF1; /* Rev B */
pub const CS4349_REVC2: u8 = 0xFF; /* Rev C2 */

/*
 * PDN_DONE Poll Maximum
 * If soft ramp is set it will take much longer to power down
 * the system.
 */
pub const PDN_POLL_MAX: u32 = 900;

/* Bitfield Definitions */

/* CS4349_MODE */
/* (Digital Interface Format, De-Emphasis Control, Functional Mode */
pub const DIF2: u8 = 1 << 6;
pub const DIF1: u8 = 1 << 5;
pub const DIF0: u8 = 1 << 4;
pub const DEM1: u8 = 1 << 3;
pub const DEM0: u8 = 1 << 2;
pub const FM1: u8 = 1 << 1;
pub const DIF_LEFT_JST: u8 = 0x00;
pub const DIF_I2S: u8 = 0x01;
pub const DIF_RGHT_JST16: u8 = 0x02;
pub const DIF_RGHT_JST24: u8 = 0x03;
pub const DIF_TDM0: u8 = 0x04;
pub const DIF_TDM1: u8 = 0x05;
pub const DIF_TDM2: u8 = 0x06;
pub const DIF_TDM3: u8 = 0x07;
pub const DIF_MASK: u8 = 0x70;
pub const fn MODE_FORMAT(x: u8) -> u8 {
    (x & 7) << 4
}
pub const DEM_MASK: u8 = 0x0C;
pub const NO_DEM: u8 = 0x00;
pub const DEM_441: u8 = 0x04;
pub const DEM_48K: u8 = 0x08;
pub const DEM_32K: u8 = 0x0C;
pub const FM_AUTO: u8 = 0x00;
pub const FM_SNGL: u8 = 0x01;
pub const FM_DBL: u8 = 0x02;
pub const FM_QUAD: u8 = 0x03;
pub const FM_SNGL_MIN: u32 = 30000;
pub const FM_SNGL_MAX: u32 = 54000;
pub const FM_DBL_MAX: u32 = 108000;
pub const FM_QUAD_MAX: u32 = 216000;
pub const FM_MASK: u8 = 0x03;

/* CS4349_VMI (VMI = Volume, Mixing and Inversion Controls) */
pub const VOLBISA: u8 = 1 << 7;
pub const VOLAISB: u8 = 1 << 7;
/* INVERT_A only available for Left Jstfd, Right Jstfd16 and Right Jstfd24 */
pub const INVERT_A: u8 = 1 << 6;
/* INVERT_B only available for Left Jstfd, Right Jstfd16 and Right Jstfd24 */
pub const INVERT_B: u8 = 1 << 5;
pub const ATAPI3: u8 = 1 << 3;
pub const ATAPI2: u8 = 1 << 2;
pub const ATAPI1: u8 = 1 << 1;
pub const ATAPI0: u8 = 1 << 0;
pub const MUTEAB: u8 = 0x00;
pub const MUTEA_RIGHTB: u8 = 0x01;
pub const MUTEA_LEFTB: u8 = 0x02;
pub const MUTEA_SUMLRDIV2B: u8 = 0x03;
pub const RIGHTA_MUTEB: u8 = 0x04;
pub const RIGHTA_RIGHTB: u8 = 0x05;
pub const RIGHTA_LEFTB: u8 = 0x06;
pub const RIGHTA_SUMLRDIV2B: u8 = 0x07;
pub const LEFTA_MUTEB: u8 = 0x08;
pub const LEFTA_RIGHTB: u8 = 0x09; /* Default */
pub const LEFTA_LEFTB: u8 = 0x0A;
pub const LEFTA_SUMLRDIV2B: u8 = 0x0B;
pub const SUMLRDIV2A_MUTEB: u8 = 0x0C;
pub const SUMLRDIV2A_RIGHTB: u8 = 0x0D;
pub const SUMLRDIV2A_LEFTB: u8 = 0x0E;
pub const SUMLRDIV2_AB: u8 = 0x0F;
pub const CHMIX_MASK: u8 = 0x0F;

/* CS4349_MUTE */
pub const AUTOMUTE: u8 = 1 << 7;
pub const MUTEC_AB: u8 = 1 << 5;
pub const MUTE_A: u8 = 1 << 4;
pub const MUTE_B: u8 = 1 << 3;
pub const MUTE_AB_MASK: u8 = 0x18;

/* CS4349_RMPFLT (Ramp and Filter Control) */
pub const SCZ1: u8 = 1 << 7;
pub const SCZ0: u8 = 1 << 6;
pub const RMP_UP: u8 = 1 << 5;
pub const RMP_DN: u8 = 1 << 4;
pub const FILT_SEL: u8 = 1 << 2;
pub const IMMDT_CHNG: u8 = 0x31;
pub const ZEROCRSS: u8 = 0x71;
pub const SOFT_RMP: u8 = 0xB1;
pub const SFTRMP_ZEROCRSS: u8 = 0xF1;
pub const SR_ZC_MASK: u8 = 0xC0;

/* CS4349_MISC */
pub const PWR_DWN: u8 = 1 << 7;
pub const FREEZE: u8 = 1 << 5;
pub const POPG_EN: u8 = 1 << 4;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
