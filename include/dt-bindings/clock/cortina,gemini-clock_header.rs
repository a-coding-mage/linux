/* SPDX-License-Identifier: GPL-2.0 */

/* RTC, AHB, APB, CPU, PCI, TVC, UART clocks and 13 gates */
pub const GEMINI_NUM_CLKS: i32 = 20;

pub const GEMINI_CLK_RTC: i32 = 0;
pub const GEMINI_CLK_AHB: i32 = 1;
pub const GEMINI_CLK_APB: i32 = 2;
pub const GEMINI_CLK_CPU: i32 = 3;
pub const GEMINI_CLK_PCI: i32 = 4;
pub const GEMINI_CLK_TVC: i32 = 5;
pub const GEMINI_CLK_UART: i32 = 6;
pub const GEMINI_CLK_GATES: i32 = 7;
pub const GEMINI_CLK_GATE_SECURITY: i32 = 7;
pub const GEMINI_CLK_GATE_GMAC0: i32 = 8;
pub const GEMINI_CLK_GATE_GMAC1: i32 = 9;
pub const GEMINI_CLK_GATE_SATA0: i32 = 10;
pub const GEMINI_CLK_GATE_SATA1: i32 = 11;
pub const GEMINI_CLK_GATE_USB0: i32 = 12;
pub const GEMINI_CLK_GATE_USB1: i32 = 13;
pub const GEMINI_CLK_GATE_IDE: i32 = 14;
pub const GEMINI_CLK_GATE_PCI: i32 = 15;
pub const GEMINI_CLK_GATE_DDR: i32 = 16;
pub const GEMINI_CLK_GATE_FLASH: i32 = 17;
pub const GEMINI_CLK_GATE_TVC: i32 = 18;
pub const GEMINI_CLK_GATE_BOOT: i32 = 19;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
