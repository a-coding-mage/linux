// Faithful Rust translation of lpc32xx.h.
#![allow(non_snake_case, non_upper_case_globals, dead_code)]

macro_rules! IOMEM { ($x:expr) => { $x }; }
macro_rules! io_p2v { ($x:expr) => { $x }; }
macro_rules! io_v2p { ($x:expr) => { $x }; }

/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * arch/arm/mach-lpc32xx/include/mach/platform.h
 *
 * Author: Kevin Wells <kevin.wells@nxp.com>
 *
 * Copyright (C) 2010 NXP Semiconductors
 */


macro_rules! _SBF { ($f:expr, $v:expr) => { (($v) << ($f)) }; }
macro_rules! _BIT { ($n:expr) => { _SBF($n, 1) }; }

/*
 * AHB 0 physical base addresses
 */
pub const LPC32XX_SLC_BASE: u32 = 0x20020000;
pub const LPC32XX_SSP0_BASE: u32 = 0x20084000;
pub const LPC32XX_SPI1_BASE: u32 = 0x20088000;
pub const LPC32XX_SSP1_BASE: u32 = 0x2008C000;
pub const LPC32XX_SPI2_BASE: u32 = 0x20090000;
pub const LPC32XX_I2S0_BASE: u32 = 0x20094000;
pub const LPC32XX_SD_BASE: u32 = 0x20098000;
pub const LPC32XX_I2S1_BASE: u32 = 0x2009C000;
pub const LPC32XX_MLC_BASE: u32 = 0x200A8000;
pub const LPC32XX_AHB0_START: u32 = LPC32XX_SLC_BASE;
pub const LPC32XX_AHB0_SIZE: u32 = 0x00089000;

/*
 * AHB 1 physical base addresses
 */
pub const LPC32XX_DMA_BASE: u32 = 0x31000000;
pub const LPC32XX_USB_BASE: u32 = 0x31020000;
pub const LPC32XX_USBH_BASE: u32 = 0x31020000;
pub const LPC32XX_USB_OTG_BASE: u32 = 0x31020000;
pub const LPC32XX_OTG_I2C_BASE: u32 = 0x31020300;
pub const LPC32XX_LCD_BASE: u32 = 0x31040000;
pub const LPC32XX_ETHERNET_BASE: u32 = 0x31060000;
pub const LPC32XX_EMC_BASE: u32 = 0x31080000;
pub const LPC32XX_ETB_CFG_BASE: u32 = 0x310C0000;
pub const LPC32XX_ETB_DATA_BASE: u32 = 0x310E0000;
pub const LPC32XX_AHB1_START: u32 = LPC32XX_DMA_BASE;
pub const LPC32XX_AHB1_SIZE: u32 = 0x000E1000;

/*
 * FAB physical base addresses
 */
pub const LPC32XX_CLK_PM_BASE: u32 = 0x40004000;
pub const LPC32XX_MIC_BASE: u32 = 0x40008000;
pub const LPC32XX_SIC1_BASE: u32 = 0x4000C000;
pub const LPC32XX_SIC2_BASE: u32 = 0x40010000;
pub const LPC32XX_HS_UART1_BASE: u32 = 0x40014000;
pub const LPC32XX_HS_UART2_BASE: u32 = 0x40018000;
pub const LPC32XX_HS_UART7_BASE: u32 = 0x4001C000;
pub const LPC32XX_RTC_BASE: u32 = 0x40024000;
pub const LPC32XX_RTC_RAM_BASE: u32 = 0x40024080;
pub const LPC32XX_GPIO_BASE: u32 = 0x40028000;
pub const LPC32XX_PWM3_BASE: u32 = 0x4002C000;
pub const LPC32XX_PWM4_BASE: u32 = 0x40030000;
pub const LPC32XX_MSTIM_BASE: u32 = 0x40034000;
pub const LPC32XX_HSTIM_BASE: u32 = 0x40038000;
pub const LPC32XX_WDTIM_BASE: u32 = 0x4003C000;
pub const LPC32XX_DEBUG_CTRL_BASE: u32 = 0x40040000;
pub const LPC32XX_TIMER0_BASE: u32 = 0x40044000;
pub const LPC32XX_ADC_BASE: u32 = 0x40048000;
pub const LPC32XX_TIMER1_BASE: u32 = 0x4004C000;
pub const LPC32XX_KSCAN_BASE: u32 = 0x40050000;
pub const LPC32XX_UART_CTRL_BASE: u32 = 0x40054000;
pub const LPC32XX_TIMER2_BASE: u32 = 0x40058000;
pub const LPC32XX_PWM1_BASE: u32 = 0x4005C000;
pub const LPC32XX_PWM2_BASE: u32 = 0x4005C004;
pub const LPC32XX_TIMER3_BASE: u32 = 0x40060000;

/*
 * APB physical base addresses
 */
pub const LPC32XX_UART3_BASE: u32 = 0x40080000;
pub const LPC32XX_UART4_BASE: u32 = 0x40088000;
pub const LPC32XX_UART5_BASE: u32 = 0x40090000;
pub const LPC32XX_UART6_BASE: u32 = 0x40098000;
pub const LPC32XX_I2C1_BASE: u32 = 0x400A0000;
pub const LPC32XX_I2C2_BASE: u32 = 0x400A8000;

/*
 * FAB and APB base and sizing
 */
pub const LPC32XX_FABAPB_START: u32 = LPC32XX_CLK_PM_BASE;
pub const LPC32XX_FABAPB_SIZE: u32 = 0x000A5000;

/*
 * Internal memory bases and sizes
 */
pub const LPC32XX_IRAM_BASE: u32 = 0x08000000;
pub const LPC32XX_IROM_BASE: u32 = 0x0C000000;

/*
 * External Static Memory Bank Address Space Bases
 */
pub const LPC32XX_EMC_CS0_BASE: u32 = 0xE0000000;
pub const LPC32XX_EMC_CS1_BASE: u32 = 0xE1000000;
pub const LPC32XX_EMC_CS2_BASE: u32 = 0xE2000000;
pub const LPC32XX_EMC_CS3_BASE: u32 = 0xE3000000;

/*
 * External SDRAM Memory Bank Address Space Bases
 */
pub const LPC32XX_EMC_DYCS0_BASE: u32 = 0x80000000;
pub const LPC32XX_EMC_DYCS1_BASE: u32 = 0xA0000000;

/*
 * Clock and crystal information
 */
pub const LPC32XX_MAIN_OSC_FREQ: u32 = 13000000;
pub const LPC32XX_CLOCK_OSC_FREQ: u32 = 32768;

/*
 * Clock and Power control register offsets
 */
