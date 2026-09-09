/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2010 John Crispin <john@phrozen.org>
 */

/* CONFIG_SOC_FALCON conditional from the original header. */

/* Dependencies supplied by other translated headers. */

/* Chip IDs */
pub const SOC_ID_FALCON: u32 = 0x01B8;

/* SoC Types */
pub const SOC_TYPE_FALCON: u32 = 0x01;

/*
 * during early_printk no ioremap possible at this early stage
 * let's use KSEG1 instead
 */
pub const LTQ_ASC0_BASE_ADDR: usize = 0x1E100C00;
/* KSEG1ADDR(LTQ_ASC0_BASE_ADDR), supplied by the platform headers. */
pub const LTQ_EARLY_ASC: usize = KSEG1ADDR(LTQ_ASC0_BASE_ADDR);

/* WDT */
pub const LTQ_RST_CAUSE_WDTRST: u32 = 0x0002;

/* CHIP ID */
pub const LTQ_STATUS_BASE_ADDR: usize = 0x1E802000;

pub const FALCON_CHIPID: *mut u32 = (KSEG1 + LTQ_STATUS_BASE_ADDR + 0x0c) as *mut u32;
pub const FALCON_CHIPTYPE: *mut u32 = (KSEG1 + LTQ_STATUS_BASE_ADDR + 0x38) as *mut u32;
pub const FALCON_CHIPCONF: *mut u32 = (KSEG1 + LTQ_STATUS_BASE_ADDR + 0x40) as *mut u32;

/* SYSCTL - start/stop/restart/configure/... different parts of the Soc */
pub const SYSCTL_SYS1: u32 = 0;
pub const SYSCTL_SYSETH: u32 = 1;
pub const SYSCTL_SYSGPE: u32 = 2;

/* BOOT_SEL - find what boot media we have */
pub const BS_FLASH: u32 = 0x1;
pub const BS_SPI: u32 = 0x4;

/* global register ranges */
extern "C" {
    pub static mut ltq_ebu_membase: *mut core::ffi::c_void;
    pub static mut ltq_sys1_membase: *mut core::ffi::c_void;

    pub fn pinctrl_falcon_get_range_size(id: core::ffi::c_int) -> core::ffi::c_int;
    pub fn pinctrl_falcon_add_gpio_range(range: *mut crate::pinctrl_gpio_range);
}

#[macro_export]
macro_rules! ltq_ebu_w32 {
    ($x:expr, $y:expr) => {
        ltq_w32(($x), unsafe { ltq_ebu_membase.add($y as usize) })
    };
}

#[macro_export]
macro_rules! ltq_ebu_r32 {
    ($x:expr) => {
        ltq_r32(unsafe { ltq_ebu_membase.add($x as usize) })
    };
}

#[macro_export]
macro_rules! ltq_sys1_w32 {
    ($x:expr, $y:expr) => {
        ltq_w32(($x), unsafe { ltq_sys1_membase.add($y as usize) })
    };
}

#[macro_export]
macro_rules! ltq_sys1_r32 {
    ($x:expr) => {
        ltq_r32(unsafe { ltq_sys1_membase.add($x as usize) })
    };
}

#[macro_export]
macro_rules! ltq_sys1_w32_mask {
    ($clear:expr, $set:expr, $reg:expr) => {
        ltq_sys1_w32!((ltq_sys1_r32!($reg) & !($clear)) | ($set), $reg)
    };
}

/*
 * to keep the irq code generic we need to define this to 0 as falcon
 * has no EIU/EBU
 */
pub const LTQ_EBU_PCC_ISTAT: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
