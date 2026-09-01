// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8960.h  --  WM8960 Soc Audio driver
 */

/* WM8960 register space */

pub const WM8960_CACHEREGNUM: u32 = 56;

pub const WM8960_LINVOL: u32 = 0x0;
pub const WM8960_RINVOL: u32 = 0x1;
pub const WM8960_LOUT1: u32 = 0x2;
pub const WM8960_ROUT1: u32 = 0x3;
pub const WM8960_CLOCK1: u32 = 0x4;
pub const WM8960_DACCTL1: u32 = 0x5;
pub const WM8960_DACCTL2: u32 = 0x6;
pub const WM8960_IFACE1: u32 = 0x7;
pub const WM8960_CLOCK2: u32 = 0x8;
pub const WM8960_IFACE2: u32 = 0x9;
pub const WM8960_LDAC: u32 = 0xa;
pub const WM8960_RDAC: u32 = 0xb;

pub const WM8960_RESET: u32 = 0xf;
pub const WM8960_3D: u32 = 0x10;
pub const WM8960_ALC1: u32 = 0x11;
pub const WM8960_ALC2: u32 = 0x12;
pub const WM8960_ALC3: u32 = 0x13;
pub const WM8960_NOISEG: u32 = 0x14;
pub const WM8960_LADC: u32 = 0x15;
pub const WM8960_RADC: u32 = 0x16;
pub const WM8960_ADDCTL1: u32 = 0x17;
pub const WM8960_ADDCTL2: u32 = 0x18;
pub const WM8960_POWER1: u32 = 0x19;
pub const WM8960_POWER2: u32 = 0x1a;
pub const WM8960_ADDCTL3: u32 = 0x1b;
pub const WM8960_APOP1: u32 = 0x1c;
pub const WM8960_APOP2: u32 = 0x1d;

pub const WM8960_LINPATH: u32 = 0x20;
pub const WM8960_RINPATH: u32 = 0x21;
pub const WM8960_LOUTMIX: u32 = 0x22;

pub const WM8960_ROUTMIX: u32 = 0x25;
pub const WM8960_MONOMIX1: u32 = 0x26;
pub const WM8960_MONOMIX2: u32 = 0x27;
pub const WM8960_LOUT2: u32 = 0x28;
pub const WM8960_ROUT2: u32 = 0x29;
pub const WM8960_MONO: u32 = 0x2a;
pub const WM8960_INBMIX1: u32 = 0x2b;
pub const WM8960_INBMIX2: u32 = 0x2c;
pub const WM8960_BYPASS1: u32 = 0x2d;
pub const WM8960_BYPASS2: u32 = 0x2e;
pub const WM8960_POWER3: u32 = 0x2f;
pub const WM8960_ADDCTL4: u32 = 0x30;
pub const WM8960_CLASSD1: u32 = 0x31;

pub const WM8960_CLASSD3: u32 = 0x33;
pub const WM8960_PLL1: u32 = 0x34;
pub const WM8960_PLL2: u32 = 0x35;
pub const WM8960_PLL3: u32 = 0x36;
pub const WM8960_PLL4: u32 = 0x37;

/*
 * WM8960 Clock dividers
 */
pub const WM8960_SYSCLKDIV: u32 = 0;
pub const WM8960_DACDIV: u32 = 1;
pub const WM8960_OPCLKDIV: u32 = 2;
pub const WM8960_DCLKDIV: u32 = 3;
pub const WM8960_TOCLKSEL: u32 = 4;

pub const WM8960_SYSCLK_DIV_1: u32 = 0 << 1;
pub const WM8960_SYSCLK_DIV_2: u32 = 2 << 1;

pub const WM8960_SYSCLK_AUTO: u32 = 0 << 0;
pub const WM8960_SYSCLK_PLL: u32 = 1 << 0;
pub const WM8960_SYSCLK_MCLK: u32 = 2 << 0;

pub const WM8960_DAC_DIV_1: u32 = 0 << 3;
pub const WM8960_DAC_DIV_1_5: u32 = 1 << 3;
pub const WM8960_DAC_DIV_2: u32 = 2 << 3;
pub const WM8960_DAC_DIV_3: u32 = 3 << 3;
pub const WM8960_DAC_DIV_4: u32 = 4 << 3;
pub const WM8960_DAC_DIV_5_5: u32 = 5 << 3;
pub const WM8960_DAC_DIV_6: u32 = 6 << 3;

pub const WM8960_DCLK_DIV_1_5: u32 = 0 << 6;
pub const WM8960_DCLK_DIV_2: u32 = 1 << 6;
pub const WM8960_DCLK_DIV_3: u32 = 2 << 6;
pub const WM8960_DCLK_DIV_4: u32 = 3 << 6;
pub const WM8960_DCLK_DIV_6: u32 = 4 << 6;
pub const WM8960_DCLK_DIV_8: u32 = 5 << 6;
pub const WM8960_DCLK_DIV_12: u32 = 6 << 6;
pub const WM8960_DCLK_DIV_16: u32 = 7 << 6;

pub const WM8960_TOCLK_F19: u32 = 0 << 1;
pub const WM8960_TOCLK_F21: u32 = 1 << 1;

pub const WM8960_OPCLK_DIV_1: u32 = 0 << 0;
pub const WM8960_OPCLK_DIV_2: u32 = 1 << 0;
pub const WM8960_OPCLK_DIV_3: u32 = 2 << 0;
pub const WM8960_OPCLK_DIV_4: u32 = 3 << 0;
pub const WM8960_OPCLK_DIV_5_5: u32 = 4 << 0;
pub const WM8960_OPCLK_DIV_6: u32 = 5 << 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