macro_rules! _PMREG { ($x:expr) => { io_p2v!(LPC32XX_CLK_PM_BASE + ($x)) }; }
pub const LPC32XX_CLKPWR_DEBUG_CTRL: u32 = _PMREG(0x000);
pub const LPC32XX_CLKPWR_BOOTMAP: u32 = _PMREG(0x014);
pub const LPC32XX_CLKPWR_P01_ER: u32 = _PMREG(0x018);
pub const LPC32XX_CLKPWR_USBCLK_PDIV: u32 = _PMREG(0x01C);
pub const LPC32XX_CLKPWR_INT_ER: u32 = _PMREG(0x020);
pub const LPC32XX_CLKPWR_INT_RS: u32 = _PMREG(0x024);
pub const LPC32XX_CLKPWR_INT_SR: u32 = _PMREG(0x028);
pub const LPC32XX_CLKPWR_INT_AP: u32 = _PMREG(0x02C);
pub const LPC32XX_CLKPWR_PIN_ER: u32 = _PMREG(0x030);
pub const LPC32XX_CLKPWR_PIN_RS: u32 = _PMREG(0x034);
pub const LPC32XX_CLKPWR_PIN_SR: u32 = _PMREG(0x038);
pub const LPC32XX_CLKPWR_PIN_AP: u32 = _PMREG(0x03C);
pub const LPC32XX_CLKPWR_HCLK_DIV: u32 = _PMREG(0x040);
pub const LPC32XX_CLKPWR_PWR_CTRL: u32 = _PMREG(0x044);
pub const LPC32XX_CLKPWR_PLL397_CTRL: u32 = _PMREG(0x048);
pub const LPC32XX_CLKPWR_MAIN_OSC_CTRL: u32 = _PMREG(0x04C);
pub const LPC32XX_CLKPWR_SYSCLK_CTRL: u32 = _PMREG(0x050);
pub const LPC32XX_CLKPWR_LCDCLK_CTRL: u32 = _PMREG(0x054);
pub const LPC32XX_CLKPWR_HCLKPLL_CTRL: u32 = _PMREG(0x058);
pub const LPC32XX_CLKPWR_ADC_CLK_CTRL_1: u32 = _PMREG(0x060);
pub const LPC32XX_CLKPWR_USB_CTRL: u32 = _PMREG(0x064);
pub const LPC32XX_CLKPWR_SDRAMCLK_CTRL: u32 = _PMREG(0x068);
pub const LPC32XX_CLKPWR_DDR_LAP_NOM: u32 = _PMREG(0x06C);
pub const LPC32XX_CLKPWR_DDR_LAP_COUNT: u32 = _PMREG(0x070);
pub const LPC32XX_CLKPWR_DDR_LAP_DELAY: u32 = _PMREG(0x074);
pub const LPC32XX_CLKPWR_SSP_CLK_CTRL: u32 = _PMREG(0x078);
pub const LPC32XX_CLKPWR_I2S_CLK_CTRL: u32 = _PMREG(0x07C);
pub const LPC32XX_CLKPWR_MS_CTRL: u32 = _PMREG(0x080);
pub const LPC32XX_CLKPWR_MACCLK_CTRL: u32 = _PMREG(0x090);
pub const LPC32XX_CLKPWR_TEST_CLK_SEL: u32 = _PMREG(0x0A4);
pub const LPC32XX_CLKPWR_SFW_INT: u32 = _PMREG(0x0A8);
pub const LPC32XX_CLKPWR_I2C_CLK_CTRL: u32 = _PMREG(0x0AC);
pub const LPC32XX_CLKPWR_KEY_CLK_CTRL: u32 = _PMREG(0x0B0);
pub const LPC32XX_CLKPWR_ADC_CLK_CTRL: u32 = _PMREG(0x0B4);
pub const LPC32XX_CLKPWR_PWM_CLK_CTRL: u32 = _PMREG(0x0B8);
pub const LPC32XX_CLKPWR_TIMER_CLK_CTRL: u32 = _PMREG(0x0BC);
pub const LPC32XX_CLKPWR_TIMERS_PWMS_CLK_CTRL_1: u32 = _PMREG(0x0C0);
pub const LPC32XX_CLKPWR_SPI_CLK_CTRL: u32 = _PMREG(0x0C4);
pub const LPC32XX_CLKPWR_NAND_CLK_CTRL: u32 = _PMREG(0x0C8);
pub const LPC32XX_CLKPWR_UART3_CLK_CTRL: u32 = _PMREG(0x0D0);
pub const LPC32XX_CLKPWR_UART4_CLK_CTRL: u32 = _PMREG(0x0D4);
pub const LPC32XX_CLKPWR_UART5_CLK_CTRL: u32 = _PMREG(0x0D8);
pub const LPC32XX_CLKPWR_UART6_CLK_CTRL: u32 = _PMREG(0x0DC);
pub const LPC32XX_CLKPWR_IRDA_CLK_CTRL: u32 = _PMREG(0x0E0);
pub const LPC32XX_CLKPWR_UART_CLK_CTRL: u32 = _PMREG(0x0E4);
pub const LPC32XX_CLKPWR_DMA_CLK_CTRL: u32 = _PMREG(0x0E8);
pub const LPC32XX_CLKPWR_AUTOCLOCK: u32 = _PMREG(0x0EC);
macro_rules! LPC32XX_CLKPWR_DEVID { ($x:expr) => { _PMREG(0x130 + ($x)) }; }

/*
 * clkpwr_debug_ctrl register definitions
*/
pub const LPC32XX_CLKPWR_VFP_CLOCK_ENABLE_BIT: u32 = _BIT(4);

/*
 * clkpwr_bootmap register definitions
 */
pub const LPC32XX_CLKPWR_BOOTMAP_SEL_BIT: u32 = _BIT(1);

