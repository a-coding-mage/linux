/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Parts of this file are based on Ralink's 2.6.21 BSP
 *
 * Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 * Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

extern "C" {
    pub static mut ralink_soc: ralink_soc_type;
}

pub unsafe fn soc_is_rt3050() -> i32 {
    (ralink_soc == RT305X_SOC_RT3050) as i32
}

pub unsafe fn soc_is_rt3052() -> i32 {
    (ralink_soc == RT305X_SOC_RT3052) as i32
}

pub unsafe fn soc_is_rt305x() -> i32 {
    (soc_is_rt3050() != 0 || soc_is_rt3052() != 0) as i32
}

pub unsafe fn soc_is_rt3350() -> i32 {
    (ralink_soc == RT305X_SOC_RT3350) as i32
}

pub unsafe fn soc_is_rt3352() -> i32 {
    (ralink_soc == RT305X_SOC_RT3352) as i32
}

pub unsafe fn soc_is_rt5350() -> i32 {
    (ralink_soc == RT305X_SOC_RT5350) as i32
}

#[macro_export]
macro_rules! IOMEM {
    ($x:expr) => {
        (KSEG1ADDR($x) as *mut core::ffi::c_void)
    };
}

#[macro_export]
macro_rules! RT305X_SYSC_BASE {
    () => { IOMEM!(0x10000000) };
}

pub const SYSC_REG_CHIP_NAME0: u32 = 0x00;
pub const SYSC_REG_CHIP_NAME1: u32 = 0x04;
pub const SYSC_REG_CHIP_ID: u32 = 0x0c;
pub const SYSC_REG_SYSTEM_CONFIG: u32 = 0x10;

pub const RT3052_CHIP_NAME0: u32 = 0x30335452;
pub const RT3052_CHIP_NAME1: u32 = 0x20203235;

pub const RT3350_CHIP_NAME0: u32 = 0x33335452;
pub const RT3350_CHIP_NAME1: u32 = 0x20203035;

pub const RT3352_CHIP_NAME0: u32 = 0x33335452;
pub const RT3352_CHIP_NAME1: u32 = 0x20203235;

pub const RT5350_CHIP_NAME0: u32 = 0x33355452;
pub const RT5350_CHIP_NAME1: u32 = 0x20203035;

pub const CHIP_ID_ID_MASK: u32 = 0xff;
pub const CHIP_ID_ID_SHIFT: u32 = 8;
pub const CHIP_ID_REV_MASK: u32 = 0xff;

pub const RT305X_SYSCFG_SRAM_CS0_MODE_SHIFT: u32 = 2;
pub const RT305X_SYSCFG_SRAM_CS0_MODE_WDT: u32 = 0x1;

pub const RT5350_SYSCFG0_DRAM_SIZE_SHIFT: u32 = 12;
pub const RT5350_SYSCFG0_DRAM_SIZE_MASK: u32 = 7;
pub const RT5350_SYSCFG0_DRAM_SIZE_2M: u32 = 0;
pub const RT5350_SYSCFG0_DRAM_SIZE_8M: u32 = 1;
pub const RT5350_SYSCFG0_DRAM_SIZE_16M: u32 = 2;
pub const RT5350_SYSCFG0_DRAM_SIZE_32M: u32 = 3;
pub const RT5350_SYSCFG0_DRAM_SIZE_64M: u32 = 4;

/* multi function gpio pins */
pub const RT305X_GPIO_I2C_SD: u32 = 1;
pub const RT305X_GPIO_I2C_SCLK: u32 = 2;
pub const RT305X_GPIO_SPI_EN: u32 = 3;
pub const RT305X_GPIO_SPI_CLK: u32 = 4;
/* GPIO 7-14 is shared between UART0, PCM  and I2S interfaces */
pub const RT305X_GPIO_7: u32 = 7;
pub const RT305X_GPIO_10: u32 = 10;
pub const RT305X_GPIO_14: u32 = 14;
pub const RT305X_GPIO_UART1_TXD: u32 = 15;
pub const RT305X_GPIO_UART1_RXD: u32 = 16;
pub const RT305X_GPIO_JTAG_TDO: u32 = 17;
pub const RT305X_GPIO_JTAG_TDI: u32 = 18;
pub const RT305X_GPIO_MDIO_MDC: u32 = 22;
pub const RT305X_GPIO_MDIO_MDIO: u32 = 23;
pub const RT305X_GPIO_SDRAM_MD16: u32 = 24;
pub const RT305X_GPIO_SDRAM_MD31: u32 = 39;
pub const RT305X_GPIO_GE0_TXD0: u32 = 40;
pub const RT305X_GPIO_GE0_RXCLK: u32 = 51;

pub const RT3352_SYSC_REG_SYSCFG0: u32 = 0x010;
pub const RT3352_SYSC_REG_SYSCFG1: u32 = 0x014;
pub const RT3352_SYSC_REG_RSTCTRL: u32 = 0x034;
pub const RT3352_SYSC_REG_USB_PS: u32 = 0x05c;

pub const RT3352_RSTCTRL_UHST: u32 = 1u32 << 22;
pub const RT3352_RSTCTRL_UDEV: u32 = 1u32 << 25;
pub const RT3352_SYSCFG1_USB0_HOST_MODE: u32 = 1u32 << 10;

pub const RT305X_SDRAM_BASE: u32 = 0x00000000;
pub const RT305X_MEM_SIZE_MIN: u32 = 2;
pub const RT305X_MEM_SIZE_MAX: u32 = 64;
pub const RT3352_MEM_SIZE_MIN: u32 = 2;
pub const RT3352_MEM_SIZE_MAX: u32 = 256;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
