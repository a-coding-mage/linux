/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * VFIO Region definitions for ZPCI devices
 *
 * Copyright IBM Corp. 2020
 *
 * Author(s): Pierre Morel <pmorel@linux.ibm.com>
 *            Matthew Rosato <mjrosato@linux.ibm.com>
 */

// Dependencies supplied by the corresponding Linux VFIO and types bindings.

/**
 * VFIO_DEVICE_INFO_CAP_ZPCI_BASE - Base PCI Function information
 *
 * This capability provides a set of descriptive information about the
 * associated PCI function.
 */
#[repr(C)]
pub struct vfio_device_info_cap_zpci_base {
    pub header: vfio_info_cap_header,
    pub start_dma: u64,      /* Start of available DMA addresses */
    pub end_dma: u64,        /* End of available DMA addresses */
    pub pchid: u16,          /* Physical Channel ID */
    pub vfn: u16,            /* Virtual function number */
    pub fmb_length: u16,     /* Measurement Block Length (in bytes) */
    pub pft: u8,             /* PCI Function Type */
    pub gid: u8,             /* PCI function group ID */
    /* End of version 1 */
    pub fh: u32,             /* PCI function handle */
    /* End of version 2 */
    pub ccdf_err_length: u32, /* PCI CCDF length */
    /* End of version 3 */
}

/**
 * VFIO_DEVICE_INFO_CAP_ZPCI_GROUP - Base PCI Function Group information
 *
 * This capability provides a set of descriptive information about the group of
 * PCI functions that the associated device belongs to.
 */
#[repr(C)]
pub struct vfio_device_info_cap_zpci_group {
    pub header: vfio_info_cap_header,
    pub dasm: u64,       /* DMA Address space mask */
    pub msi_addr: u64,   /* MSI address */
    pub flags: u64,
    pub mui: u16,        /* Measurement Block Update Interval */
    pub noi: u16,        /* Maximum number of MSIs */
    pub maxstbl: u16,    /* Maximum Store Block Length */
    pub version: u8,     /* Supported PCI Version */
    /* End of version 1 */
    pub reserved: u8,
    pub imaxstbl: u16,   /* Maximum Interpreted Store Block Length */
    /* End of version 2 */
}

pub const VFIO_DEVICE_INFO_ZPCI_FLAG_REFRESH: u64 = 1;

/**
 * VFIO_DEVICE_INFO_CAP_ZPCI_UTIL - Utility String
 *
 * This capability provides the utility string for the associated device, which
 * is a device identifier string made up of EBCDID characters.  'size' specifies
 * the length of 'util_str'.
 */
#[repr(C)]
pub struct vfio_device_info_cap_zpci_util {
    pub header: vfio_info_cap_header,
    pub size: u32,
    pub util_str: [u8; 0],
}

/**
 * VFIO_DEVICE_INFO_CAP_ZPCI_PFIP - PCI Function Path
 *
 * This capability provides the PCI function path string, which is an identifier
 * that describes the internal hardware path of the device. 'size' specifies the
 * length of 'pfip'.
 */
#[repr(C)]
pub struct vfio_device_info_cap_zpci_pfip {
    pub header: vfio_info_cap_header,
    pub size: u32,
    pub pfip: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