/*
 * clkpwr_start_gpio register bit definitions
 */
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO23_BIT: u32 = _BIT(31);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO22_BIT: u32 = _BIT(30);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO21_BIT: u32 = _BIT(29);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO20_BIT: u32 = _BIT(28);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO19_BIT: u32 = _BIT(27);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO18_BIT: u32 = _BIT(26);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO17_BIT: u32 = _BIT(25);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO16_BIT: u32 = _BIT(24);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO15_BIT: u32 = _BIT(23);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO14_BIT: u32 = _BIT(22);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO13_BIT: u32 = _BIT(21);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO12_BIT: u32 = _BIT(20);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO11_BIT: u32 = _BIT(19);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO10_BIT: u32 = _BIT(18);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO9_BIT: u32 = _BIT(17);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO8_BIT: u32 = _BIT(16);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO7_BIT: u32 = _BIT(15);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO6_BIT: u32 = _BIT(14);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO5_BIT: u32 = _BIT(13);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO4_BIT: u32 = _BIT(12);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO3_BIT: u32 = _BIT(11);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO2_BIT: u32 = _BIT(10);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO1_BIT: u32 = _BIT(9);
pub const LPC32XX_CLKPWR_GPIOSRC_P1IO0_BIT: u32 = _BIT(8);
pub const LPC32XX_CLKPWR_GPIOSRC_P0IO7_BIT: u32 = _BIT(7);
pub const LPC32XX_CLKPWR_GPIOSRC_P0IO6_BIT: u32 = _BIT(6);
pub const LPC32XX_CLKPWR_GPIOSRC_P0IO5_BIT: u32 = _BIT(5);
pub const LPC32XX_CLKPWR_GPIOSRC_P0IO4_BIT: u32 = _BIT(4);
pub const LPC32XX_CLKPWR_GPIOSRC_P0IO3_BIT: u32 = _BIT(3);
pub const LPC32XX_CLKPWR_GPIOSRC_P0IO2_BIT: u32 = _BIT(2);
pub const LPC32XX_CLKPWR_GPIOSRC_P0IO1_BIT: u32 = _BIT(1);
pub const LPC32XX_CLKPWR_GPIOSRC_P0IO0_BIT: u32 = _BIT(0);

/*
 * clkpwr_usbclk_pdiv register definitions
 */
pub const LPC32XX_CLKPWR_USBPDIV_PLL_MASK: u32 = 0xF;

/*
 * clkpwr_start_int, clkpwr_start_raw_sts_int, clkpwr_start_sts_int,
 * clkpwr_start_pol_int, register bit definitions
 */
pub const LPC32XX_CLKPWR_INTSRC_ADC_BIT: u32 = _BIT(31);
pub const LPC32XX_CLKPWR_INTSRC_TS_P_BIT: u32 = _BIT(30);
pub const LPC32XX_CLKPWR_INTSRC_TS_AUX_BIT: u32 = _BIT(29);
pub const LPC32XX_CLKPWR_INTSRC_USBAHNEEDCLK_BIT: u32 = _BIT(26);
pub const LPC32XX_CLKPWR_INTSRC_MSTIMER_BIT: u32 = _BIT(25);
pub const LPC32XX_CLKPWR_INTSRC_RTC_BIT: u32 = _BIT(24);
pub const LPC32XX_CLKPWR_INTSRC_USBNEEDCLK_BIT: u32 = _BIT(23);
pub const LPC32XX_CLKPWR_INTSRC_USB_BIT: u32 = _BIT(22);
pub const LPC32XX_CLKPWR_INTSRC_I2C_BIT: u32 = _BIT(21);
pub const LPC32XX_CLKPWR_INTSRC_USBOTGTIMER_BIT: u32 = _BIT(20);
pub const LPC32XX_CLKPWR_INTSRC_USBATXINT_BIT: u32 = _BIT(19);
pub const LPC32XX_CLKPWR_INTSRC_KEY_BIT: u32 = _BIT(16);
pub const LPC32XX_CLKPWR_INTSRC_MAC_BIT: u32 = _BIT(7);
pub const LPC32XX_CLKPWR_INTSRC_P0P1_BIT: u32 = _BIT(6);
pub const LPC32XX_CLKPWR_INTSRC_GPIO_05_BIT: u32 = _BIT(5);
pub const LPC32XX_CLKPWR_INTSRC_GPIO_04_BIT: u32 = _BIT(4);
pub const LPC32XX_CLKPWR_INTSRC_GPIO_03_BIT: u32 = _BIT(3);
pub const LPC32XX_CLKPWR_INTSRC_GPIO_02_BIT: u32 = _BIT(2);
pub const LPC32XX_CLKPWR_INTSRC_GPIO_01_BIT: u32 = _BIT(1);
pub const LPC32XX_CLKPWR_INTSRC_GPIO_00_BIT: u32 = _BIT(0);

/*
 * clkpwr_start_pin, clkpwr_start_raw_sts_pin, clkpwr_start_sts_pin,
 * clkpwr_start_pol_pin register bit definitions
 */
pub const LPC32XX_CLKPWR_EXTSRC_U7_RX_BIT: u32 = _BIT(31);
pub const LPC32XX_CLKPWR_EXTSRC_U7_HCTS_BIT: u32 = _BIT(30);
pub const LPC32XX_CLKPWR_EXTSRC_U6_IRRX_BIT: u32 = _BIT(28);
pub const LPC32XX_CLKPWR_EXTSRC_U5_RX_BIT: u32 = _BIT(26);
pub const LPC32XX_CLKPWR_EXTSRC_GPI_28_BIT: u32 = _BIT(25);
pub const LPC32XX_CLKPWR_EXTSRC_U3_RX_BIT: u32 = _BIT(24);
pub const LPC32XX_CLKPWR_EXTSRC_U2_HCTS_BIT: u32 = _BIT(23);
pub const LPC32XX_CLKPWR_EXTSRC_U2_RX_BIT: u32 = _BIT(22);
pub const LPC32XX_CLKPWR_EXTSRC_U1_RX_BIT: u32 = _BIT(21);
pub const LPC32XX_CLKPWR_EXTSRC_MSDIO_INT_BIT: u32 = _BIT(18);
pub const LPC32XX_CLKPWR_EXTSRC_MSDIO_SRT_BIT: u32 = _BIT(17);
pub const LPC32XX_CLKPWR_EXTSRC_GPI_06_BIT: u32 = _BIT(16);
pub const LPC32XX_CLKPWR_EXTSRC_GPI_05_BIT: u32 = _BIT(15);
pub const LPC32XX_CLKPWR_EXTSRC_GPI_04_BIT: u32 = _BIT(14);
pub const LPC32XX_CLKPWR_EXTSRC_GPI_03_BIT: u32 = _BIT(13);
pub const LPC32XX_CLKPWR_EXTSRC_GPI_02_BIT: u32 = _BIT(12);
pub const LPC32XX_CLKPWR_EXTSRC_GPI_01_BIT: u32 = _BIT(11);
pub const LPC32XX_CLKPWR_EXTSRC_GPI_00_BIT: u32 = _BIT(10);
pub const LPC32XX_CLKPWR_EXTSRC_SYSCLKEN_BIT: u32 = _BIT(9);
pub const LPC32XX_CLKPWR_EXTSRC_SPI1_DATIN_BIT: u32 = _BIT(8);
pub const LPC32XX_CLKPWR_EXTSRC_GPI_07_BIT: u32 = _BIT(7);
pub const LPC32XX_CLKPWR_EXTSRC_SPI2_DATIN_BIT: u32 = _BIT(6);
pub const LPC32XX_CLKPWR_EXTSRC_GPI_19_BIT: u32 = _BIT(5);
pub const LPC32XX_CLKPWR_EXTSRC_GPI_09_BIT: u32 = _BIT(4);
pub const LPC32XX_CLKPWR_EXTSRC_GPI_08_BIT: u32 = _BIT(3);

