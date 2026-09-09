/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * Copyright 2006-2007, Michael Ellerman, IBM Corporation.
 */

/* CONFIG_PCI_MSI */
#[cfg(feature = "CONFIG_PCI_MSI")]
extern "C" {
    pub fn mpic_msi_reserve_hwirq(mpic: *mut mpic, hwirq: irq_hw_number_t);
    pub fn mpic_msi_init_allocator(mpic: *mut mpic) -> ::core::ffi::c_int;
    pub fn mpic_u3msi_init(mpic: *mut mpic) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_PCI_MSI"))]
#[inline]
pub unsafe fn mpic_msi_reserve_hwirq(_mpic: *mut mpic, _hwirq: irq_hw_number_t) {
    return;
}

#[cfg(not(feature = "CONFIG_PCI_MSI"))]
#[inline]
pub unsafe fn mpic_u3msi_init(_mpic: *mut mpic) -> ::core::ffi::c_int {
    return -1;
}

/* CONFIG_PCI_MSI && CONFIG_PPC_PASEMI */
#[cfg(all(feature = "CONFIG_PCI_MSI", feature = "CONFIG_PPC_PASEMI"))]
extern "C" {
    pub fn mpic_pasemi_msi_init(mpic: *mut mpic) -> ::core::ffi::c_int;
}

#[cfg(not(all(feature = "CONFIG_PCI_MSI", feature = "CONFIG_PPC_PASEMI")))]
#[inline]
pub unsafe fn mpic_pasemi_msi_init(_mpic: *mut mpic) -> ::core::ffi::c_int {
    return -1;
}

extern "C" {
    pub fn mpic_set_irq_type(d: *mut irq_data, flow_type: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn mpic_set_vector(virq: ::core::ffi::c_uint, vector: ::core::ffi::c_uint);
    pub fn mpic_set_affinity(
        d: *mut irq_data,
        cpumask: *const cpumask,
        force: bool,
    ) -> ::core::ffi::c_int;
    pub fn mpic_reset_core(cpu: ::core::ffi::c_int);
}

/* CONFIG_FSL_SOC */
#[cfg(feature = "CONFIG_FSL_SOC")]
extern "C" {
    pub fn mpic_map_error_int(
        mpic: *mut mpic,
        virq: ::core::ffi::c_uint,
        hw: irq_hw_number_t,
    ) -> ::core::ffi::c_int;
    pub fn mpic_err_int_init(mpic: *mut mpic, irqnum: irq_hw_number_t);
    pub fn mpic_setup_error_int(mpic: *mut mpic, intvec: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_FSL_SOC"))]
#[inline]
pub unsafe fn mpic_map_error_int(
    _mpic: *mut mpic,
    _virq: ::core::ffi::c_uint,
    _hw: irq_hw_number_t,
) -> ::core::ffi::c_int {
    return 0;
}

#[cfg(not(feature = "CONFIG_FSL_SOC"))]
#[inline]
pub unsafe fn mpic_err_int_init(_mpic: *mut mpic, _irqnum: irq_hw_number_t) {
    return;
}

#[cfg(not(feature = "CONFIG_FSL_SOC"))]
#[inline]
pub unsafe fn mpic_setup_error_int(
    _mpic: *mut mpic,
    _intvec: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return -1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
