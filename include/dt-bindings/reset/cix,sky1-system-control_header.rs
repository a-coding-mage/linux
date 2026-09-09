/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Author: Jerry Zhu <jerry.zhu@cixtech.com> */

/* func reset for sky1 fch */
pub const SW_I3C0_RST_FUNC_G_N: u32 = 0;
pub const SW_I3C0_RST_FUNC_I_N: u32 = 1;
pub const SW_I3C1_RST_FUNC_G_N: u32 = 2;
pub const SW_I3C1_RST_FUNC_I_N: u32 = 3;
pub const SW_UART0_RST_FUNC_N: u32 = 4;
pub const SW_UART1_RST_FUNC_N: u32 = 5;
pub const SW_UART2_RST_FUNC_N: u32 = 6;
pub const SW_UART3_RST_FUNC_N: u32 = 7;
pub const SW_TIMER_RST_FUNC_N: u32 = 8;

/* apb reset for sky1 fch */
pub const SW_I3C0_RST_APB_N: u32 = 9;
pub const SW_I3C1_RST_APB_N: u32 = 10;
pub const SW_DMA_RST_AXI_N: u32 = 11;
pub const SW_UART0_RST_APB_N: u32 = 12;
pub const SW_UART1_RST_APB_N: u32 = 13;
pub const SW_UART2_RST_APB_N: u32 = 14;
pub const SW_UART3_RST_APB_N: u32 = 15;
pub const SW_SPI0_RST_APB_N: u32 = 16;
pub const SW_SPI1_RST_APB_N: u32 = 17;
pub const SW_I2C0_RST_APB_N: u32 = 18;
pub const SW_I2C1_RST_APB_N: u32 = 19;
pub const SW_I2C2_RST_APB_N: u32 = 20;
pub const SW_I2C3_RST_APB_N: u32 = 21;
pub const SW_I2C4_RST_APB_N: u32 = 22;
pub const SW_I2C5_RST_APB_N: u32 = 23;
pub const SW_I2C6_RST_APB_N: u32 = 24;
pub const SW_I2C7_RST_APB_N: u32 = 25;
pub const SW_GPIO_RST_APB_N: u32 = 26;

/* fch rst for xspi */
pub const SW_XSPI_REG_RST_N: u32 = 27;
pub const SW_XSPI_SYS_RST_N: u32 = 28;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
