/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2020 Intel Corporation. All rights reserved. */

// Dependency intent: names supplied by linux/pci.h and cxl.h remain external.

pub const CXL_MEMORY_PROGIF: u32 = 0x10;

/*
 * NOTE: Currently all the functions which are enabled for CXL require their
 * vectors to be in the first 16.  Use this as the default max.
 */
pub const CXL_PCI_DEFAULT_MAX_VECTORS: u32 = 16;

/*
 * Table Access DOE, CDAT Read Entry Response
 *
 * Spec refs:
 *
 * CXL 3.1 8.1.11, Table 8-14: Read Entry Response
 * CDAT Specification 1.03: 2 CDAT Data Structures
 */
#[repr(C, packed)]
pub struct cdat_header {
    pub length: __le32,
    pub revision: u8,
    pub checksum: u8,
    pub reserved: [u8; 6],
    pub sequence: __le32,
}

#[repr(C, packed)]
pub struct cdat_entry_header {
    pub type_: u8,
    pub reserved: u8,
    pub length: __le16,
}

/*
 * The DOE CDAT read response contains a CDAT read entry (either the
 * CDAT header or a structure).
 */
#[repr(C, packed)]
pub union cdat_data {
    pub header: cdat_header,
    pub entry: cdat_entry_header,
}

/* There is an additional CDAT response header of 4 bytes. */
#[repr(C, packed)]
pub struct cdat_doe_rsp {
    pub doe_header: __le32,
    pub data: [u8; 0],
}

/*
 * CXL v3.0 6.2.3 Table 6-4
 * The table indicates that if PCIe Flit Mode is set, then CXL is in 256B flits
 * mode, otherwise it's 68B flits mode.
 */
#[inline]
pub unsafe fn cxl_pci_flit_256(pdev: *mut pci_dev) -> bool {
    let mut lnksta2: u16 = 0;
    pcie_capability_read_word(pdev, PCI_EXP_LNKSTA2, &mut lnksta2);
    (lnksta2 & PCI_EXP_LNKSTA2_FLIT) != 0
}

/*
 * Assume that the caller has already validated that @pdev has CXL
 * capabilities, any RCiEP with CXL capabilities is treated as a
 * Restricted CXL Device (RCD) and finds upstream port and endpoint
 * registers in a Root Complex Register Block (RCRB).
 */
#[inline]
pub unsafe fn is_cxl_restricted(pdev: *mut pci_dev) -> bool {
    pci_pcie_type(pdev) == PCI_EXP_TYPE_RC_END
}

pub struct cxl_dev_state;

extern "C" {
    pub fn read_cdat_data(port: *mut cxl_port);
}

// CONFIG_CXL_RAS is a build-time condition from the C environment.
#[cfg(feature = "CONFIG_CXL_RAS")]
extern "C" {
    pub fn cxl_cor_error_detected(pdev: *mut pci_dev);
    pub fn cxl_error_detected(pdev: *mut pci_dev, state: pci_channel_state_t) -> pci_ers_result_t;
    pub fn devm_cxl_dport_rch_ras_setup(dport: *mut cxl_dport);
    pub fn devm_cxl_port_ras_setup(port: *mut cxl_port);
}

#[cfg(not(feature = "CONFIG_CXL_RAS"))]
#[inline]
pub unsafe fn cxl_cor_error_detected(_pdev: *mut pci_dev) {}

#[cfg(not(feature = "CONFIG_CXL_RAS"))]
#[inline]
pub unsafe fn cxl_error_detected(
    _pdev: *mut pci_dev,
    _state: pci_channel_state_t,
) -> pci_ers_result_t {
    PCI_ERS_RESULT_NONE
}

#[cfg(not(feature = "CONFIG_CXL_RAS"))]
#[inline]
pub unsafe fn devm_cxl_dport_rch_ras_setup(_dport: *mut cxl_dport) {}

#[cfg(not(feature = "CONFIG_CXL_RAS"))]
#[inline]
pub unsafe fn devm_cxl_port_ras_setup(_port: *mut cxl_port) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
