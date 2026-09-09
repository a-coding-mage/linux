/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2019-20 Sean Anderson <seanga2@gmail.com>
 * Copyright (c) 2020 Western Digital Corporation or its affiliates.
 */

/*
 * Kendryte K210 SoC system controller registers offsets.
 * Taken from Kendryte SDK (kendryte-standalone-sdk).
 */
pub const K210_SYSCTL_GIT_ID: u32 = 0x00; // Git short commit id
pub const K210_SYSCTL_UART_BAUD: u32 = 0x04; // Default UARTHS baud rate
pub const K210_SYSCTL_PLL0: u32 = 0x08; // PLL0 controller
pub const K210_SYSCTL_PLL1: u32 = 0x0C; // PLL1 controller
pub const K210_SYSCTL_PLL2: u32 = 0x10; // PLL2 controller
pub const K210_SYSCTL_PLL_LOCK: u32 = 0x18; // PLL lock tester
pub const K210_SYSCTL_ROM_ERROR: u32 = 0x1C; // AXI ROM detector
pub const K210_SYSCTL_SEL0: u32 = 0x20; // Clock select controller 0
pub const K210_SYSCTL_SEL1: u32 = 0x24; // Clock select controller 1
pub const K210_SYSCTL_EN_CENT: u32 = 0x28; // Central clock enable
pub const K210_SYSCTL_EN_PERI: u32 = 0x2C; // Peripheral clock enable
pub const K210_SYSCTL_SOFT_RESET: u32 = 0x30; // Soft reset ctrl
pub const K210_SYSCTL_PERI_RESET: u32 = 0x34; // Peripheral reset controller
pub const K210_SYSCTL_THR0: u32 = 0x38; // Clock threshold controller 0
pub const K210_SYSCTL_THR1: u32 = 0x3C; // Clock threshold controller 1
pub const K210_SYSCTL_THR2: u32 = 0x40; // Clock threshold controller 2
pub const K210_SYSCTL_THR3: u32 = 0x44; // Clock threshold controller 3
pub const K210_SYSCTL_THR4: u32 = 0x48; // Clock threshold controller 4
pub const K210_SYSCTL_THR5: u32 = 0x4C; // Clock threshold controller 5
pub const K210_SYSCTL_THR6: u32 = 0x50; // Clock threshold controller 6
pub const K210_SYSCTL_MISC: u32 = 0x54; // Miscellaneous controller
pub const K210_SYSCTL_PERI: u32 = 0x58; // Peripheral controller
pub const K210_SYSCTL_SPI_SLEEP: u32 = 0x5C; // SPI sleep controller
pub const K210_SYSCTL_RESET_STAT: u32 = 0x60; // Reset source status
pub const K210_SYSCTL_DMA_SEL0: u32 = 0x64; // DMA handshake selector 0
pub const K210_SYSCTL_DMA_SEL1: u32 = 0x68; // DMA handshake selector 1
pub const K210_SYSCTL_POWER_SEL: u32 = 0x6C; // IO Power Mode Select controller

unsafe extern "C" {
    pub fn k210_clk_early_init(regs: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
