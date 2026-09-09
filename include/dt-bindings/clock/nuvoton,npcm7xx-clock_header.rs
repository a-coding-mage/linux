/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Nuvoton NPCM7xx Clock Generator binding
 * clock binding number for all clocks supported by nuvoton,npcm7xx-clk
 *
 * Copyright (C) 2018 Nuvoton Technologies tali.perry@nuvoton.com
 *
 */

pub const NPCM7XX_CLK_CPU: u32 = 0;
pub const NPCM7XX_CLK_GFX_PIXEL: u32 = 1;
pub const NPCM7XX_CLK_MC: u32 = 2;
pub const NPCM7XX_CLK_ADC: u32 = 3;
pub const NPCM7XX_CLK_AHB: u32 = 4;
pub const NPCM7XX_CLK_TIMER: u32 = 5;
pub const NPCM7XX_CLK_UART: u32 = 6;
pub const NPCM7XX_CLK_MMC: u32 = 7;
pub const NPCM7XX_CLK_SPI3: u32 = 8;
pub const NPCM7XX_CLK_PCI: u32 = 9;
pub const NPCM7XX_CLK_AXI: u32 = 10;
pub const NPCM7XX_CLK_APB4: u32 = 11;
pub const NPCM7XX_CLK_APB3: u32 = 12;
pub const NPCM7XX_CLK_APB2: u32 = 13;
pub const NPCM7XX_CLK_APB1: u32 = 14;
pub const NPCM7XX_CLK_APB5: u32 = 15;
pub const NPCM7XX_CLK_CLKOUT: u32 = 16;
pub const NPCM7XX_CLK_GFX: u32 = 17;
pub const NPCM7XX_CLK_SU: u32 = 18;
pub const NPCM7XX_CLK_SU48: u32 = 19;
pub const NPCM7XX_CLK_SDHC: u32 = 20;
pub const NPCM7XX_CLK_SPI0: u32 = 21;
pub const NPCM7XX_CLK_SPIX: u32 = 22;

pub const NPCM7XX_CLK_REFCLK: u32 = 23;
pub const NPCM7XX_CLK_SYSBYPCK: u32 = 24;
pub const NPCM7XX_CLK_MCBYPCK: u32 = 25;

pub const NPCM7XX_NUM_CLOCKS: u32 = NPCM7XX_CLK_MCBYPCK + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
