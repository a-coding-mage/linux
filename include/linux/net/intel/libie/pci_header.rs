/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2025 Intel Corporation */

// Dependency supplied by the Linux PCI implementation: <linux/pci.h>

/// struct libie_pci_mmio_region - structure for MMIO region info
/// @list: used to add a MMIO region to the list of MMIO regions in
///        libie_mmio_info
/// @addr: virtual address of MMIO region start
/// @offset: start offset of the MMIO region
/// @size: size of the MMIO region
/// @bar_idx: BAR index to which the MMIO region belongs to
#[repr(C)]
pub struct libie_pci_mmio_region {
    pub list: list_head,
    pub addr: *mut core::ffi::c_void,
    pub offset: resource_size_t,
    pub size: resource_size_t,
    pub bar_idx: u16,
}

/// struct libie_mmio_info - contains list of MMIO regions
/// @pdev: PCI device pointer
/// @mmio_list: list of MMIO regions
#[repr(C)]
pub struct libie_mmio_info {
    pub pdev: *mut pci_dev,
    pub mmio_list: list_head,
}

// C variadic macros. COUNT_ARGS and the optional-argument forwarding are
// preserved by the underlying declarations below.
// libie_pci_map_mmio_region(mmio_info, offset, size, ...) expands to:
// __libie_pci_map_mmio_region(mmio_info, offset, size,
//                             COUNT_ARGS(__VA_ARGS__), ##__VA_ARGS__)
// libie_pci_get_mmio_addr(mmio_info, offset, ...) expands to:
// __libie_pci_get_mmio_addr(mmio_info, offset,
//                           COUNT_ARGS(__VA_ARGS__), ##__VA_ARGS__)

unsafe extern "C" {
    pub fn __libie_pci_map_mmio_region(
        mmio_info: *mut libie_mmio_info,
        offset: resource_size_t,
        size: resource_size_t,
        num_args: core::ffi::c_int,
        ...,
    ) -> bool;

    pub fn __libie_pci_get_mmio_addr(
        mmio_info: *mut libie_mmio_info,
        offset: resource_size_t,
        num_args: core::ffi::c_int,
        ...,
    ) -> *mut core::ffi::c_void;

    pub fn libie_pci_unmap_all_mmio_regions(mmio_info: *mut libie_mmio_info);

    pub fn libie_pci_unmap_fltr_regs(
        mmio_info: *mut libie_mmio_info,
        fltr: Option<unsafe extern "C" fn(
            mmio_info: *mut libie_mmio_info,
            reg: *mut libie_pci_mmio_region,
        ) -> bool>,
    );

    pub fn libie_pci_init_dev(pdev: *mut pci_dev) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
