// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Aztech AZT2316 Driver
 * Copyright (C) 2007,2010  Rene Herman
 */

pub const AZT2316: bool = true;

pub const CRD_NAME: &str = "Aztech AZT2316";
pub const DRV_NAME: &str = "AZT2316";
pub const DEV_NAME: &str = "azt2316";

pub const GALAXY_DSP_MAJOR: u32 = 3;
pub const GALAXY_DSP_MINOR: u32 = 1;

pub const GALAXY_CONFIG_SIZE: u32 = 4;

/*
 * 32-bit config register
 */

pub const GALAXY_CONFIG_SBA_220: u32 = 0 << 0;
pub const GALAXY_CONFIG_SBA_240: u32 = 1 << 0;
pub const GALAXY_CONFIG_SBA_260: u32 = 2 << 0;
pub const GALAXY_CONFIG_SBA_280: u32 = 3 << 0;
pub const GALAXY_CONFIG_SBA_MASK: u32 = GALAXY_CONFIG_SBA_280;

pub const GALAXY_CONFIG_SBIRQ_2: u32 = 1 << 2;
pub const GALAXY_CONFIG_SBIRQ_5: u32 = 1 << 3;
pub const GALAXY_CONFIG_SBIRQ_7: u32 = 1 << 4;
pub const GALAXY_CONFIG_SBIRQ_10: u32 = 1 << 5;

pub const GALAXY_CONFIG_SBDMA_DISABLE: u32 = 0 << 6;
pub const GALAXY_CONFIG_SBDMA_0: u32 = 1 << 6;
pub const GALAXY_CONFIG_SBDMA_1: u32 = 2 << 6;
pub const GALAXY_CONFIG_SBDMA_3: u32 = 3 << 6;

pub const GALAXY_CONFIG_WSSA_530: u32 = 0 << 8;
pub const GALAXY_CONFIG_WSSA_604: u32 = 1 << 8;
pub const GALAXY_CONFIG_WSSA_E80: u32 = 2 << 8;
pub const GALAXY_CONFIG_WSSA_F40: u32 = 3 << 8;

pub const GALAXY_CONFIG_WSS_ENABLE: u32 = 1 << 10;

pub const GALAXY_CONFIG_GAME_ENABLE: u32 = 1 << 11;

pub const GALAXY_CONFIG_MPUA_300: u32 = 0 << 12;
pub const GALAXY_CONFIG_MPUA_330: u32 = 1 << 12;

pub const GALAXY_CONFIG_MPU_ENABLE: u32 = 1 << 13;

pub const GALAXY_CONFIG_CDA_310: u32 = 0 << 14;
pub const GALAXY_CONFIG_CDA_320: u32 = 1 << 14;
pub const GALAXY_CONFIG_CDA_340: u32 = 2 << 14;
pub const GALAXY_CONFIG_CDA_350: u32 = 3 << 14;
pub const GALAXY_CONFIG_CDA_MASK: u32 = GALAXY_CONFIG_CDA_350;

pub const GALAXY_CONFIG_CD_DISABLE: u32 = 0 << 16;
pub const GALAXY_CONFIG_CD_PANASONIC: u32 = 1 << 16;
pub const GALAXY_CONFIG_CD_SONY: u32 = 2 << 16;
pub const GALAXY_CONFIG_CD_MITSUMI: u32 = 3 << 16;
pub const GALAXY_CONFIG_CD_AZTECH: u32 = 4 << 16;
pub const GALAXY_CONFIG_CD_UNUSED_5: u32 = 5 << 16;
pub const GALAXY_CONFIG_CD_UNUSED_6: u32 = 6 << 16;
pub const GALAXY_CONFIG_CD_UNUSED_7: u32 = 7 << 16;
pub const GALAXY_CONFIG_CD_MASK: u32 = GALAXY_CONFIG_CD_UNUSED_7;

pub const GALAXY_CONFIG_CDDMA8_DISABLE: u32 = 0 << 20;
pub const GALAXY_CONFIG_CDDMA8_0: u32 = 1 << 20;
pub const GALAXY_CONFIG_CDDMA8_1: u32 = 2 << 20;
pub const GALAXY_CONFIG_CDDMA8_3: u32 = 3 << 20;
pub const GALAXY_CONFIG_CDDMA8_MASK: u32 = GALAXY_CONFIG_CDDMA8_3;

pub const GALAXY_CONFIG_CDDMA16_DISABLE: u32 = 0 << 22;
pub const GALAXY_CONFIG_CDDMA16_5: u32 = 1 << 22;
pub const GALAXY_CONFIG_CDDMA16_6: u32 = 2 << 22;
pub const GALAXY_CONFIG_CDDMA16_7: u32 = 3 << 22;
pub const GALAXY_CONFIG_CDDMA16_MASK: u32 = GALAXY_CONFIG_CDDMA16_7;

pub const GALAXY_CONFIG_MPUIRQ_2: u32 = 1 << 24;
pub const GALAXY_CONFIG_MPUIRQ_5: u32 = 1 << 25;
pub const GALAXY_CONFIG_MPUIRQ_7: u32 = 1 << 26;
pub const GALAXY_CONFIG_MPUIRQ_10: u32 = 1 << 27;

pub const GALAXY_CONFIG_CDIRQ_5: u32 = 1 << 28;
pub const GALAXY_CONFIG_CDIRQ_11: u32 = 1 << 29;
pub const GALAXY_CONFIG_CDIRQ_12: u32 = 1 << 30;
pub const GALAXY_CONFIG_CDIRQ_15: u32 = 1 << 31;
pub const GALAXY_CONFIG_CDIRQ_MASK: u32 = GALAXY_CONFIG_CDIRQ_5
    | GALAXY_CONFIG_CDIRQ_11
    | GALAXY_CONFIG_CDIRQ_12
    | GALAXY_CONFIG_CDIRQ_15;

pub const GALAXY_CONFIG_MASK: u32 = GALAXY_CONFIG_SBA_MASK
    | GALAXY_CONFIG_CDA_MASK
    | GALAXY_CONFIG_CD_MASK
    | GALAXY_CONFIG_CDDMA16_MASK
    | GALAXY_CONFIG_CDDMA8_MASK
    | GALAXY_CONFIG_CDIRQ_MASK;

// C source includes the shared implementation here:
// #include "galaxy.c"

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
