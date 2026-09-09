// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 Oleksij Rempel <linux@rempel-privat.de>.
 */

// Kernel dependencies supplied by the surrounding Rust translation environment.

const HW_AHBCLKCTRL0: u32 = 0x0020;
const HW_AHBCLKCTRL1: u32 = 0x0030;
const HW_SYSPLLCTRL: u32 = 0x0100;
const HW_MAINCLKSEL: u32 = 0x0120;
const HW_MAINCLKUEN: u32 = 0x0124;
const HW_UARTCLKSEL: u32 = 0x0128;
const HW_UARTCLKUEN: u32 = 0x012c;
const HW_I2S0CLKSEL: u32 = 0x0130;
const HW_I2S0CLKUEN: u32 = 0x0134;
const HW_I2S1CLKSEL: u32 = 0x0138;
const HW_I2S1CLKUEN: u32 = 0x013c;
const HW_WDTCLKSEL: u32 = 0x0160;
const HW_WDTCLKUEN: u32 = 0x0164;
const HW_CLKOUTCLKSEL: u32 = 0x0170;
const HW_CLKOUTCLKUEN: u32 = 0x0174;
const HW_CPUCLKDIV: u32 = 0x017c;
const HW_SYSAHBCLKDIV: u32 = 0x0180;
const HW_I2S0MCLKDIV: u32 = 0x0190;
const HW_I2S0SCLKDIV: u32 = 0x0194;
const HW_I2S1MCLKDIV: u32 = 0x0188;
const HW_I2S1SCLKDIV: u32 = 0x018c;
const HW_UART0CLKDIV: u32 = 0x0198;
const HW_UART1CLKDIV: u32 = 0x019c;
const HW_UART2CLKDIV: u32 = 0x01a0;
const HW_UART3CLKDIV: u32 = 0x01a4;
const HW_UART4CLKDIV: u32 = 0x01a8;
const HW_UART5CLKDIV: u32 = 0x01ac;
const HW_UART6CLKDIV: u32 = 0x01b0;
const HW_UART7CLKDIV: u32 = 0x01b4;
const HW_UART8CLKDIV: u32 = 0x01b8;
const HW_UART9CLKDIV: u32 = 0x01bc;
const HW_SPI0CLKDIV: u32 = 0x01c0;
const HW_SPI1CLKDIV: u32 = 0x01c4;
const HW_QUADSPICLKDIV: u32 = 0x01c8;
const HW_SSP0CLKDIV: u32 = 0x01d0;
const HW_NANDCLKDIV: u32 = 0x01d4;
const HW_TRACECLKDIV: u32 = 0x01e0;
const HW_CAMMCLKDIV: u32 = 0x01e8;
const HW_WDTCLKDIV: u32 = 0x01ec;
const HW_CLKOUTCLKDIV: u32 = 0x01f4;
const HW_MACCLKDIV: u32 = 0x01f8;
const HW_LCDCLKDIV: u32 = 0x01fc;
const HW_ADCANACLKDIV: u32 = 0x0200;

static mut clk_data: *mut clk_hw_onecell_data = core::ptr::null_mut();
static mut asm9260_clk_lock: spinlock_t = DEFINE_SPINLOCK!();

#[repr(C)]
struct asm9260_div_clk { idx: u32, name: *const core::ffi::c_char, parent_name: *const core::ffi::c_char, reg: u32 }
#[repr(C)]
struct asm9260_gate_data { idx: u32, name: *const core::ffi::c_char, parent_name: *const core::ffi::c_char, reg: u32, bit_idx: u8, flags: c_ulong }
#[repr(C)]
struct asm9260_mux_clock { mask: u8, table: *mut u32, name: *const core::ffi::c_char, parent_data: *const clk_parent_data, num_parents: u8, offset: c_ulong, flags: c_ulong }

static mut base: *mut core::ffi::c_void = core::ptr::null_mut();

