/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 * Copyright (C) 2017 SiFive
 */

// Translated from the RISC-V IRQ header. C includes and header guards omitted.

pub const INVALID_CONTEXT: u32 = u32::MAX;

// Opaque types supplied by the included kernel headers.
pub enum fwnode_handle {}
pub enum resource {}
pub enum cpumask_t {}

#[cfg(CONFIG_SMP)]
pub unsafe extern "C" {
    pub fn arch_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: core::ffi::c_int);
}
// C macro alias: arch_trigger_cpumask_backtrace -> arch_trigger_cpumask_backtrace.

pub unsafe extern "C" {
    pub fn riscv_set_intc_hwnode_fn(
        func: Option<unsafe extern "C" fn() -> *mut fwnode_handle>,
    );

    pub fn riscv_get_intc_hwnode() -> *mut fwnode_handle;
    pub fn riscv_get_hart_index(
        fwnode: *mut fwnode_handle,
        logical_index: u32,
        hart_index: *mut u32,
    ) -> core::ffi::c_int;
}

#[cfg(CONFIG_ACPI)]
#[repr(C)]
pub enum riscv_irqchip_type {
    ACPI_RISCV_IRQCHIP_INTC = 0x00,
    ACPI_RISCV_IRQCHIP_IMSIC = 0x01,
    ACPI_RISCV_IRQCHIP_PLIC = 0x02,
    ACPI_RISCV_IRQCHIP_APLIC = 0x03,
    ACPI_RISCV_IRQCHIP_SMSI = 0x04,
}

#[cfg(CONFIG_ACPI)]
pub unsafe extern "C" {
    pub fn riscv_acpi_get_gsi_info(
        fwnode: *mut fwnode_handle,
        gsi_base: *mut u32,
        id: *mut u32,
        nr_irqs: *mut u32,
        nr_idcs: *mut u32,
    ) -> core::ffi::c_int;
    pub fn riscv_acpi_get_gsi_domain_id(gsi: u32) -> *mut fwnode_handle;
    pub fn acpi_rintc_index_to_hartid(index: u32) -> core::ffi::c_ulong;
    pub fn acpi_rintc_ext_parent_to_hartid(
        plic_id: core::ffi::c_uint,
        ctxt_idx: core::ffi::c_uint,
    ) -> core::ffi::c_ulong;
    pub fn acpi_rintc_get_plic_nr_contexts(plic_id: core::ffi::c_uint) -> core::ffi::c_uint;
    pub fn acpi_rintc_get_plic_context(
        plic_id: core::ffi::c_uint,
        ctxt_idx: core::ffi::c_uint,
    ) -> core::ffi::c_uint;
    pub fn acpi_rintc_get_imsic_mmio_info(
        index: u32,
        res: *mut resource,
    ) -> core::ffi::c_int;
    pub fn riscv_acpi_update_gsi_range(gsi_base: u32, nr_irqs: u32) -> core::ffi::c_int;
}

#[cfg(not(CONFIG_ACPI))]
pub unsafe fn riscv_acpi_get_gsi_info(
    _fwnode: *mut fwnode_handle,
    _gsi_base: *mut u32,
    _id: *mut u32,
    _nr_irqs: *mut u32,
    _nr_idcs: *mut u32,
) -> core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_ACPI))]
pub unsafe fn acpi_rintc_index_to_hartid(_index: u32) -> core::ffi::c_ulong {
    INVALID_HARTID
}

#[cfg(not(CONFIG_ACPI))]
pub unsafe fn acpi_rintc_ext_parent_to_hartid(
    _plic_id: core::ffi::c_uint,
    _ctxt_idx: core::ffi::c_uint,
) -> core::ffi::c_ulong {
    INVALID_HARTID
}

#[cfg(not(CONFIG_ACPI))]
pub unsafe fn acpi_rintc_get_plic_nr_contexts(_plic_id: core::ffi::c_uint) -> core::ffi::c_uint {
    INVALID_CONTEXT
}

#[cfg(not(CONFIG_ACPI))]
pub unsafe fn acpi_rintc_get_plic_context(
    _plic_id: core::ffi::c_uint,
    _ctxt_idx: core::ffi::c_uint,
) -> core::ffi::c_uint {
    INVALID_CONTEXT
}

#[cfg(not(CONFIG_ACPI))]
pub unsafe fn acpi_rintc_get_imsic_mmio_info(
    _index: u32,
    _res: *mut resource,
) -> core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_ACPI))]
pub unsafe fn riscv_acpi_update_gsi_range(_gsi_base: u32, _nr_irqs: u32) -> core::ffi::c_int {
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
