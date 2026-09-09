/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for the PRCMU bindings.
 */

/*
 * Clock identifiers.
 */
pub const ARMCLK: i32 = 0;
pub const PRCMU_ACLK: i32 = 1;
pub const PRCMU_SVAMMCSPCLK: i32 = 2;
pub const PRCMU_SDMMCHCLK: i32 = 2; /* DBx540 only. */
pub const PRCMU_SIACLK: i32 = 3;
pub const PRCMU_SIAMMDSPCLK: i32 = 3; /* DBx540 only. */
pub const PRCMU_SGACLK: i32 = 4;
pub const PRCMU_UARTCLK: i32 = 5;
pub const PRCMU_MSP02CLK: i32 = 6;
pub const PRCMU_MSP1CLK: i32 = 7;
pub const PRCMU_I2CCLK: i32 = 8;
pub const PRCMU_SDMMCCLK: i32 = 9;
pub const PRCMU_SLIMCLK: i32 = 10;
pub const PRCMU_CAMCLK: i32 = 10; /* DBx540 only. */
pub const PRCMU_PER1CLK: i32 = 11;
pub const PRCMU_PER2CLK: i32 = 12;
pub const PRCMU_PER3CLK: i32 = 13;
pub const PRCMU_PER5CLK: i32 = 14;
pub const PRCMU_PER6CLK: i32 = 15;
pub const PRCMU_PER7CLK: i32 = 16;
pub const PRCMU_LCDCLK: i32 = 17;
pub const PRCMU_BMLCLK: i32 = 18;
pub const PRCMU_HSITXCLK: i32 = 19;
pub const PRCMU_HSIRXCLK: i32 = 20;
pub const PRCMU_HDMICLK: i32 = 21;
pub const PRCMU_APEATCLK: i32 = 22;
pub const PRCMU_APETRACECLK: i32 = 23;
pub const PRCMU_MCDECLK: i32 = 24;
pub const PRCMU_IPI2CCLK: i32 = 25;
pub const PRCMU_DSIALTCLK: i32 = 26;
pub const PRCMU_DMACLK: i32 = 27;
pub const PRCMU_B2R2CLK: i32 = 28;
pub const PRCMU_TVCLK: i32 = 29;
pub const SPARE_UNIPROCLK: i32 = 30;
pub const PRCMU_SSPCLK: i32 = 31;
pub const PRCMU_RNGCLK: i32 = 32;
pub const PRCMU_UICCCLK: i32 = 33;
pub const PRCMU_G1CLK: i32 = 34; /* DBx540 only. */
pub const PRCMU_HVACLK: i32 = 35; /* DBx540 only. */
pub const PRCMU_SPARE1CLK: i32 = 36;
pub const PRCMU_SPARE2CLK: i32 = 37;

pub const PRCMU_NUM_REG_CLOCKS: i32 = 38;

pub const PRCMU_RTCCLK: i32 = PRCMU_NUM_REG_CLOCKS;
pub const PRCMU_SYSCLK: i32 = 39;
pub const PRCMU_CDCLK: i32 = 40;
pub const PRCMU_TIMCLK: i32 = 41;
pub const PRCMU_PLLSOC0: i32 = 42;
pub const PRCMU_PLLSOC1: i32 = 43;
pub const PRCMU_ARMSS: i32 = 44;
pub const PRCMU_PLLDDR: i32 = 45;

/* DSI Clocks */
pub const PRCMU_PLLDSI: i32 = 46;
pub const PRCMU_DSI0CLK: i32 = 47;
pub const PRCMU_DSI1CLK: i32 = 48;
pub const PRCMU_DSI0ESCCLK: i32 = 49;
pub const PRCMU_DSI1ESCCLK: i32 = 50;
pub const PRCMU_DSI2ESCCLK: i32 = 51;

/* LCD DSI PLL - Ux540 only */
pub const PRCMU_PLLDSI_LCD: i32 = 52;
pub const PRCMU_DSI0CLK_LCD: i32 = 53;
pub const PRCMU_DSI1CLK_LCD: i32 = 54;
pub const PRCMU_DSI0ESCCLK_LCD: i32 = 55;
pub const PRCMU_DSI1ESCCLK_LCD: i32 = 56;
pub const PRCMU_DSI2ESCCLK_LCD: i32 = 57;

pub const PRCMU_NUM_CLKS: i32 = 58;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