static asm9260_div_clks: &[asm9260_div_clk] = &[
    asm9260_div_clk { idx: CLKID_SYS_CPU, name: c"cpu_div".as_ptr(), parent_name: c"main_gate".as_ptr(), reg: HW_CPUCLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_AHB, name: c"ahb_div".as_ptr(), parent_name: c"cpu_div".as_ptr(), reg: HW_SYSAHBCLKDIV },
    // i2s has two dividers: one for only external mclk and internal divider for all clks.
    asm9260_div_clk { idx: CLKID_SYS_I2S0M, name: c"i2s0m_div".as_ptr(), parent_name: c"i2s0_mclk".as_ptr(), reg: HW_I2S0MCLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_I2S1M, name: c"i2s1m_div".as_ptr(), parent_name: c"i2s1_mclk".as_ptr(), reg: HW_I2S1MCLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_I2S0S, name: c"i2s0s_div".as_ptr(), parent_name: c"i2s0_gate".as_ptr(), reg: HW_I2S0SCLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_I2S1S, name: c"i2s1s_div".as_ptr(), parent_name: c"i2s0_gate".as_ptr(), reg: HW_I2S1SCLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_UART0, name: c"uart0_div".as_ptr(), parent_name: c"uart_gate".as_ptr(), reg: HW_UART0CLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_UART1, name: c"uart1_div".as_ptr(), parent_name: c"uart_gate".as_ptr(), reg: HW_UART1CLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_UART2, name: c"uart2_div".as_ptr(), parent_name: c"uart_gate".as_ptr(), reg: HW_UART2CLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_UART3, name: c"uart3_div".as_ptr(), parent_name: c"uart_gate".as_ptr(), reg: HW_UART3CLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_UART4, name: c"uart4_div".as_ptr(), parent_name: c"uart_gate".as_ptr(), reg: HW_UART4CLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_UART5, name: c"uart5_div".as_ptr(), parent_name: c"uart_gate".as_ptr(), reg: HW_UART5CLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_UART6, name: c"uart6_div".as_ptr(), parent_name: c"uart_gate".as_ptr(), reg: HW_UART6CLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_UART7, name: c"uart7_div".as_ptr(), parent_name: c"uart_gate".as_ptr(), reg: HW_UART7CLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_UART8, name: c"uart8_div".as_ptr(), parent_name: c"uart_gate".as_ptr(), reg: HW_UART8CLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_UART9, name: c"uart9_div".as_ptr(), parent_name: c"uart_gate".as_ptr(), reg: HW_UART9CLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_SPI0, name: c"spi0_div".as_ptr(), parent_name: c"main_gate".as_ptr(), reg: HW_SPI0CLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_SPI1, name: c"spi1_div".as_ptr(), parent_name: c"main_gate".as_ptr(), reg: HW_SPI1CLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_QUADSPI, name: c"quadspi_div".as_ptr(), parent_name: c"main_gate".as_ptr(), reg: HW_QUADSPICLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_SSP0, name: c"ssp0_div".as_ptr(), parent_name: c"main_gate".as_ptr(), reg: HW_SSP0CLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_NAND, name: c"nand_div".as_ptr(), parent_name: c"main_gate".as_ptr(), reg: HW_NANDCLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_TRACE, name: c"trace_div".as_ptr(), parent_name: c"main_gate".as_ptr(), reg: HW_TRACECLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_CAMM, name: c"camm_div".as_ptr(), parent_name: c"main_gate".as_ptr(), reg: HW_CAMMCLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_MAC, name: c"mac_div".as_ptr(), parent_name: c"main_gate".as_ptr(), reg: HW_MACCLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_LCD, name: c"lcd_div".as_ptr(), parent_name: c"main_gate".as_ptr(), reg: HW_LCDCLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_ADCANA, name: c"adcana_div".as_ptr(), parent_name: c"main_gate".as_ptr(), reg: HW_ADCANACLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_WDT, name: c"wdt_div".as_ptr(), parent_name: c"wdt_gate".as_ptr(), reg: HW_WDTCLKDIV },
    asm9260_div_clk { idx: CLKID_SYS_CLKOUT, name: c"clkout_div".as_ptr(), parent_name: c"clkout_gate".as_ptr(), reg: HW_CLKOUTCLKDIV },
];

static asm9260_mux_gates: &[asm9260_gate_data] = &[
    asm9260_gate_data { idx: 0, name: c"main_gate".as_ptr(), parent_name: c"main_mux".as_ptr(), reg: HW_MAINCLKUEN, bit_idx: 0, flags: 0 },
    asm9260_gate_data { idx: 0, name: c"uart_gate".as_ptr(), parent_name: c"uart_mux".as_ptr(), reg: HW_UARTCLKUEN, bit_idx: 0, flags: 0 },
    asm9260_gate_data { idx: 0, name: c"i2s0_gate".as_ptr(), parent_name: c"i2s0_mux".as_ptr(), reg: HW_I2S0CLKUEN, bit_idx: 0, flags: 0 },
    asm9260_gate_data { idx: 0, name: c"i2s1_gate".as_ptr(), parent_name: c"i2s1_mux".as_ptr(), reg: HW_I2S1CLKUEN, bit_idx: 0, flags: 0 },
    asm9260_gate_data { idx: 0, name: c"wdt_gate".as_ptr(), parent_name: c"wdt_mux".as_ptr(), reg: HW_WDTCLKUEN, bit_idx: 0, flags: 0 },
    asm9260_gate_data { idx: 0, name: c"clkout_gate".as_ptr(), parent_name: c"clkout_mux".as_ptr(), reg: HW_CLKOUTCLKUEN, bit_idx: 0, flags: 0 },
];

