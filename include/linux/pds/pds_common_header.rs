/* SPDX-License-Identifier: (GPL-2.0 OR Linux-OpenIB) OR BSD-2-Clause */
/* Copyright(c) 2023 Advanced Micro Devices, Inc. */

/* Dependency supplied by the surrounding kernel translation. */

pub const PDS_CORE_DRV_NAME: &str = "pds_core";

/* the device's internal addressing uses up to 52 bits */
pub const PDS_CORE_ADDR_LEN: u32 = 52;
pub const PDS_CORE_ADDR_MASK: u64 = (1u64 << PDS_ADDR_LEN) - 1;
pub const PDS_PAGE_SIZE: u32 = 4096;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum pds_core_driver_type {
    PDS_DRIVER_LINUX = 1,
    PDS_DRIVER_WIN = 2,
    PDS_DRIVER_DPDK = 3,
    PDS_DRIVER_FREEBSD = 4,
    PDS_DRIVER_IPXE = 5,
    PDS_DRIVER_ESXI = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum pds_core_vif_types {
    PDS_DEV_TYPE_CORE = 0,
    PDS_DEV_TYPE_VDPA = 1,
    PDS_DEV_TYPE_VFIO = 2,
    PDS_DEV_TYPE_ETH = 3,
    PDS_DEV_TYPE_RDMA = 4,
    PDS_DEV_TYPE_LM = 5,
    PDS_DEV_TYPE_FWCTL = 6,

    /* new ones added before this line */
    PDS_DEV_TYPE_MAX = 16, /* don't change - used in struct size */
}

pub const PDS_DEV_TYPE_CORE_STR: &str = "Core";
pub const PDS_DEV_TYPE_VDPA_STR: &str = "vDPA";
pub const PDS_DEV_TYPE_VFIO_STR: &str = "vfio";
pub const PDS_DEV_TYPE_ETH_STR: &str = "Eth";
pub const PDS_DEV_TYPE_RDMA_STR: &str = "RDMA";
pub const PDS_DEV_TYPE_LM_STR: &str = "LM";
pub const PDS_DEV_TYPE_FWCTL_STR: &str = "fwctl";

pub const PDS_VDPA_DEV_NAME: &str = "pds_core.vDPA";
pub const PDS_VFIO_LM_DEV_NAME: &str = "pds_core.LM.vfio";

#[repr(C)]
pub struct pdsc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn pdsc_register_notify(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn pdsc_unregister_notify(nb: *mut notifier_block);
    pub fn pdsc_get_pf_struct(vf_pdev: *mut pci_dev) -> *mut ::core::ffi::c_void;
    pub fn pds_client_register(pf: *mut pdsc, devname: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn pds_client_unregister(pf: *mut pdsc, client_id: u16) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
