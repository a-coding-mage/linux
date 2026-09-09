/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright(c) 2023 Advanced Micro Devices, Inc */

// Dependency supplied by <linux/auxiliary_bus.h>.
pub struct auxiliary_device;
pub struct pci_dev;
pub union pds_core_adminq_cmd {
    _bindgen_opaque: [u8; 0],
}
pub union pds_core_adminq_comp {
    _bindgen_opaque: [u8; 0],
}

#[repr(C)]
pub struct pds_auxiliary_dev {
    pub aux_dev: auxiliary_device,
    pub vf_pdev: *mut pci_dev,
    pub client_id: u16,
}

extern "C" {
    pub fn pds_client_adminq_cmd(
        padev: *mut pds_auxiliary_dev,
        req: *mut pds_core_adminq_cmd,
        req_len: usize,
        resp: *mut pds_core_adminq_comp,
        flags: u64,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