// AHB gate table. Values and names mirror the device-tree clock binding.
static asm9260_ahb_gates: &[asm9260_gate_data] = &[
    asm9260_gate_data { idx: CLKID_AHB_ROM, name: c"rom".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 1, flags: CLK_IGNORE_UNUSED },
    asm9260_gate_data { idx: CLKID_AHB_RAM, name: c"ram".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 2, flags: CLK_IGNORE_UNUSED },
    asm9260_gate_data { idx: CLKID_AHB_GPIO, name: c"gpio".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 4, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_MAC, name: c"mac".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 5, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_EMI, name: c"emi".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 6, flags: CLK_IGNORE_UNUSED },
    asm9260_gate_data { idx: CLKID_AHB_USB0, name: c"usb0".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 7, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_USB1, name: c"usb1".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 8, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_DMA0, name: c"dma0".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 9, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_DMA1, name: c"dma1".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 10, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_UART0, name: c"uart0".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 11, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_UART1, name: c"uart1".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 12, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_UART2, name: c"uart2".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 13, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_UART3, name: c"uart3".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 14, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_UART4, name: c"uart4".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 15, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_UART5, name: c"uart5".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 16, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_UART6, name: c"uart6".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 17, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_UART7, name: c"uart7".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 18, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_UART8, name: c"uart8".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 19, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_UART9, name: c"uart9".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 20, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_I2S0, name: c"i2s0".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 21, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_I2C0, name: c"i2c0".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 22, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_I2C1, name: c"i2c1".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 23, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_SSP0, name: c"ssp0".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 24, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_IOCONFIG, name: c"ioconf".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 25, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_WDT, name: c"wdt".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 26, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_CAN0, name: c"can0".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 27, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_CAN1, name: c"can1".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 28, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_MPWM, name: c"mpwm".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 29, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_SPI0, name: c"spi0".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 30, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_SPI1, name: c"spi1".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL0, bit_idx: 31, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_QEI, name: c"qei".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 0, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_QUADSPI0, name: c"quadspi0".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 1, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_CAMIF, name: c"capmif".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 2, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_LCDIF, name: c"lcdif".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 3, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_TIMER0, name: c"timer0".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 4, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_TIMER1, name: c"timer1".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 5, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_TIMER2, name: c"timer2".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 6, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_TIMER3, name: c"timer3".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 7, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_IRQ, name: c"irq".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 8, flags: CLK_IGNORE_UNUSED },
    asm9260_gate_data { idx: CLKID_AHB_RTC, name: c"rtc".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 9, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_NAND, name: c"nand".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 10, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_ADC0, name: c"adc0".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 11, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_LED, name: c"led".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 12, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_DAC0, name: c"dac0".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 13, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_LCD, name: c"lcd".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 14, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_I2S1, name: c"i2s1".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 15, flags: 0 },
    asm9260_gate_data { idx: CLKID_AHB_MAC1, name: c"mac1".as_ptr(), parent_name: c"ahb_div".as_ptr(), reg: HW_AHBCLKCTRL1, bit_idx: 16, flags: 0 },
];

static mut main_mux_p: [clk_parent_data; 2] = [clk_parent_data { index: 0 }, clk_parent_data { name: c"pll".as_ptr() }];
static mut i2s0_mux_p: [clk_parent_data; 3] = [clk_parent_data { index: 0 }, clk_parent_data { name: c"pll".as_ptr() }, clk_parent_data { name: c"i2s0m_div".as_ptr() }];
static mut i2s1_mux_p: [clk_parent_data; 3] = [clk_parent_data { index: 0 }, clk_parent_data { name: c"pll".as_ptr() }, clk_parent_data { name: c"i2s1m_div".as_ptr() }];
static mut clkout_mux_p: [clk_parent_data; 3] = [clk_parent_data { index: 0 }, clk_parent_data { name: c"pll".as_ptr() }, clk_parent_data { name: c"rtc".as_ptr() }];
static mut three_mux_table: [u32; 3] = [0, 1, 3];

