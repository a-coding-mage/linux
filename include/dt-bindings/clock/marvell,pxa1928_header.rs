/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Clock ID values here correspond to the control register offset/4.
 */

/* apb peripherals */
pub const PXA1928_CLK_RTC: u32 = 0x00;
pub const PXA1928_CLK_TWSI0: u32 = 0x01;
pub const PXA1928_CLK_TWSI1: u32 = 0x02;
pub const PXA1928_CLK_TWSI2: u32 = 0x03;
pub const PXA1928_CLK_TWSI3: u32 = 0x04;
pub const PXA1928_CLK_OWIRE: u32 = 0x05;
pub const PXA1928_CLK_KPC: u32 = 0x06;
pub const PXA1928_CLK_TB_ROTARY: u32 = 0x07;
pub const PXA1928_CLK_SW_JTAG: u32 = 0x08;
pub const PXA1928_CLK_TIMER1: u32 = 0x09;
pub const PXA1928_CLK_UART0: u32 = 0x0b;
pub const PXA1928_CLK_UART1: u32 = 0x0c;
pub const PXA1928_CLK_UART2: u32 = 0x0d;
pub const PXA1928_CLK_GPIO: u32 = 0x0e;
pub const PXA1928_CLK_PWM0: u32 = 0x0f;
pub const PXA1928_CLK_PWM1: u32 = 0x10;
pub const PXA1928_CLK_PWM2: u32 = 0x11;
pub const PXA1928_CLK_PWM3: u32 = 0x12;
pub const PXA1928_CLK_SSP0: u32 = 0x13;
pub const PXA1928_CLK_SSP1: u32 = 0x14;
pub const PXA1928_CLK_SSP2: u32 = 0x15;

pub const PXA1928_CLK_TWSI4: u32 = 0x1f;
pub const PXA1928_CLK_TWSI5: u32 = 0x20;
pub const PXA1928_CLK_UART3: u32 = 0x22;
pub const PXA1928_CLK_THSENS_GLOB: u32 = 0x24;
pub const PXA1928_CLK_THSENS_CPU: u32 = 0x26;
pub const PXA1928_CLK_THSENS_VPU: u32 = 0x27;
pub const PXA1928_CLK_THSENS_GC: u32 = 0x28;

/* axi peripherals */
pub const PXA1928_CLK_SDH0: u32 = 0x15;
pub const PXA1928_CLK_SDH1: u32 = 0x16;
pub const PXA1928_CLK_USB: u32 = 0x17;
pub const PXA1928_CLK_NAND: u32 = 0x18;
pub const PXA1928_CLK_DMA: u32 = 0x19;

pub const PXA1928_CLK_SDH2: u32 = 0x3a;
pub const PXA1928_CLK_SDH3: u32 = 0x3b;
pub const PXA1928_CLK_HSIC: u32 = 0x3e;
pub const PXA1928_CLK_SDH4: u32 = 0x57;
pub const PXA1928_CLK_GC3D: u32 = 0x5d;
pub const PXA1928_CLK_GC2D: u32 = 0x5f;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
