/* SPDX-License-Identifier: MIT */
/*
 * PCI Backend/Frontend Common Data Structures & Macros
 *
 *   Author: Ryan Wilson <hap9@epoch.ncsc.mil>
 */

/* Be sure to bump this number if you change this file */
pub const XEN_PCI_MAGIC: &str = "7";

/* xen_pci_sharedinfo flags */
pub const _XEN_PCIF_active: u32 = 0;
pub const XEN_PCIF_active: u32 = 1 << _XEN_PCIF_active;
pub const XEN_PCIB_AERHANDLER: u32 = 1;
pub const XEN_PCIB_AERHANDLER: u32 = 1 << XEN_PCIB_AERHANDLER;
pub const _XEN_PCIB_active: u32 = 2;
pub const XEN_PCIB_active: u32 = 1 << _XEN_PCIB_active;

/* xen_pci_op commands */
pub const XEN_PCI_OP_conf_read: u32 = 0;
pub const XEN_PCI_OP_conf_write: u32 = 1;
pub const XEN_PCI_OP_enable_msi: u32 = 2;
pub const XEN_PCI_OP_disable_msi: u32 = 3;
pub const XEN_PCI_OP_enable_msix: u32 = 4;
pub const XEN_PCI_OP_disable_msix: u32 = 5;
pub const XEN_PCI_OP_aer_detected: u32 = 6;
pub const XEN_PCI_OP_aer_resume: u32 = 7;
pub const XEN_PCI_OP_aer_mmio: u32 = 8;
pub const XEN_PCI_OP_aer_slotreset: u32 = 9;

/* xen_pci_op error numbers */
pub const XEN_PCI_ERR_success: i32 = 0;
pub const XEN_PCI_ERR_dev_not_found: i32 = -1;
pub const XEN_PCI_ERR_invalid_offset: i32 = -2;
pub const XEN_PCI_ERR_access_denied: i32 = -3;
pub const XEN_PCI_ERR_not_implemented: i32 = -4;
/* XEN_PCI_ERR_op_failed - backend failed to complete the operation */
pub const XEN_PCI_ERR_op_failed: i32 = -5;

/*
 * it should be PAGE_SIZE-sizeof(struct xen_pci_op))/sizeof(struct msix_entry))
 * Should not exceed 128
 */
pub const SH_INFO_MAX_VEC: usize = 128;

#[repr(C)]
pub struct xen_msix_entry {
    pub vector: u16,
    pub entry: u16,
}

#[repr(C)]
pub struct xen_pci_op {
    /* IN: what action to perform: XEN_PCI_OP_* */
    pub cmd: u32,

    /* OUT: will contain an error number (if any) from errno.h */
    pub err: i32,

    /* IN: which device to touch */
    pub domain: u32, /* PCI Domain/Segment */
    pub bus: u32,
    pub devfn: u32,

    /* IN: which configuration registers to touch */
    pub offset: i32,
    pub size: i32,

    /* IN/OUT: Contains the result after a READ or the value to WRITE */
    pub value: u32,
    /* IN: Contains extra infor for this operation */
    pub info: u32,
    /*IN:  param for msi-x */
    pub msix_entries: [xen_msix_entry; SH_INFO_MAX_VEC],
}

/*used for pcie aer handling*/
#[repr(C)]
pub struct xen_pcie_aer_op {
    /* IN: what action to perform: XEN_PCI_OP_* */
    pub cmd: u32,
    /*IN/OUT: return aer_op result or carry error_detected state as input*/
    pub err: i32,

    /* IN: which device to touch */
    pub domain: u32, /* PCI Domain/Segment*/
    pub bus: u32,
    pub devfn: u32,
}

#[repr(C)]
pub struct xen_pci_sharedinfo {
    /* flags - XEN_PCIF_* */
    pub flags: u32,
    pub op: xen_pci_op,
    pub aer_op: xen_pcie_aer_op,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
