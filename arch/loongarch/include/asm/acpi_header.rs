/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Author: Jianmin Lv <lvjianmin@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the corresponding assembly and ACPI components:
// <asm/smp.h>, <asm/suspend.h>

#[cfg(feature = "CONFIG_ACPI")]
extern "C" {
    pub static mut acpi_strict: ::core::ffi::c_int;
    pub static mut acpi_disabled: ::core::ffi::c_int;
    pub static mut acpi_pci_disabled: ::core::ffi::c_int;
    pub static mut acpi_noirq: ::core::ffi::c_int;
    pub static mut pptt_enabled: ::core::ffi::c_int;

    pub fn acpi_os_ioremap(
        phys: acpi_physical_address,
        size: acpi_size,
    ) -> *mut ::core::ffi::c_void;
}

#[cfg(feature = "CONFIG_ACPI")]
#[inline]
pub unsafe fn disable_acpi() {
    acpi_disabled = 1;
    acpi_pci_disabled = 1;
    acpi_noirq = 1;
}

#[cfg(feature = "CONFIG_ACPI")]
#[inline]
pub fn acpi_has_cpu_in_madt() -> bool {
    true
}

#[cfg(feature = "CONFIG_ACPI")]
pub const MAX_CORE_PIC: usize = 2048;

#[cfg(feature = "CONFIG_ACPI")]
extern "C" {
    pub static mut acpi_wakeup_device_list: list_head;
    pub static mut acpi_core_pic: [acpi_madt_core_pic; MAX_CORE_PIC];

    pub fn acpi_add_early_pio();
    pub fn acpi_remove_early_pio();
    pub fn parse_acpi_topology() -> ::core::ffi::c_int;
}

pub const ACPI_TABLE_UPGRADE_MAX_PHYS: usize = ARCH_LOW_ADDRESS_LIMIT;

extern "C" {
    pub fn loongarch_acpi_suspend() -> ::core::ffi::c_int;
    pub static mut acpi_suspend_lowlevel:
        Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
}

#[inline]
pub unsafe fn acpi_get_wakeup_address() -> usize {
    #[cfg(feature = "CONFIG_SUSPEND")]
    {
        return loongarch_wakeup_start as usize;
    }
    0usize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
