/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016, Semihalf
 *	Author: Tomasz Nowicki <tn@semihalf.com>
 */

// Dependencies supplied by other translated headers are intentionally not
// implemented here.

#[inline]
pub const fn iort_irq_mask(irq: u64) -> u64 {
    irq & 0xffff_ffffu64
}

#[inline]
pub const fn iort_irq_trigger_mask(irq: u64) -> u64 {
    (irq >> 32) & 0xffff_ffffu64
}

/*
 * PMCG model identifiers for use in smmu pmu driver. Please note
 * that this is purely for the use of software and has nothing to
 * do with hardware or with IORT specification.
 */
pub const IORT_SMMU_V3_PMCG_GENERIC: u32 = 0x0000_0000;
pub const IORT_SMMU_V3_PMCG_HISI_HIP08: u32 = 0x0000_0001;
pub const IORT_SMMU_V3_PMCG_HISI_HIP09: u32 = 0x0000_0002;

extern "C" {
    pub fn iort_register_domain_token(
        trans_id: i32,
        base: phys_addr_t,
        fw_node: *mut fwnode_handle,
    ) -> i32;
    pub fn iort_deregister_domain_token(trans_id: i32);
    pub fn iort_find_domain_token(trans_id: i32) -> *mut fwnode_handle;
    pub fn iort_iwb_handle(iwb_id: u32) -> acpi_handle;
    pub fn iort_iwb_handle_fwnode(iwb_id: u32) -> *mut fwnode_handle;
}

// The CONFIG_ACPI_IORT branch is preserved as a Rust configuration condition.
#[cfg(feature = "CONFIG_ACPI_IORT")]
extern "C" {
    pub fn iort_msi_map_id(dev: *mut device, id: u32) -> u32;
    pub fn iort_msi_xlate(dev: *mut device, id: u32, node: *mut *mut fwnode_handle) -> u32;
    pub fn iort_its_translate_pa(node: *mut fwnode_handle, base: *mut phys_addr_t) -> i32;
    pub fn iort_get_device_domain(
        dev: *mut device,
        id: u32,
        bus_token: irq_domain_bus_token,
    ) -> *mut irq_domain;
    pub fn iort_pmsi_get_msi_info(
        dev: *mut device,
        dev_id: *mut u32,
        pa: *mut phys_addr_t,
    ) -> i32;
    pub fn acpi_configure_pmsi_domain(dev: *mut device);
    pub fn iort_get_rmr_sids(iommu_fwnode: *mut fwnode_handle, head: *mut list_head);
    pub fn iort_put_rmr_sids(iommu_fwnode: *mut fwnode_handle, head: *mut list_head);
    pub fn iort_dma_get_ranges(dev: *mut device, limit: *mut u64) -> i32;
    pub fn iort_iommu_configure_id(dev: *mut device, id_in: *const u32) -> i32;
    pub fn iort_iommu_get_resv_regions(dev: *mut device, head: *mut list_head);
    pub fn acpi_iort_dma_get_max_cpu_address() -> phys_addr_t;
}

#[cfg(not(feature = "CONFIG_ACPI_IORT"))]
#[inline]
pub unsafe fn iort_msi_map_id(_dev: *mut device, id: u32) -> u32 { id }

#[cfg(not(feature = "CONFIG_ACPI_IORT"))]
#[inline]
pub unsafe fn iort_msi_xlate(_dev: *mut device, id: u32, _node: *mut *mut fwnode_handle) -> u32 { id }

#[cfg(not(feature = "CONFIG_ACPI_IORT"))]
#[inline]
pub unsafe fn iort_its_translate_pa(_node: *mut fwnode_handle, _base: *mut phys_addr_t) -> i32 { -ENODEV }

#[cfg(not(feature = "CONFIG_ACPI_IORT"))]
#[inline]
pub unsafe fn iort_get_device_domain(
    _dev: *mut device,
    _id: u32,
    _bus_token: irq_domain_bus_token,
) -> *mut irq_domain { core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_ACPI_IORT"))]
#[inline]
pub unsafe fn iort_pmsi_get_msi_info(
    _dev: *mut device,
    _dev_id: *mut u32,
    _pa: *mut phys_addr_t,
) -> i32 { -ENODEV }

#[cfg(not(feature = "CONFIG_ACPI_IORT"))]
#[inline]
pub unsafe fn acpi_configure_pmsi_domain(_dev: *mut device) {}

#[cfg(not(feature = "CONFIG_ACPI_IORT"))]
#[inline]
pub unsafe fn iort_get_rmr_sids(_iommu_fwnode: *mut fwnode_handle, _head: *mut list_head) {}

#[cfg(not(feature = "CONFIG_ACPI_IORT"))]
#[inline]
pub unsafe fn iort_put_rmr_sids(_iommu_fwnode: *mut fwnode_handle, _head: *mut list_head) {}

#[cfg(not(feature = "CONFIG_ACPI_IORT"))]
#[inline]
pub unsafe fn iort_dma_get_ranges(_dev: *mut device, _limit: *mut u64) -> i32 { -ENODEV }

#[cfg(not(feature = "CONFIG_ACPI_IORT"))]
#[inline]
pub unsafe fn iort_iommu_configure_id(_dev: *mut device, _id_in: *const u32) -> i32 { -ENODEV }

#[cfg(not(feature = "CONFIG_ACPI_IORT"))]
#[inline]
pub unsafe fn iort_iommu_get_resv_regions(_dev: *mut device, _head: *mut list_head) {}

#[cfg(not(feature = "CONFIG_ACPI_IORT"))]
#[inline]
pub unsafe fn acpi_iort_dma_get_max_cpu_address() -> phys_addr_t { PHYS_ADDR_MAX }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