/*
 * clkpwr_hclk_div register definitions
 */
pub const LPC32XX_CLKPWR_HCLKDIV_DDRCLK_STOP: u32 = (0x0 << 7);
pub const LPC32XX_CLKPWR_HCLKDIV_DDRCLK_NORM: u32 = (0x1 << 7);
pub const LPC32XX_CLKPWR_HCLKDIV_DDRCLK_HALF: u32 = (0x2 << 7);
macro_rules! LPC32XX_CLKPWR_HCLKDIV_PCLK_DIV { ($n:expr) => { ((($n) & 0x1F) << 2) }; }
macro_rules! LPC32XX_CLKPWR_HCLKDIV_DIV_2POW { ($n:expr) => { (($n) & 0x3) }; }

/*
 * clkpwr_pwr_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_CTRL_FORCE_PCLK: u32 = _BIT(10);
pub const LPC32XX_CLKPWR_SDRAM_SELF_RFSH: u32 = _BIT(9);
pub const LPC32XX_CLKPWR_UPD_SDRAM_SELF_RFSH: u32 = _BIT(8);
pub const LPC32XX_CLKPWR_AUTO_SDRAM_SELF_RFSH: u32 = _BIT(7);
pub const LPC32XX_CLKPWR_HIGHCORE_STATE_BIT: u32 = _BIT(5);
pub const LPC32XX_CLKPWR_SYSCLKEN_STATE_BIT: u32 = _BIT(4);
pub const LPC32XX_CLKPWR_SYSCLKEN_GPIO_EN: u32 = _BIT(3);
pub const LPC32XX_CLKPWR_SELECT_RUN_MODE: u32 = _BIT(2);
pub const LPC32XX_CLKPWR_HIGHCORE_GPIO_EN: u32 = _BIT(1);
pub const LPC32XX_CLKPWR_STOP_MODE_CTRL: u32 = _BIT(0);

/*
 * clkpwr_pll397_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_PLL397_MSLOCK_STS: u32 = _BIT(10);
pub const LPC32XX_CLKPWR_PLL397_BYPASS: u32 = _BIT(9);
pub const LPC32XX_CLKPWR_PLL397_BIAS_NORM: u32 = 0x000;
pub const LPC32XX_CLKPWR_PLL397_BIAS_N12_5: u32 = 0x040;
pub const LPC32XX_CLKPWR_PLL397_BIAS_N25: u32 = 0x080;
pub const LPC32XX_CLKPWR_PLL397_BIAS_N37_5: u32 = 0x0C0;
pub const LPC32XX_CLKPWR_PLL397_BIAS_P12_5: u32 = 0x100;
pub const LPC32XX_CLKPWR_PLL397_BIAS_P25: u32 = 0x140;
pub const LPC32XX_CLKPWR_PLL397_BIAS_P37_5: u32 = 0x180;
pub const LPC32XX_CLKPWR_PLL397_BIAS_P50: u32 = 0x1C0;
pub const LPC32XX_CLKPWR_PLL397_BIAS_MASK: u32 = 0x1C0;
pub const LPC32XX_CLKPWR_SYSCTRL_PLL397_DIS: u32 = _BIT(1);
pub const LPC32XX_CLKPWR_SYSCTRL_PLL397_STS: u32 = _BIT(0);

/*
 * clkpwr_main_osc_ctrl register definitions
 */
macro_rules! LPC32XX_CLKPWR_MOSC_ADD_CAP { ($n:expr) => { ((($n) & 0x7F) << 2) }; }
pub const LPC32XX_CLKPWR_MOSC_CAP_MASK: u32 = (0x7F << 2);
pub const LPC32XX_CLKPWR_TEST_MODE: u32 = _BIT(1);
pub const LPC32XX_CLKPWR_MOSC_DISABLE: u32 = _BIT(0);

/*
 * clkpwr_sysclk_ctrl register definitions
 */
macro_rules! LPC32XX_CLKPWR_SYSCTRL_BP_TRIG { ($n:expr) => { ((($n) & 0x3FF) << 2) }; }
pub const LPC32XX_CLKPWR_SYSCTRL_BP_MASK: u32 = (0x3FF << 2);
pub const LPC32XX_CLKPWR_SYSCTRL_USEPLL397: u32 = _BIT(1);
pub const LPC32XX_CLKPWR_SYSCTRL_SYSCLKMUX: u32 = _BIT(0);

/*
 * clkpwr_lcdclk_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_LCDCTRL_LCDTYPE_TFT12: u32 = 0x000;
pub const LPC32XX_CLKPWR_LCDCTRL_LCDTYPE_TFT16: u32 = 0x040;
pub const LPC32XX_CLKPWR_LCDCTRL_LCDTYPE_TFT15: u32 = 0x080;
pub const LPC32XX_CLKPWR_LCDCTRL_LCDTYPE_TFT24: u32 = 0x0C0;
pub const LPC32XX_CLKPWR_LCDCTRL_LCDTYPE_STN4M: u32 = 0x100;
pub const LPC32XX_CLKPWR_LCDCTRL_LCDTYPE_STN8C: u32 = 0x140;
pub const LPC32XX_CLKPWR_LCDCTRL_LCDTYPE_DSTN4M: u32 = 0x180;
pub const LPC32XX_CLKPWR_LCDCTRL_LCDTYPE_DSTN8C: u32 = 0x1C0;
pub const LPC32XX_CLKPWR_LCDCTRL_LCDTYPE_MSK: u32 = 0x01C0;
pub const LPC32XX_CLKPWR_LCDCTRL_CLK_EN: u32 = 0x020;
macro_rules! LPC32XX_CLKPWR_LCDCTRL_SET_PSCALE { ($n:expr) => { (($n - 1) & 0x1F) }; }
pub const LPC32XX_CLKPWR_LCDCTRL_PSCALE_MSK: u32 = 0x001F;

/*
 * clkpwr_hclkpll_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_HCLKPLL_POWER_UP: u32 = _BIT(16);
pub const LPC32XX_CLKPWR_HCLKPLL_CCO_BYPASS: u32 = _BIT(15);
pub const LPC32XX_CLKPWR_HCLKPLL_POSTDIV_BYPASS: u32 = _BIT(14);
pub const LPC32XX_CLKPWR_HCLKPLL_FDBK_SEL_FCLK: u32 = _BIT(13);
macro_rules! LPC32XX_CLKPWR_HCLKPLL_POSTDIV_2POW { ($n:expr) => { ((($n) & 0x3) << 11) }; }
macro_rules! LPC32XX_CLKPWR_HCLKPLL_PREDIV_PLUS1 { ($n:expr) => { ((($n) & 0x3) << 9) }; }
macro_rules! LPC32XX_CLKPWR_HCLKPLL_PLLM { ($n:expr) => { ((($n) & 0xFF) << 1) }; }
pub const LPC32XX_CLKPWR_HCLKPLL_PLL_STS: u32 = _BIT(0);

/*
 * clkpwr_adc_clk_ctrl_1 register definitions
 */
