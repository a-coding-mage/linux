// SPDX-License-Identifier: GPL-2.0
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

use core::ffi::{c_char, c_int, c_ulong};

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static acpi_disabled: bool;
    static acpi_gbl_reduced_hardware: bool;
    static acpi_gbl_FADT: AcpiGenericAddressTable;
    static cpu_has_scalefreq: bool;
    static mut loongson_sysconf: LoongsonSysconf;

    fn acpi_hw_enable_all_wakeup_gpes();
    fn acpi_write_bit_register(register: u32, value: u32) -> AcpiStatus;
    fn platform_device_register(device: *mut PlatformDevice) -> c_int;
    fn acpi_enter_sleep_state(state: u32);
    fn acpi_sleep_state_supported(state: u32) -> bool;
    fn acpi_evaluate_integer(
        pathname: *const c_char,
        arguments: *const core::ffi::c_void,
        return_value: *const core::ffi::c_void,
        value: *mut u64,
    ) -> AcpiStatus;
    fn phys_to_virt(address: usize) -> usize;
    fn pr_info(format: *const c_char, ...);
}

type AcpiStatus = u32;

#[repr(C)]
struct AcpiGenericAddressTable {
    flags: u32,
}

#[repr(C)]
struct PlatformDevice {
    name: *const c_char,
    id: c_int,
}

#[repr(C)]
struct LoongsonSysconf {
    suspend_addr: c_ulong,
}

const ACPI_BITREG_PCIEXP_WAKE_STATUS: u32 = 0;
const ACPI_BITREG_PCIEXP_WAKE_DISABLE: u32 = 1;
const ACPI_BITREG_SCI_ENABLE: u32 = 2;
const ACPI_FADT_PCI_EXPRESS_WAKE: u32 = 1 << 14;
const ACPI_STATE_S3: u32 = 3;
const ENODEV: c_int = 19;

#[repr(C)]
static mut loongson3_cpufreq_device: PlatformDevice = PlatformDevice {
    name: b"loongson3_cpufreq\0".as_ptr() as *const c_char,
    id: -1,
};

pub unsafe fn enable_gpe_wakeup() {
    if acpi_disabled {
        return;
    }

    if acpi_gbl_reduced_hardware {
        return;
    }

    acpi_hw_enable_all_wakeup_gpes();
}

pub unsafe fn enable_pci_wakeup() {
    if acpi_disabled {
        return;
    }

    if acpi_gbl_reduced_hardware {
        return;
    }

    acpi_write_bit_register(ACPI_BITREG_PCIEXP_WAKE_STATUS, 1);

    if acpi_gbl_FADT.flags & ACPI_FADT_PCI_EXPRESS_WAKE != 0 {
        acpi_write_bit_register(ACPI_BITREG_PCIEXP_WAKE_DISABLE, 0);
    }
}

unsafe fn loongson_cpufreq_init() -> c_int {
    if !cpu_has_scalefreq {
        return -ENODEV;
    }

    platform_device_register(&mut loongson3_cpufreq_device)
}

// arch_initcall(loongson_cpufreq_init);

unsafe fn default_suspend_addr() {
    acpi_enter_sleep_state(ACPI_STATE_S3);
}

unsafe fn loongson3_acpi_suspend_init() -> c_int {
    // #ifdef CONFIG_ACPI
    let mut suspend_addr: u64 = 0;

    if acpi_disabled {
        return 0;
    }

    if !acpi_gbl_reduced_hardware {
        acpi_write_bit_register(ACPI_BITREG_SCI_ENABLE, 1);
    }

    if !acpi_sleep_state_supported(ACPI_STATE_S3) {
        return 0;
    }

    let status = acpi_evaluate_integer(
        b"\\SADR\0".as_ptr() as *const c_char,
        core::ptr::null(),
        core::ptr::null(),
        &mut suspend_addr,
    );
    if status != 0 || suspend_addr == 0 {
        pr_info(b"ACPI S3 supported with hardware register default\n\0".as_ptr() as *const c_char);
        loongson_sysconf.suspend_addr = default_suspend_addr as usize as c_ulong;
    } else {
        pr_info(b"ACPI S3 supported with Loongson ACPI SADR extension\n\0".as_ptr() as *const c_char);
        // PHYSADDR(suspend_addr) is an architecture-provided address conversion.
        loongson_sysconf.suspend_addr = phys_to_virt(suspend_addr as usize) as c_ulong;
    }
    // #endif
    0
}

// device_initcall(loongson3_acpi_suspend_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