static mut asm9260_mux_clks: [asm9260_mux_clock; 6] = [
    asm9260_mux_clock { mask: 1, table: three_mux_table.as_mut_ptr(), name: c"main_mux".as_ptr(), parent_data: main_mux_p.as_ptr(), num_parents: 2, offset: HW_MAINCLKSEL as c_ulong, flags: 0 },
    asm9260_mux_clock { mask: 1, table: three_mux_table.as_mut_ptr(), name: c"uart_mux".as_ptr(), parent_data: main_mux_p.as_ptr(), num_parents: 2, offset: HW_UARTCLKSEL as c_ulong, flags: 0 },
    asm9260_mux_clock { mask: 1, table: three_mux_table.as_mut_ptr(), name: c"wdt_mux".as_ptr(), parent_data: main_mux_p.as_ptr(), num_parents: 2, offset: HW_WDTCLKSEL as c_ulong, flags: 0 },
    asm9260_mux_clock { mask: 3, table: three_mux_table.as_mut_ptr(), name: c"i2s0_mux".as_ptr(), parent_data: i2s0_mux_p.as_ptr(), num_parents: 3, offset: HW_I2S0CLKSEL as c_ulong, flags: 0 },
    asm9260_mux_clock { mask: 3, table: three_mux_table.as_mut_ptr(), name: c"i2s1_mux".as_ptr(), parent_data: i2s1_mux_p.as_ptr(), num_parents: 3, offset: HW_I2S1CLKSEL as c_ulong, flags: 0 },
    asm9260_mux_clock { mask: 3, table: three_mux_table.as_mut_ptr(), name: c"clkout_mux".as_ptr(), parent_data: clkout_mux_p.as_ptr(), num_parents: 3, offset: HW_CLKOUTCLKSEL as c_ulong, flags: 0 },
];

unsafe fn asm9260_acc_init(np: *mut device_node) {
    let mut pll_hw: *mut clk_hw;
    let mut hws: *mut *mut clk_hw;
    let pll_clk = c"pll".as_ptr();
    let pll_parent_data = clk_parent_data { index: 0 };
    let mut rate: u32;
    let mut n: usize;

    clk_data = kzalloc_flex::<clk_hw_onecell_data>(*clk_data, &mut hws, MAX_CLKS);
    if clk_data.is_null() { return; }
    (*clk_data).num = MAX_CLKS;
    hws = (*clk_data).hws;
    base = of_io_request_and_map(np, 0, (*np).name);
    if IS_ERR(base) { panic!("unable to map resource"); }
    rate = (ioread32(base.add(HW_SYSPLLCTRL as usize)) & 0xffff).wrapping_mul(1_000_000);
    pll_hw = clk_hw_register_fixed_rate_parent_accuracy(core::ptr::null_mut(), pll_clk, &pll_parent_data, 0, rate);
    if IS_ERR(pll_hw) { panic!("can't register REFCLK. Check DT!"); }

    for mc in asm9260_mux_clks.iter() {
        clk_hw_register_mux_table_parent_data(core::ptr::null_mut(), mc.name, mc.parent_data, mc.num_parents, mc.flags, base.add(mc.offset as usize), 0, mc.mask, 0, mc.table, &mut asm9260_clk_lock);
    }
    for gd in asm9260_mux_gates.iter() {
        clk_hw_register_gate(core::ptr::null_mut(), gd.name, gd.parent_name, gd.flags | CLK_SET_RATE_PARENT, base.add(gd.reg as usize), gd.bit_idx, 0, &mut asm9260_clk_lock);
    }
    for dc in asm9260_div_clks.iter() {
        *hws.add(dc.idx as usize) = clk_hw_register_divider(core::ptr::null_mut(), dc.name, dc.parent_name, CLK_SET_RATE_PARENT, base.add(dc.reg as usize), 0, 8, CLK_DIVIDER_ONE_BASED, &mut asm9260_clk_lock);
    }
    for gd in asm9260_ahb_gates.iter() {
        *hws.add(gd.idx as usize) = clk_hw_register_gate(core::ptr::null_mut(), gd.name, gd.parent_name, gd.flags, base.add(gd.reg as usize), gd.bit_idx, 0, &mut asm9260_clk_lock);
    }
    n = 0;
    while n < MAX_CLKS as usize {
        if !IS_ERR(*hws.add(n)) { n += 1; continue; }
        pr_err!("Unable to register leaf clock {}", n);
        iounmap(base);
        return;
    }
    of_clk_add_hw_provider(np, of_clk_hw_onecell_get, clk_data);
}

// CLK_OF_DECLARE(asm9260_acc, "alphascale,asm9260-clock-controller", asm9260_acc_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