macro_rules! LPC32XX_CLKPWR_ADCCTRL1_RTDIV { ($n:expr) => { ((($n) & 0xFF) << 0) }; }
pub const LPC32XX_CLKPWR_ADCCTRL1_PCLK_SEL: u32 = _BIT(8);

/*
 * clkpwr_usb_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_USBCTRL_HCLK_EN: u32 = _BIT(24);
pub const LPC32XX_CLKPWR_USBCTRL_USBI2C_EN: u32 = _BIT(23);
pub const LPC32XX_CLKPWR_USBCTRL_USBDVND_EN: u32 = _BIT(22);
pub const LPC32XX_CLKPWR_USBCTRL_USBHSTND_EN: u32 = _BIT(21);
pub const LPC32XX_CLKPWR_USBCTRL_PU_ADD: u32 = (0x0 << 19);
pub const LPC32XX_CLKPWR_USBCTRL_BUS_KEEPER: u32 = (0x1 << 19);
pub const LPC32XX_CLKPWR_USBCTRL_PD_ADD: u32 = (0x3 << 19);
pub const LPC32XX_CLKPWR_USBCTRL_CLK_EN2: u32 = _BIT(18);
pub const LPC32XX_CLKPWR_USBCTRL_CLK_EN1: u32 = _BIT(17);
pub const LPC32XX_CLKPWR_USBCTRL_PLL_PWRUP: u32 = _BIT(16);
pub const LPC32XX_CLKPWR_USBCTRL_CCO_BYPASS: u32 = _BIT(15);
pub const LPC32XX_CLKPWR_USBCTRL_POSTDIV_BYPASS: u32 = _BIT(14);
pub const LPC32XX_CLKPWR_USBCTRL_FDBK_SEL_FCLK: u32 = _BIT(13);
macro_rules! LPC32XX_CLKPWR_USBCTRL_POSTDIV_2POW { ($n:expr) => { ((($n) & 0x3) << 11) }; }
macro_rules! LPC32XX_CLKPWR_USBCTRL_PREDIV_PLUS1 { ($n:expr) => { ((($n) & 0x3) << 9) }; }
macro_rules! LPC32XX_CLKPWR_USBCTRL_FDBK_PLUS1 { ($n:expr) => { ((($n) & 0xFF) << 1) }; }
pub const LPC32XX_CLKPWR_USBCTRL_PLL_STS: u32 = _BIT(0);

/*
 * clkpwr_sdramclk_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_SDRCLK_FASTSLEW_CLK: u32 = _BIT(22);
pub const LPC32XX_CLKPWR_SDRCLK_FASTSLEW: u32 = _BIT(21);
pub const LPC32XX_CLKPWR_SDRCLK_FASTSLEW_DAT: u32 = _BIT(20);
pub const LPC32XX_CLKPWR_SDRCLK_SW_DDR_RESET: u32 = _BIT(19);
macro_rules! LPC32XX_CLKPWR_SDRCLK_HCLK_DLY { ($n:expr) => { ((($n) & 0x1F) << 14) }; }
pub const LPC32XX_CLKPWR_SDRCLK_DLY_ADDR_STS: u32 = _BIT(13);
macro_rules! LPC32XX_CLKPWR_SDRCLK_SENS_FACT { ($n:expr) => { ((($n) & 0x7) << 10) }; }
pub const LPC32XX_CLKPWR_SDRCLK_USE_CAL: u32 = _BIT(9);
pub const LPC32XX_CLKPWR_SDRCLK_DO_CAL: u32 = _BIT(8);
pub const LPC32XX_CLKPWR_SDRCLK_CAL_ON_RTC: u32 = _BIT(7);
macro_rules! LPC32XX_CLKPWR_SDRCLK_DQS_DLY { ($n:expr) => { ((($n) & 0x1F) << 2) }; }
pub const LPC32XX_CLKPWR_SDRCLK_USE_DDR: u32 = _BIT(1);
pub const LPC32XX_CLKPWR_SDRCLK_CLK_DIS: u32 = _BIT(0);

/*
 * clkpwr_ssp_blk_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_SSPCTRL_DMA_SSP1RX: u32 = _BIT(5);
pub const LPC32XX_CLKPWR_SSPCTRL_DMA_SSP1TX: u32 = _BIT(4);
pub const LPC32XX_CLKPWR_SSPCTRL_DMA_SSP0RX: u32 = _BIT(3);
pub const LPC32XX_CLKPWR_SSPCTRL_DMA_SSP0TX: u32 = _BIT(2);
pub const LPC32XX_CLKPWR_SSPCTRL_SSPCLK1_EN: u32 = _BIT(1);
pub const LPC32XX_CLKPWR_SSPCTRL_SSPCLK0_EN: u32 = _BIT(0);

/*
 * clkpwr_i2s_clk_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_I2SCTRL_I2S1_RX_FOR_TX: u32 = _BIT(6);
pub const LPC32XX_CLKPWR_I2SCTRL_I2S1_TX_FOR_RX: u32 = _BIT(5);
pub const LPC32XX_CLKPWR_I2SCTRL_I2S1_USE_DMA: u32 = _BIT(4);
pub const LPC32XX_CLKPWR_I2SCTRL_I2S0_RX_FOR_TX: u32 = _BIT(3);
pub const LPC32XX_CLKPWR_I2SCTRL_I2S0_TX_FOR_RX: u32 = _BIT(2);
pub const LPC32XX_CLKPWR_I2SCTRL_I2SCLK1_EN: u32 = _BIT(1);
pub const LPC32XX_CLKPWR_I2SCTRL_I2SCLK0_EN: u32 = _BIT(0);

/*
 * clkpwr_ms_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_MSCARD_MSDIO_PIN_DIS: u32 = _BIT(10);
pub const LPC32XX_CLKPWR_MSCARD_MSDIO_PU_EN: u32 = _BIT(9);
pub const LPC32XX_CLKPWR_MSCARD_MSDIO23_DIS: u32 = _BIT(8);
pub const LPC32XX_CLKPWR_MSCARD_MSDIO1_DIS: u32 = _BIT(7);
pub const LPC32XX_CLKPWR_MSCARD_MSDIO0_DIS: u32 = _BIT(6);
pub const LPC32XX_CLKPWR_MSCARD_SDCARD_EN: u32 = _BIT(5);
macro_rules! LPC32XX_CLKPWR_MSCARD_SDCARD_DIV { ($n:expr) => { (($n) & 0xF) }; }

/*
 * clkpwr_macclk_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_MACCTRL_NO_ENET_PIS: u32 = 0x00;
pub const LPC32XX_CLKPWR_MACCTRL_USE_MII_PINS: u32 = 0x08;
pub const LPC32XX_CLKPWR_MACCTRL_USE_RMII_PINS: u32 = 0x18;
pub const LPC32XX_CLKPWR_MACCTRL_PINS_MSK: u32 = 0x18;
pub const LPC32XX_CLKPWR_MACCTRL_DMACLK_EN: u32 = _BIT(2);
pub const LPC32XX_CLKPWR_MACCTRL_MMIOCLK_EN: u32 = _BIT(1);
pub const LPC32XX_CLKPWR_MACCTRL_HRCCLK_EN: u32 = _BIT(0);

/*
 * clkpwr_test_clk_sel register definitions
 */
