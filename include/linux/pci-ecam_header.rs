/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2016 Broadcom
 */

/* Dependencies supplied by the surrounding kernel translation. */

/*
 * Memory address shift values for the byte-level address that
 * can be used when accessing the PCI Express Configuration Space.
 */

/*
 * Enhanced Configuration Access Mechanism (ECAM)
 *
 * See PCI Express Base Specification, Revision 5.0, Version 1.0,
 * Section 7.2.2, Table 7-1, p. 677.
 */
pub const PCIE_ECAM_BUS_SHIFT: u32 = 20; /* Bus number */
pub const PCIE_ECAM_DEVFN_SHIFT: u32 = 12; /* Device and Function number */

pub const PCIE_ECAM_BUS_MASK: u32 = 0xff;
pub const PCIE_ECAM_DEVFN_MASK: u32 = 0xff;
pub const PCIE_ECAM_REG_MASK: u32 = 0xfff; /* Limit offset to a maximum of 4K */

#[inline]
pub const fn PCIE_ECAM_BUS(x: u32) -> u32 {
    (x & PCIE_ECAM_BUS_MASK) << PCIE_ECAM_BUS_SHIFT
}

#[inline]
pub const fn PCIE_ECAM_DEVFN(x: u32) -> u32 {
    (x & PCIE_ECAM_DEVFN_MASK) << PCIE_ECAM_DEVFN_SHIFT
}

#[inline]
pub const fn PCIE_ECAM_REG(x: u32) -> u32 {
    x & PCIE_ECAM_REG_MASK
}

#[inline]
pub const fn PCIE_ECAM_OFFSET(bus: u32, devfn: u32, where_: u32) -> u32 {
    PCIE_ECAM_BUS(bus) | PCIE_ECAM_DEVFN(devfn) | PCIE_ECAM_REG(where_)
}

/*
 * struct to hold pci ops and bus shift of the config window
 * for a PCI controller.
 */
#[repr(C)]
pub struct pci_ecam_ops {
    pub bus_shift: core::ffi::c_uint,
    pub pci_ops: pci_ops,
    pub init: Option<unsafe extern "C" fn(*mut pci_config_window) -> core::ffi::c_int>,
    pub enable_device: Option<unsafe extern "C" fn(*mut pci_host_bridge, *mut pci_dev) -> core::ffi::c_int>,
    pub disable_device: Option<unsafe extern "C" fn(*mut pci_host_bridge, *mut pci_dev)>,
}

/*
 * struct to hold the mappings of a config space window. This
 * is expected to be used as sysdata for PCI controllers that
 * use ECAM.
 */
#[repr(C)]
pub union pci_config_window_win {
    pub win: *mut core::ffi::c_void, /* 64-bit single mapping */
    pub winp: *mut *mut core::ffi::c_void, /* 32-bit per-bus mapping */
}

#[repr(C)]
pub struct pci_config_window {
    pub res: resource,
    pub busr: resource,
    pub bus_shift: core::ffi::c_uint,
    pub priv_: *mut core::ffi::c_void,
    pub ops: *const pci_ecam_ops,
    pub win: pci_config_window_win,
    pub parent: *mut device, /* ECAM res was from this dev */
}

/* create and free pci_config_window */
extern "C" {
    pub fn pci_ecam_create(
        dev: *mut device,
        cfgres: *mut resource,
        busr: *mut resource,
        ops: *const pci_ecam_ops,
    ) -> *mut pci_config_window;
    pub fn pci_ecam_free(cfg: *mut pci_config_window);
}

/* map_bus when ->sysdata is an instance of pci_config_window */
extern "C" {
    pub fn pci_ecam_map_bus(bus: *mut pci_bus, devfn: core::ffi::c_uint, where_: core::ffi::c_int)
        -> *mut core::ffi::c_void;
}

/* default ECAM ops */
extern "C" {
    pub static pci_generic_ecam_ops: pci_ecam_ops;
}

/* default CAM ops */
extern "C" {
    pub static pci_generic_cam_ops: pci_ecam_ops;
}

/* The following declarations are enabled when CONFIG_ACPI and CONFIG_PCI_QUIRKS are defined. */
#[cfg(all(CONFIG_ACPI, CONFIG_PCI_QUIRKS))]
extern "C" {
    pub static pci_32b_ops: pci_ecam_ops; /* 32-bit accesses only */
    pub static pci_32b_read_ops: pci_ecam_ops; /* 32-bit read only */
    pub static hisi_pcie_ops: pci_ecam_ops; /* HiSilicon */
    pub static thunder_pem_ecam_ops: pci_ecam_ops; /* Cavium ThunderX 1.x & 2.x */
    pub static pci_thunder_ecam_ops: pci_ecam_ops; /* Cavium ThunderX 1.x */
    pub static xgene_v1_pcie_ecam_ops: pci_ecam_ops; /* APM X-Gene PCIe v1 */
    pub static xgene_v2_pcie_ecam_ops: pci_ecam_ops; /* APM X-Gene PCIe v2.x */
    pub static al_pcie_ops: pci_ecam_ops; /* Amazon Annapurna Labs PCIe */
    pub static tegra194_pcie_ops: pci_ecam_ops; /* Tegra194 PCIe */
    pub static loongson_pci_ecam_ops: pci_ecam_ops; /* Loongson PCIe */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
