/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2014 Renesas Solutions Corp.
 * Copyright (C) 2014 Wolfram Sang, Sang Engineering <wsa@sang-engineering.com>
 */

pub const R7S72100_CLK_PLL: u32 = 0;
pub const R7S72100_CLK_I: u32 = 1;
pub const R7S72100_CLK_G: u32 = 2;

/* MSTP2 */
pub const R7S72100_CLK_CORESIGHT: u32 = 0;

/* MSTP3 */
pub const R7S72100_CLK_IEBUS: u32 = 7;
pub const R7S72100_CLK_IRDA: u32 = 6;
pub const R7S72100_CLK_LIN0: u32 = 5;
pub const R7S72100_CLK_LIN1: u32 = 4;
pub const R7S72100_CLK_MTU2: u32 = 3;
pub const R7S72100_CLK_CAN: u32 = 2;
pub const R7S72100_CLK_ADCPWR: u32 = 1;
pub const R7S72100_CLK_PWM: u32 = 0;

/* MSTP4 */
pub const R7S72100_CLK_SCIF0: u32 = 7;
pub const R7S72100_CLK_SCIF1: u32 = 6;
pub const R7S72100_CLK_SCIF2: u32 = 5;
pub const R7S72100_CLK_SCIF3: u32 = 4;
pub const R7S72100_CLK_SCIF4: u32 = 3;
pub const R7S72100_CLK_SCIF5: u32 = 2;
pub const R7S72100_CLK_SCIF6: u32 = 1;
pub const R7S72100_CLK_SCIF7: u32 = 0;

/* MSTP5 */
pub const R7S72100_CLK_SCI0: u32 = 7;
pub const R7S72100_CLK_SCI1: u32 = 6;
pub const R7S72100_CLK_SG0: u32 = 5;
pub const R7S72100_CLK_SG1: u32 = 4;
pub const R7S72100_CLK_SG2: u32 = 3;
pub const R7S72100_CLK_SG3: u32 = 2;
pub const R7S72100_CLK_OSTM0: u32 = 1;
pub const R7S72100_CLK_OSTM1: u32 = 0;

/* MSTP6 */
pub const R7S72100_CLK_ADC: u32 = 7;
pub const R7S72100_CLK_CEU: u32 = 6;
pub const R7S72100_CLK_DOC0: u32 = 5;
pub const R7S72100_CLK_DOC1: u32 = 4;
pub const R7S72100_CLK_DRC0: u32 = 3;
pub const R7S72100_CLK_DRC1: u32 = 2;
pub const R7S72100_CLK_JCU: u32 = 1;
pub const R7S72100_CLK_RTC: u32 = 0;

/* MSTP7 */
pub const R7S72100_CLK_VDEC0: u32 = 7;
pub const R7S72100_CLK_VDEC1: u32 = 6;
pub const R7S72100_CLK_ETHER: u32 = 4;
pub const R7S72100_CLK_NAND: u32 = 3;
pub const R7S72100_CLK_USB0: u32 = 1;
pub const R7S72100_CLK_USB1: u32 = 0;

/* MSTP8 */
pub const R7S72100_CLK_IMR0: u32 = 7;
pub const R7S72100_CLK_IMR1: u32 = 6;
pub const R7S72100_CLK_IMRDISP: u32 = 5;
pub const R7S72100_CLK_MMCIF: u32 = 4;
pub const R7S72100_CLK_MLB: u32 = 3;
pub const R7S72100_CLK_ETHAVB: u32 = 2;
pub const R7S72100_CLK_SCUX: u32 = 1;

/* MSTP9 */
pub const R7S72100_CLK_I2C0: u32 = 7;
pub const R7S72100_CLK_I2C1: u32 = 6;
pub const R7S72100_CLK_I2C2: u32 = 5;
pub const R7S72100_CLK_I2C3: u32 = 4;
pub const R7S72100_CLK_SPIBSC0: u32 = 3;
pub const R7S72100_CLK_SPIBSC1: u32 = 2;
pub const R7S72100_CLK_VDC50: u32 = 1; /* and LVDS */
pub const R7S72100_CLK_VDC51: u32 = 0;

/* MSTP10 */
pub const R7S72100_CLK_SPI0: u32 = 7;
pub const R7S72100_CLK_SPI1: u32 = 6;
pub const R7S72100_CLK_SPI2: u32 = 5;
pub const R7S72100_CLK_SPI3: u32 = 4;
pub const R7S72100_CLK_SPI4: u32 = 3;
pub const R7S72100_CLK_CDROM: u32 = 2;
pub const R7S72100_CLK_SPDIF: u32 = 1;
pub const R7S72100_CLK_RGPVG2: u32 = 0;

/* MSTP11 */
pub const R7S72100_CLK_SSI0: u32 = 5;
pub const R7S72100_CLK_SSI1: u32 = 4;
pub const R7S72100_CLK_SSI2: u32 = 3;
pub const R7S72100_CLK_SSI3: u32 = 2;
pub const R7S72100_CLK_SSI4: u32 = 1;
pub const R7S72100_CLK_SSI5: u32 = 0;

/* MSTP12 */
pub const R7S72100_CLK_SDHI00: u32 = 3;
pub const R7S72100_CLK_SDHI01: u32 = 2;
pub const R7S72100_CLK_SDHI10: u32 = 1;
pub const R7S72100_CLK_SDHI11: u32 = 0;

/* MSTP13 */
pub const R7S72100_CLK_PIX1: u32 = 2;
pub const R7S72100_CLK_PIX0: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