pub const LPC32XX_CLKPWR_TESTCLK1_SEL_PERCLK: u32 = (0x0 << 5);
pub const LPC32XX_CLKPWR_TESTCLK1_SEL_RTC: u32 = (0x1 << 5);
pub const LPC32XX_CLKPWR_TESTCLK1_SEL_MOSC: u32 = (0x2 << 5);
pub const LPC32XX_CLKPWR_TESTCLK1_SEL_MASK: u32 = (0x3 << 5);
pub const LPC32XX_CLKPWR_TESTCLK_TESTCLK1_EN: u32 = _BIT(4);
pub const LPC32XX_CLKPWR_TESTCLK2_SEL_HCLK: u32 = (0x0 << 1);
pub const LPC32XX_CLKPWR_TESTCLK2_SEL_PERCLK: u32 = (0x1 << 1);
pub const LPC32XX_CLKPWR_TESTCLK2_SEL_USBCLK: u32 = (0x2 << 1);
pub const LPC32XX_CLKPWR_TESTCLK2_SEL_MOSC: u32 = (0x5 << 1);
pub const LPC32XX_CLKPWR_TESTCLK2_SEL_PLL397: u32 = (0x7 << 1);
pub const LPC32XX_CLKPWR_TESTCLK2_SEL_MASK: u32 = (0x7 << 1);
pub const LPC32XX_CLKPWR_TESTCLK_TESTCLK2_EN: u32 = _BIT(0);

/*
 * clkpwr_sw_int register definitions
 */
macro_rules! LPC32XX_CLKPWR_SW_INT { ($n:expr) => { (_BIT(0) | ((($n) & 0x7F) << 1)) }; }
macro_rules! LPC32XX_CLKPWR_SW_GET_ARG { ($n:expr) => { ((($n) & 0xFE) >> 1) }; }

/*
 * clkpwr_i2c_clk_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_I2CCLK_USBI2CHI_DRIVE: u32 = _BIT(4);
pub const LPC32XX_CLKPWR_I2CCLK_I2C2HI_DRIVE: u32 = _BIT(3);
pub const LPC32XX_CLKPWR_I2CCLK_I2C1HI_DRIVE: u32 = _BIT(2);
pub const LPC32XX_CLKPWR_I2CCLK_I2C2CLK_EN: u32 = _BIT(1);
pub const LPC32XX_CLKPWR_I2CCLK_I2C1CLK_EN: u32 = _BIT(0);

/*
 * clkpwr_key_clk_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_KEYCLKCTRL_CLK_EN: u32 = 0x1;

/*
 * clkpwr_adc_clk_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_ADC32CLKCTRL_CLK_EN: u32 = 0x1;

/*
 * clkpwr_pwm_clk_ctrl register definitions
 */
macro_rules! LPC32XX_CLKPWR_PWMCLK_PWM2_DIV { ($n:expr) => { ((($n) & 0xF) << 8) }; }
macro_rules! LPC32XX_CLKPWR_PWMCLK_PWM1_DIV { ($n:expr) => { ((($n) & 0xF) << 4) }; }
pub const LPC32XX_CLKPWR_PWMCLK_PWM2SEL_PCLK: u32 = 0x8;
pub const LPC32XX_CLKPWR_PWMCLK_PWM2CLK_EN: u32 = 0x4;
pub const LPC32XX_CLKPWR_PWMCLK_PWM1SEL_PCLK: u32 = 0x2;
pub const LPC32XX_CLKPWR_PWMCLK_PWM1CLK_EN: u32 = 0x1;

/*
 * clkpwr_timer_clk_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_PWMCLK_HSTIMER_EN: u32 = 0x2;
pub const LPC32XX_CLKPWR_PWMCLK_WDOG_EN: u32 = 0x1;

/*
 * clkpwr_timers_pwms_clk_ctrl_1 register definitions
 */
pub const LPC32XX_CLKPWR_TMRPWMCLK_MPWM_EN: u32 = 0x40;
pub const LPC32XX_CLKPWR_TMRPWMCLK_TIMER3_EN: u32 = 0x20;
pub const LPC32XX_CLKPWR_TMRPWMCLK_TIMER2_EN: u32 = 0x10;
pub const LPC32XX_CLKPWR_TMRPWMCLK_TIMER1_EN: u32 = 0x08;
pub const LPC32XX_CLKPWR_TMRPWMCLK_TIMER0_EN: u32 = 0x04;
pub const LPC32XX_CLKPWR_TMRPWMCLK_PWM4_EN: u32 = 0x02;
pub const LPC32XX_CLKPWR_TMRPWMCLK_PWM3_EN: u32 = 0x01;

/*
 * clkpwr_spi_clk_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_SPICLK_SET_SPI2DATIO: u32 = 0x80;
pub const LPC32XX_CLKPWR_SPICLK_SET_SPI2CLK: u32 = 0x40;
pub const LPC32XX_CLKPWR_SPICLK_USE_SPI2: u32 = 0x20;
pub const LPC32XX_CLKPWR_SPICLK_SPI2CLK_EN: u32 = 0x10;
pub const LPC32XX_CLKPWR_SPICLK_SET_SPI1DATIO: u32 = 0x08;
pub const LPC32XX_CLKPWR_SPICLK_SET_SPI1CLK: u32 = 0x04;
pub const LPC32XX_CLKPWR_SPICLK_USE_SPI1: u32 = 0x02;
pub const LPC32XX_CLKPWR_SPICLK_SPI1CLK_EN: u32 = 0x01;

/*
 * clkpwr_nand_clk_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_NANDCLK_INTSEL_MLC: u32 = 0x20;
pub const LPC32XX_CLKPWR_NANDCLK_DMA_RNB: u32 = 0x10;
pub const LPC32XX_CLKPWR_NANDCLK_DMA_INT: u32 = 0x08;
pub const LPC32XX_CLKPWR_NANDCLK_SEL_SLC: u32 = 0x04;
pub const LPC32XX_CLKPWR_NANDCLK_MLCCLK_EN: u32 = 0x02;
pub const LPC32XX_CLKPWR_NANDCLK_SLCCLK_EN: u32 = 0x01;

/*
 * clkpwr_uart3_clk_ctrl, clkpwr_uart4_clk_ctrl, clkpwr_uart5_clk_ctrl
 * and clkpwr_uart6_clk_ctrl register definitions
 */
