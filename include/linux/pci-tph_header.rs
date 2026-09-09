/* SPDX-License-Identifier: GPL-2.0 */
/*
 * TPH (TLP Processing Hints)
 *
 * Copyright (C) 2024 Advanced Micro Devices, Inc.
 *     Eric Van Tassell <Eric.VanTassell@amd.com>
 *     Wei Huang <wei.huang2@amd.com>
 */

/*
 * According to the ECN for PCI Firmware Spec, Steering Tag can be different
 * depending on the memory type: Volatile Memory or Persistent Memory. When a
 * caller query about a target's Steering Tag, it must provide the target's
 * tph_mem_type. ECN link: https://members.pcisig.com/wg/PCI-SIG/document/15470.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tph_mem_type {
    TPH_MEM_TYPE_VM,
    TPH_MEM_TYPE_PM,
}

#[cfg(CONFIG_PCIE_TPH)]
extern "C" {
    pub fn pcie_tph_set_st_entry(
        pdev: *mut pci_dev,
        index: ::core::ffi::c_uint,
        tag: u16,
    ) -> ::core::ffi::c_int;
    pub fn pcie_tph_get_cpu_st(
        dev: *mut pci_dev,
        mem_type: tph_mem_type,
        cpu: ::core::ffi::c_uint,
        tag: *mut u16,
    ) -> ::core::ffi::c_int;
    pub fn pcie_disable_tph(pdev: *mut pci_dev);
    pub fn pcie_enable_tph(pdev: *mut pci_dev, mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn pcie_tph_get_st_table_size(pdev: *mut pci_dev) -> u16;
    pub fn pcie_tph_get_st_table_loc(pdev: *mut pci_dev) -> u32;
}

#[cfg(not(CONFIG_PCIE_TPH))]
#[inline]
pub unsafe fn pcie_tph_set_st_entry(
    _pdev: *mut pci_dev,
    _index: ::core::ffi::c_uint,
    _tag: u16,
) -> ::core::ffi::c_int {
    -EINVAL
}

#[cfg(not(CONFIG_PCIE_TPH))]
#[inline]
pub unsafe fn pcie_tph_get_cpu_st(
    _dev: *mut pci_dev,
    _mem_type: tph_mem_type,
    _cpu: ::core::ffi::c_uint,
    _tag: *mut u16,
) -> ::core::ffi::c_int {
    -EINVAL
}

#[cfg(not(CONFIG_PCIE_TPH))]
#[inline]
pub unsafe fn pcie_disable_tph(_pdev: *mut pci_dev) {}

#[cfg(not(CONFIG_PCIE_TPH))]
#[inline]
pub unsafe fn pcie_enable_tph(
    _pdev: *mut pci_dev,
    _mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
