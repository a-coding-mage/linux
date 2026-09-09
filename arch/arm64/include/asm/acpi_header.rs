/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 2013-2014, Linaro Ltd.
 *	Author: Al Stone <al.stone@linaro.org>
 *	Author: Graeme Gregory <graeme.gregory@linaro.org>
 *	Author: Hanjun Guo <hanjun.guo@linaro.org>
 */

// C header dependencies are supplied by other translated files.

/* Macros for consistency checks of the GICC subtable of MADT. */
pub const ACPI_MADT_GICC_MIN_LENGTH: usize =
    core::mem::offset_of!(acpi_madt_generic_interrupt, efficiency_class);

#[inline]
pub unsafe fn BAD_MADT_GICC_ENTRY(
    entry: *const acpi_madt_generic_interrupt,
    end: usize,
) -> bool {
    entry.is_null()
        || (*entry).header.length < ACPI_MADT_GICC_MIN_LENGTH as _
        || (entry as usize).wrapping_add((*entry).header.length as usize) > end
}

pub const ACPI_MADT_GICC_SPE: usize =
    core::mem::offset_of!(acpi_madt_generic_interrupt, spe_interrupt) + core::mem::size_of::<u16>();
pub const ACPI_MADT_GICC_TRBE: usize =
    core::mem::offset_of!(acpi_madt_generic_interrupt, trbe_interrupt) + core::mem::size_of::<u16>();

/* Arm® Functional Fixed Hardware Specification Version 1.2.
 * Table 2: Arm Architecture context loss flags
 */
pub const CPUIDLE_CORE_CTXT: u32 = 1u32 << 0; /* Core context Lost */

#[inline]
pub fn arch_get_idle_state_flags(arch_flags: u32) -> u32 {
    if arch_flags & CPUIDLE_CORE_CTXT != 0 {
        return CPUIDLE_FLAG_TIMER_STOP;
    }
    0
}

pub const CPUIDLE_TRACE_CTXT: u32 = 1u32 << 1; /* Trace context loss */
pub const CPUIDLE_GICR_CTXT: u32 = 1u32 << 2; /* GICR */
pub const CPUIDLE_GICD_CTXT: u32 = 1u32 << 3; /* GICD */

/* Basic configuration for ACPI. */
#[cfg(CONFIG_ACPI)]
extern "C" {
    pub fn __acpi_get_mem_attribute(addr: phys_addr_t) -> pgprot_t;
    pub fn acpi_os_ioremap(phys: acpi_physical_address, size: acpi_size) -> *mut core::ffi::c_void;
    pub static mut acpi_disabled: i32;
    pub static mut acpi_noirq: i32;
    pub static mut acpi_pci_disabled: i32;
    pub fn acpi_cpu_get_madt_gicc(cpu: i32) -> *mut acpi_madt_generic_interrupt;
    pub fn get_cpu_for_acpi_id(uid: u32) -> i32;
    pub fn acpi_init_cpus();
    pub fn apei_claim_sea(regs: *mut pt_regs) -> i32;
}

#[cfg(CONFIG_ACPI)]
pub type phys_cpuid_t = u64;
#[cfg(CONFIG_ACPI)]
pub const PHYS_CPUID_INVALID: phys_cpuid_t = INVALID_HWID;
#[cfg(CONFIG_ACPI)]
pub const acpi_strict: i32 = 1; /* No out-of-spec workarounds on ARM64 */

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

#[inline]
pub fn cpu_physical_id(cpu: i32) -> i32 { cpu_logical_map(cpu) }

#[inline]
pub fn acpi_has_cpu_in_madt() -> bool { true }

#[cfg(not(CONFIG_ACPI))]
#[inline]
pub fn acpi_init_cpus() {}
#[cfg(not(CONFIG_ACPI))]
#[inline]
pub fn apei_claim_sea(_regs: *mut pt_regs) -> i32 { -ENOENT }

#[cfg(CONFIG_ARM64_ACPI_PARKING_PROTOCOL)]
extern "C" {
    pub fn acpi_parking_protocol_valid(cpu: i32) -> bool;
    pub fn acpi_set_mailbox_entry(cpu: i32, processor: *mut acpi_madt_generic_interrupt);
}
#[cfg(not(CONFIG_ARM64_ACPI_PARKING_PROTOCOL))]
#[inline]
pub fn acpi_parking_protocol_valid(_cpu: i32) -> bool { false }
#[cfg(not(CONFIG_ARM64_ACPI_PARKING_PROTOCOL))]
#[inline]
pub fn acpi_set_mailbox_entry(_cpu: i32, _processor: *mut acpi_madt_generic_interrupt) {}

#[inline(always)]
pub fn acpi_get_enable_method(cpu: i32) -> *const core::ffi::c_char {
    if acpi_psci_present() { return b"psci\0".as_ptr() as *const _; }
    if acpi_parking_protocol_valid(cpu) { return b"parking-protocol\0".as_ptr() as *const _; }
    core::ptr::null()
}

#[cfg(CONFIG_ACPI_APEI)]
pub const acpi_disable_cmcff: i32 = 1;
#[cfg(CONFIG_ACPI_APEI)]
#[inline]
pub fn arch_apei_get_mem_attribute(addr: phys_addr_t) -> pgprot_t {
    unsafe { __acpi_get_mem_attribute(addr) }
}

#[cfg(CONFIG_ACPI_NUMA)]
extern "C" {
    pub fn arm64_acpi_numa_init() -> i32;
    pub fn acpi_numa_get_nid(cpu: u32) -> i32;
    pub fn acpi_map_cpus_to_nodes();
}
#[cfg(not(CONFIG_ACPI_NUMA))]
#[inline]
pub fn arm64_acpi_numa_init() -> i32 { -ENOSYS }
#[cfg(not(CONFIG_ACPI_NUMA))]
#[inline]
pub fn acpi_numa_get_nid(_cpu: u32) -> i32 { NUMA_NO_NODE }
#[cfg(not(CONFIG_ACPI_NUMA))]
#[inline]
pub fn acpi_map_cpus_to_nodes() {}

pub const ACPI_TABLE_UPGRADE_MAX_PHYS: usize = MEMBLOCK_ALLOC_ACCESSIBLE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