macro_rules! LPC32XX_CLKPWR_UART_Y_DIV { ($y:expr) => { (($y) & 0xFF) }; }
macro_rules! LPC32XX_CLKPWR_UART_X_DIV { ($x:expr) => { ((($x) & 0xFF) << 8) }; }
pub const LPC32XX_CLKPWR_UART_USE_HCLK: u32 = _BIT(16);

/*
 * clkpwr_irda_clk_ctrl register definitions
 */
macro_rules! LPC32XX_CLKPWR_IRDA_Y_DIV { ($y:expr) => { (($y) & 0xFF) }; }
macro_rules! LPC32XX_CLKPWR_IRDA_X_DIV { ($x:expr) => { ((($x) & 0xFF) << 8) }; }

/*
 * clkpwr_uart_clk_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_UARTCLKCTRL_UART6_EN: u32 = _BIT(3);
pub const LPC32XX_CLKPWR_UARTCLKCTRL_UART5_EN: u32 = _BIT(2);
pub const LPC32XX_CLKPWR_UARTCLKCTRL_UART4_EN: u32 = _BIT(1);
pub const LPC32XX_CLKPWR_UARTCLKCTRL_UART3_EN: u32 = _BIT(0);

/*
 * clkpwr_dmaclk_ctrl register definitions
 */
pub const LPC32XX_CLKPWR_DMACLKCTRL_CLK_EN: u32 = 0x1;

/*
 * clkpwr_autoclock register definitions
 */
pub const LPC32XX_CLKPWR_AUTOCLK_USB_EN: u32 = 0x40;
pub const LPC32XX_CLKPWR_AUTOCLK_IRAM_EN: u32 = 0x02;
pub const LPC32XX_CLKPWR_AUTOCLK_IROM_EN: u32 = 0x01;

/*
 * Interrupt controller register offsets
 */
macro_rules! LPC32XX_INTC_MASK { ($x:expr) => { io_p2v!(($x) + 0x00) }; }
macro_rules! LPC32XX_INTC_RAW_STAT { ($x:expr) => { io_p2v!(($x) + 0x04) }; }
macro_rules! LPC32XX_INTC_STAT { ($x:expr) => { io_p2v!(($x) + 0x08) }; }
macro_rules! LPC32XX_INTC_POLAR { ($x:expr) => { io_p2v!(($x) + 0x0C) }; }
macro_rules! LPC32XX_INTC_ACT_TYPE { ($x:expr) => { io_p2v!(($x) + 0x10) }; }
macro_rules! LPC32XX_INTC_TYPE { ($x:expr) => { io_p2v!(($x) + 0x14) }; }

/*
 * Timer/counter register offsets
 */
macro_rules! LPC32XX_TIMER_IR { ($x:expr) => { io_p2v!(($x) + 0x00) }; }
macro_rules! LPC32XX_TIMER_TCR { ($x:expr) => { io_p2v!(($x) + 0x04) }; }
macro_rules! LPC32XX_TIMER_TC { ($x:expr) => { io_p2v!(($x) + 0x08) }; }
macro_rules! LPC32XX_TIMER_PR { ($x:expr) => { io_p2v!(($x) + 0x0C) }; }
macro_rules! LPC32XX_TIMER_PC { ($x:expr) => { io_p2v!(($x) + 0x10) }; }
macro_rules! LPC32XX_TIMER_MCR { ($x:expr) => { io_p2v!(($x) + 0x14) }; }
macro_rules! LPC32XX_TIMER_MR0 { ($x:expr) => { io_p2v!(($x) + 0x18) }; }
macro_rules! LPC32XX_TIMER_MR1 { ($x:expr) => { io_p2v!(($x) + 0x1C) }; }
macro_rules! LPC32XX_TIMER_MR2 { ($x:expr) => { io_p2v!(($x) + 0x20) }; }
macro_rules! LPC32XX_TIMER_MR3 { ($x:expr) => { io_p2v!(($x) + 0x24) }; }
macro_rules! LPC32XX_TIMER_CCR { ($x:expr) => { io_p2v!(($x) + 0x28) }; }
macro_rules! LPC32XX_TIMER_CR0 { ($x:expr) => { io_p2v!(($x) + 0x2C) }; }
macro_rules! LPC32XX_TIMER_CR1 { ($x:expr) => { io_p2v!(($x) + 0x30) }; }
macro_rules! LPC32XX_TIMER_CR2 { ($x:expr) => { io_p2v!(($x) + 0x34) }; }
macro_rules! LPC32XX_TIMER_CR3 { ($x:expr) => { io_p2v!(($x) + 0x38) }; }
macro_rules! LPC32XX_TIMER_EMR { ($x:expr) => { io_p2v!(($x) + 0x3C) }; }
macro_rules! LPC32XX_TIMER_CTCR { ($x:expr) => { io_p2v!(($x) + 0x70) }; }

/*
 * ir register definitions
 */
macro_rules! LPC32XX_TIMER_CNTR_MTCH_BIT { ($n:expr) => { (1 << (($n) & 0x3)) }; }
macro_rules! LPC32XX_TIMER_CNTR_CAPT_BIT { ($n:expr) => { (1 << (4 + (($n) & 0x3))) }; }

/*
 * tcr register definitions
 */
pub const LPC32XX_TIMER_CNTR_TCR_EN: u32 = 0x1;
pub const LPC32XX_TIMER_CNTR_TCR_RESET: u32 = 0x2;

/*
 * mcr register definitions
 */
macro_rules! LPC32XX_TIMER_CNTR_MCR_MTCH { ($n:expr) => { (0x1 << (($n) * 3)) }; }
macro_rules! LPC32XX_TIMER_CNTR_MCR_RESET { ($n:expr) => { (0x1 << ((($n) * 3) + 1)) }; }
macro_rules! LPC32XX_TIMER_CNTR_MCR_STOP { ($n:expr) => { (0x1 << ((($n) * 3) + 2)) }; }

/*
 * Standard UART register offsets
 */
