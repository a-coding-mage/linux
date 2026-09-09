/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2006 Intel Corp.
 *     Tom Long Nguyen (tom.l.nguyen@intel.com)
 *     Zhang Yanmin (yanmin.zhang@intel.com)
 */

// Dependency intent from the original header: linux/errno.h and linux/types.h.

pub const AER_NONFATAL: i32 = 0;
pub const AER_FATAL: i32 = 1;
pub const AER_CORRECTABLE: i32 = 2;
pub const DPC_FATAL: i32 = 3;

/*
 * AER and DPC capabilities TLP Logging register sizes (PCIe r6.2, sec 7.8.4
 * & 7.9.14).
 */
pub const PCIE_STD_NUM_TLP_HEADERLOG: usize = 4;
pub const PCIE_STD_MAX_TLP_PREFIXLOG: usize = 4;
pub const PCIE_STD_MAX_TLP_HEADERLOG: usize =
    PCIE_STD_NUM_TLP_HEADERLOG + 10;

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub union pcie_tlp_log_data {
    pub dw: [u32; PCIE_STD_MAX_TLP_HEADERLOG],
    pub fields: pcie_tlp_log_fields,
}

#[repr(C)]
pub struct pcie_tlp_log_fields {
    pub _do_not_use: [u32; PCIE_STD_NUM_TLP_HEADERLOG],
    pub prefix: [u32; PCIE_STD_MAX_TLP_PREFIXLOG],
}

#[repr(C)]
pub struct pcie_tlp_log {
    pub data: pcie_tlp_log_data,
    pub header_len: u8, /* Length of the Logged TLP Header in DWORDs */
    pub flit: bool,    /* TLP was logged when in Flit mode */
}

#[repr(C)]
pub struct aer_capability_regs {
    pub header: u32,
    pub uncor_status: u32,
    pub uncor_mask: u32,
    pub uncor_severity: u32,
    pub cor_status: u32,
    pub cor_mask: u32,
    pub cap_control: u32,
    pub header_log: pcie_tlp_log,
    pub root_command: u32,
    pub root_status: u32,
    pub cor_err_source: u16,
    pub uncor_err_source: u16,
}

#[cfg(CONFIG_PCIEAER)]
extern "C" {
    pub fn pci_aer_clear_nonfatal_status(dev: *mut pci_dev) -> i32;
    pub fn pcie_aer_is_native(dev: *mut pci_dev) -> i32;
    pub fn pci_aer_unmask_internal_errors(dev: *mut pci_dev);
}

#[cfg(not(CONFIG_PCIEAER))]
#[inline]
pub unsafe fn pci_aer_clear_nonfatal_status(_dev: *mut pci_dev) -> i32 {
    -crate::EINVAL
}

#[cfg(not(CONFIG_PCIEAER))]
#[inline]
pub unsafe fn pcie_aer_is_native(_dev: *mut pci_dev) -> i32 {
    0
}

#[cfg(not(CONFIG_PCIEAER))]
#[inline]
pub unsafe fn pci_aer_unmask_internal_errors(_dev: *mut pci_dev) {}

extern "C" {
    pub fn pci_print_aer(
        dev: *mut pci_dev,
        aer_severity: i32,
        aer: *mut aer_capability_regs,
    );
    pub fn cper_severity_to_aer(cper_severity: i32) -> i32;
    pub fn aer_recover_queue(
        domain: i32,
        bus: u32,
        devfn: u32,
        severity: i32,
        aer_regs: *mut aer_capability_regs,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
