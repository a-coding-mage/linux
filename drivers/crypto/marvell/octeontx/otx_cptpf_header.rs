/* SPDX-License-Identifier: GPL-2.0
 * Marvell OcteonTX CPT driver
 *
 * Copyright (C) 2019 Marvell International Ltd.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

/* Dependencies corresponding to linux/types.h, linux/device.h, and
 * otx_cptpf_ucode.h are supplied externally. */

/*
 * OcteonTX CPT device structure
 */
#[repr(C)]
pub struct otx_cpt_device {
    pub reg_base: *mut core::ffi::c_void, /* Register start address */
    pub pdev: *mut pci_dev, /* Pci device handle */
    pub eng_grps: otx_cpt_eng_grps, /* Engine groups information */
    pub list: list_head,
    pub pf_type: u8, /* PF type SE or AE */
    pub max_vfs: u8, /* Maximum number of VFs supported by the CPT */
    pub vfs_enabled: u8, /* Number of enabled VFs */
}

extern "C" {
    pub fn otx_cpt_mbox_intr_handler(cpt: *mut otx_cpt_device, mbx: core::ffi::c_int);
    pub fn otx_cpt_disable_all_cores(cpt: *mut otx_cpt_device);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