macro_rules! LPC32XX_UART_DLL_FIFO { ($x:expr) => { io_p2v!(($x) + 0x00) }; }
macro_rules! LPC32XX_UART_DLM_IER { ($x:expr) => { io_p2v!(($x) + 0x04) }; }
macro_rules! LPC32XX_UART_IIR_FCR { ($x:expr) => { io_p2v!(($x) + 0x08) }; }
macro_rules! LPC32XX_UART_LCR { ($x:expr) => { io_p2v!(($x) + 0x0C) }; }
macro_rules! LPC32XX_UART_MODEM_CTRL { ($x:expr) => { io_p2v!(($x) + 0x10) }; }
macro_rules! LPC32XX_UART_LSR { ($x:expr) => { io_p2v!(($x) + 0x14) }; }
macro_rules! LPC32XX_UART_MODEM_STATUS { ($x:expr) => { io_p2v!(($x) + 0x18) }; }
macro_rules! LPC32XX_UART_RXLEV { ($x:expr) => { io_p2v!(($x) + 0x1C) }; }

/*
 * UART control structure offsets
 */
macro_rules! _UCREG { ($x:expr) => { io_p2v!( LPC32XX_UART_CTRL_BASE + ($x)) }; }
pub const LPC32XX_UARTCTL_CTRL: u32 = _UCREG(0x00);
pub const LPC32XX_UARTCTL_CLKMODE: u32 = _UCREG(0x04);
pub const LPC32XX_UARTCTL_CLOOP: u32 = _UCREG(0x08);

/*
 * ctrl register definitions
 */
pub const LPC32XX_UART_U3_MD_CTRL_EN: u32 = _BIT(11);
pub const LPC32XX_UART_IRRX6_INV_EN: u32 = _BIT(10);
pub const LPC32XX_UART_HDPX_EN: u32 = _BIT(9);
pub const LPC32XX_UART_UART6_IRDAMOD_BYPASS: u32 = _BIT(5);
pub const LPC32XX_RT_IRTX6_INV_EN: u32 = _BIT(4);
pub const LPC32XX_RT_IRTX6_INV_MIR_EN: u32 = _BIT(3);
pub const LPC32XX_RT_RX_IRPULSE_3_16_115K: u32 = _BIT(2);
pub const LPC32XX_RT_TX_IRPULSE_3_16_115K: u32 = _BIT(1);
pub const LPC32XX_UART_U5_ROUTE_TO_USB: u32 = _BIT(0);

/*
 * clkmode register definitions
 */
macro_rules! LPC32XX_UART_ENABLED_CLOCKS { ($n:expr) => { ((($n) >> 16) & 0x7F) }; }
macro_rules! LPC32XX_UART_ENABLED_CLOCK { ($n:expr, $u:expr) => { ((($n) >> (16 + ($u))) & 0x1) }; }
pub const LPC32XX_UART_ENABLED_CLKS_ANY: u32 = _BIT(14);
pub const LPC32XX_UART_CLKMODE_OFF: u32 = 0x0;
pub const LPC32XX_UART_CLKMODE_ON: u32 = 0x1;
pub const LPC32XX_UART_CLKMODE_AUTO: u32 = 0x2;
macro_rules! LPC32XX_UART_CLKMODE_MASK { ($u:expr) => { (0x3 << (((($u) - 3) * 2) + 4)) }; }
macro_rules! LPC32XX_UART_CLKMODE_LOAD { ($m:expr, $u:expr) => { (($m) << (((($u) - 3) * 2) + 4)) }; }

/*
 * GPIO Module Register offsets
 */
macro_rules! _GPREG { ($x:expr) => { io_p2v!(LPC32XX_GPIO_BASE + ($x)) }; }
pub const LPC32XX_GPIO_P_MUX_SET: u32 = _GPREG(0x100);
pub const LPC32XX_GPIO_P_MUX_CLR: u32 = _GPREG(0x104);
pub const LPC32XX_GPIO_P_MUX_STATE: u32 = _GPREG(0x108);
pub const LPC32XX_GPIO_P3_MUX_SET: u32 = _GPREG(0x110);
pub const LPC32XX_GPIO_P3_MUX_CLR: u32 = _GPREG(0x114);
pub const LPC32XX_GPIO_P3_MUX_STATE: u32 = _GPREG(0x118);
pub const LPC32XX_GPIO_P0_MUX_SET: u32 = _GPREG(0x120);
pub const LPC32XX_GPIO_P0_MUX_CLR: u32 = _GPREG(0x124);
pub const LPC32XX_GPIO_P0_MUX_STATE: u32 = _GPREG(0x128);
pub const LPC32XX_GPIO_P1_MUX_SET: u32 = _GPREG(0x130);
pub const LPC32XX_GPIO_P1_MUX_CLR: u32 = _GPREG(0x134);
pub const LPC32XX_GPIO_P1_MUX_STATE: u32 = _GPREG(0x138);
pub const LPC32XX_GPIO_P2_MUX_SET: u32 = _GPREG(0x028);
pub const LPC32XX_GPIO_P2_MUX_CLR: u32 = _GPREG(0x02C);
pub const LPC32XX_GPIO_P2_MUX_STATE: u32 = _GPREG(0x030);

/*
 * USB Otg Registers
 */
macro_rules! _OTGREG { ($x:expr) => { io_p2v!(LPC32XX_USB_OTG_BASE + ($x)) }; }
pub const LPC32XX_USB_OTG_CLK_CTRL: u32 = _OTGREG(0xFF4);
pub const LPC32XX_USB_OTG_CLK_STAT: u32 = _OTGREG(0xFF8);

/* USB OTG CLK CTRL bit defines */
pub const LPC32XX_USB_OTG_AHB_M_CLOCK_ON: u32 = _BIT(4);
pub const LPC32XX_USB_OTG_OTG_CLOCK_ON: u32 = _BIT(3);
pub const LPC32XX_USB_OTG_I2C_CLOCK_ON: u32 = _BIT(2);
pub const LPC32XX_USB_OTG_DEV_CLOCK_ON: u32 = _BIT(1);
pub const LPC32XX_USB_OTG_HOST_CLOCK_ON: u32 = _BIT(0);

/*
 * Start of virtual addresses for IO devices
 */
pub const IO_BASE: u32 = 0xF0000000;

/*
 * This macro relies on fact that for all HW i/o addresses bits 20-23 are 0
 */
macro_rules! IO_ADDRESS { ($x:expr) => { IOMEM!((((($x) & 0xff000000) >> 4) | (($x) & 0xfffff)) | IO_BASE) }; }

macro_rules! io_p2v { ($x:expr) => { (( *) () IO_ADDRESS!($x)) }; }
macro_rules! io_v2p { ($x:expr) => { (((($x) & 0x0ff00000) << 4) | (($x) & 0x000fffff)) }; }



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
