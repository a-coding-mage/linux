/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 2013-2014, Linaro Ltd.
 *	Author: Al Stone <al.stone@linaro.org>
 *	Author: Graeme Gregory <graeme.gregory@linaro.org>
 *	Author: Hanjun Guo <hanjun.guo@linaro.org>
 *
 *  Copyright (C) 2021-2023, Ventana Micro Systems Inc.
 *	Author: Sunil V L <sunilvl@ventanamicro.com>
 */

/* Basic configuration for ACPI. These cfg conditions preserve CONFIG_ACPI. */
#[cfg(CONFIG_ACPI)]
pub type phys_cpuid_t = u64;

#[cfg(CONFIG_ACPI)]
pub const PHYS_CPUID_INVALID: phys_cpuid_t = INVALID_HARTID;

#[cfg(CONFIG_ACPI)]
extern "C" {
    pub fn acpi_os_ioremap(phys: acpi_physical_address, size: acpi_size) -> *mut core::ffi::c_void;
    pub static mut acpi_disabled: core::ffi::c_int;
    pub static mut acpi_noirq: core::ffi::c_int;
    pub static mut acpi_pci_disabled: core::ffi::c_int;
}

#[cfg(CONFIG_ACPI)]
pub const acpi_strict: i32 = 1; /* No out-of-spec workarounds on RISC-V */

#[cfg(CONFIG_ACPI)]
#[inline]
pub unsafe fn disable_acpi() {
    acpi_disabled = 1;
    acpi_pci_disabled = 1;
    acpi_noirq = 1;
}

#[cfg(CONFIG_ACPI)]
#[inline]
pub unsafe fn enable_acpi() {
    acpi_disabled = 0;
    acpi_pci_disabled = 0;
    acpi_noirq = 0;
}

/*
 * The ACPI processor driver for ACPI core code needs this macro
 * to find out whether this cpu was already mapped (mapping from CPU hardware
 * ID to CPU logical ID) or not.
 */
#[cfg(CONFIG_ACPI)]
#[inline]
pub unsafe fn cpu_physical_id(cpu: core::ffi::c_int) -> _ {
    cpuid_to_hartid_map(cpu)
}

/*
 * Since MADT must provide at least one RINTC structure, the
 * CPU will be always available in MADT on RISC-V.
 */
#[cfg(CONFIG_ACPI)]
#[inline]
pub const fn acpi_has_cpu_in_madt() -> bool {
    true
}

#[cfg(CONFIG_ACPI)]
#[inline]
pub const fn arch_fix_phys_package_id(_num: core::ffi::c_int, _slot: u32) {}

#[cfg(CONFIG_ACPI)]
extern "C" {
    pub fn acpi_init_rintc_map();
    pub fn acpi_cpu_get_madt_rintc(cpu: core::ffi::c_int) -> *mut acpi_madt_rintc;
    pub fn acpi_get_riscv_isa(
        table: *mut acpi_table_header,
        cpu: u32,
        isa: *mut *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn acpi_get_cbo_block_size(
        table: *mut acpi_table_header,
        cbom_size: *mut u32,
        cboz_size: *mut u32,
        cbop_size: *mut u32,
    );
    pub fn acpi_get_riscv_gsi_handle(gsi: u32) -> acpi_handle;
}

/*
 * RISC-V Functional Fixed Hardware Specification Version v1.0.1,
 * Chapter 3.1.2, Table 4: Arch. Context Lost Flags
 */
#[cfg(CONFIG_ACPI)]
pub const RISCV_LPI_HART_TIMER_CTXT_LOST: u32 = 1u32 << 0;

#[cfg(CONFIG_ACPI)]
#[inline]
pub fn arch_get_idle_state_flags(arch_flags: u32) -> u32 {
    if arch_flags & RISCV_LPI_HART_TIMER_CTXT_LOST != 0 {
        return CPUIDLE_FLAG_TIMER_STOP;
    }
    0
}

/* C macro alias: arch_get_idle_state_flags */

#[cfg(not(CONFIG_ACPI))]
#[inline]
pub const fn acpi_init_rintc_map() {}

#[cfg(not(CONFIG_ACPI))]
#[inline]
pub const fn acpi_cpu_get_madt_rintc(_cpu: core::ffi::c_int) -> *mut acpi_madt_rintc {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_ACPI))]
#[inline]
pub const fn acpi_get_riscv_isa(
    _table: *mut acpi_table_header,
    _cpu: u32,
    _isa: *mut *const core::ffi::c_char,
) -> core::ffi::c_int {
    -EINVAL
}

#[cfg(not(CONFIG_ACPI))]
#[inline]
pub const fn acpi_get_cbo_block_size(
    _table: *mut acpi_table_header,
    _cbom_size: *mut u32,
    _cboz_size: *mut u32,
    _cbop_size: *mut u32,
) {}

#[cfg(CONFIG_ACPI_NUMA)]
extern "C" {
    pub fn acpi_map_cpus_to_nodes();
}

#[cfg(not(CONFIG_ACPI_NUMA))]
#[inline]
pub const fn acpi_map_cpus_to_nodes() {}

pub const ACPI_TABLE_UPGRADE_MAX_PHYS: _ = MEMBLOCK_ALLOC_ACCESSIBLE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
