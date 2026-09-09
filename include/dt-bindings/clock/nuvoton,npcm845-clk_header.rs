/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (C) 2021 Nuvoton Technologies.
 * Author: Tomer Maimon <tomer.maimon@nuvoton.com>
 *
 * Device Tree binding constants for NPCM8XX clock controller.
 */

pub const NPCM8XX_CLK_CPU: u32 = 0;
pub const NPCM8XX_CLK_GFX_PIXEL: u32 = 1;
pub const NPCM8XX_CLK_MC: u32 = 2;
pub const NPCM8XX_CLK_ADC: u32 = 3;
pub const NPCM8XX_CLK_AHB: u32 = 4;
pub const NPCM8XX_CLK_TIMER: u32 = 5;
pub const NPCM8XX_CLK_UART: u32 = 6;
pub const NPCM8XX_CLK_UART2: u32 = 7;
pub const NPCM8XX_CLK_MMC: u32 = 8;
pub const NPCM8XX_CLK_SPI3: u32 = 9;
pub const NPCM8XX_CLK_PCI: u32 = 10;
pub const NPCM8XX_CLK_AXI: u32 = 11;
pub const NPCM8XX_CLK_APB4: u32 = 12;
pub const NPCM8XX_CLK_APB3: u32 = 13;
pub const NPCM8XX_CLK_APB2: u32 = 14;
pub const NPCM8XX_CLK_APB1: u32 = 15;
pub const NPCM8XX_CLK_APB5: u32 = 16;
pub const NPCM8XX_CLK_CLKOUT: u32 = 17;
pub const NPCM8XX_CLK_GFX: u32 = 18;
pub const NPCM8XX_CLK_SU: u32 = 19;
pub const NPCM8XX_CLK_SU48: u32 = 20;
pub const NPCM8XX_CLK_SDHC: u32 = 21;
pub const NPCM8XX_CLK_SPI0: u32 = 22;
pub const NPCM8XX_CLK_SPI1: u32 = 23;
pub const NPCM8XX_CLK_SPIX: u32 = 24;
pub const NPCM8XX_CLK_RG: u32 = 25;
pub const NPCM8XX_CLK_RCP: u32 = 26;
pub const NPCM8XX_CLK_PRE_ADC: u32 = 27;
pub const NPCM8XX_CLK_ATB: u32 = 28;
pub const NPCM8XX_CLK_PRE_CLK: u32 = 29;
pub const NPCM8XX_CLK_TH: u32 = 30;
pub const NPCM8XX_CLK_REFCLK: u32 = 31;
pub const NPCM8XX_CLK_SYSBYPCK: u32 = 32;
pub const NPCM8XX_CLK_MCBYPCK: u32 = 33;

pub const NPCM8XX_NUM_CLOCKS: u32 = NPCM8XX_CLK_MCBYPCK + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
