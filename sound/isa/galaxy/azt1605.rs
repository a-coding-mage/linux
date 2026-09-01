// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Aztech AZT1605 Driver
 * Copyright (C) 2007,2010  Rene Herman
 */

pub const AZT1605: bool = true;

pub const CRD_NAME: &str = "Aztech AZT1605";
pub const DRV_NAME: &str = "AZT1605";
pub const DEV_NAME: &str = "azt1605";

pub const GALAXY_DSP_MAJOR: i32 = 2;
pub const GALAXY_DSP_MINOR: i32 = 1;

pub const GALAXY_CONFIG_SIZE: i32 = 3;

/*
 * 24-bit config register
 */

pub const GALAXY_CONFIG_SBA_220: i32 = 0 << 0;
pub const GALAXY_CONFIG_SBA_240: i32 = 1 << 0;
pub const GALAXY_CONFIG_SBA_260: i32 = 2 << 0;
pub const GALAXY_CONFIG_SBA_280: i32 = 3 << 0;
pub const GALAXY_CONFIG_SBA_MASK: i32 = GALAXY_CONFIG_SBA_280;

pub const GALAXY_CONFIG_MPUA_300: i32 = 0 << 2;
pub const GALAXY_CONFIG_MPUA_330: i32 = 1 << 2;

pub const GALAXY_CONFIG_MPU_ENABLE: i32 = 1 << 3;

pub const GALAXY_CONFIG_GAME_ENABLE: i32 = 1 << 4;

pub const GALAXY_CONFIG_CD_PANASONIC: i32 = 1 << 5;
pub const GALAXY_CONFIG_CD_MITSUMI: i32 = 1 << 6;
pub const GALAXY_CONFIG_CD_MASK: i32 =
    GALAXY_CONFIG_CD_PANASONIC | GALAXY_CONFIG_CD_MITSUMI;

pub const GALAXY_CONFIG_UNUSED: i32 = 1 << 7;
pub const GALAXY_CONFIG_UNUSED_MASK: i32 = GALAXY_CONFIG_UNUSED;

pub const GALAXY_CONFIG_SBIRQ_2: i32 = 1 << 8;
pub const GALAXY_CONFIG_SBIRQ_3: i32 = 1 << 9;
pub const GALAXY_CONFIG_SBIRQ_5: i32 = 1 << 10;
pub const GALAXY_CONFIG_SBIRQ_7: i32 = 1 << 11;

pub const GALAXY_CONFIG_MPUIRQ_2: i32 = 1 << 12;
pub const GALAXY_CONFIG_MPUIRQ_3: i32 = 1 << 13;
pub const GALAXY_CONFIG_MPUIRQ_5: i32 = 1 << 14;
pub const GALAXY_CONFIG_MPUIRQ_7: i32 = 1 << 15;

pub const GALAXY_CONFIG_WSSA_530: i32 = 0 << 16;
pub const GALAXY_CONFIG_WSSA_604: i32 = 1 << 16;
pub const GALAXY_CONFIG_WSSA_E80: i32 = 2 << 16;
pub const GALAXY_CONFIG_WSSA_F40: i32 = 3 << 16;

pub const GALAXY_CONFIG_WSS_ENABLE: i32 = 1 << 18;

pub const GALAXY_CONFIG_CDIRQ_11: i32 = 1 << 19;
pub const GALAXY_CONFIG_CDIRQ_12: i32 = 1 << 20;
pub const GALAXY_CONFIG_CDIRQ_15: i32 = 1 << 21;
pub const GALAXY_CONFIG_CDIRQ_MASK: i32 =
    GALAXY_CONFIG_CDIRQ_11 | GALAXY_CONFIG_CDIRQ_12 | GALAXY_CONFIG_CDIRQ_15;

pub const GALAXY_CONFIG_CDDMA_DISABLE: i32 = 0 << 22;
pub const GALAXY_CONFIG_CDDMA_0: i32 = 1 << 22;
pub const GALAXY_CONFIG_CDDMA_1: i32 = 2 << 22;
pub const GALAXY_CONFIG_CDDMA_3: i32 = 3 << 22;
pub const GALAXY_CONFIG_CDDMA_MASK: i32 = GALAXY_CONFIG_CDDMA_3;

pub const GALAXY_CONFIG_MASK: i32 =
    GALAXY_CONFIG_SBA_MASK | GALAXY_CONFIG_CD_MASK | GALAXY_CONFIG_UNUSED_MASK |
    GALAXY_CONFIG_CDIRQ_MASK | GALAXY_CONFIG_CDDMA_MASK;

// C source includes the shared implementation here:
// #include "galaxy.c"

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
