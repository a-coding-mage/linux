/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (C) 2020 Marvell.
 */

// Translated from otx2_cptvf.h.
// C dependencies supplied by other headers are referenced as external Rust types.

use core::ffi::c_void;

#[repr(C)]
pub struct otx2_cptvf_dev {
    pub reg_base: *mut c_void,             // Register start address
    pub pfvf_mbox_base: *mut c_void,       // PF-VF mbox start address
    pub pdev: *mut pci_dev,                // PCI device handle
    pub lfs: otx2_cptlfs_info,             // CPT LFs attached to this VF
    pub vf_id: u8,                         // Virtual function index

    // PF <=> VF mbox
    pub pfvf_mbox: otx2_mbox,
    pub pfvf_mbox_work: work_struct,
    pub pfvf_mbox_wq: *mut workqueue_struct,
    pub blkaddr: i32,
    pub bbuf_base: *mut c_void,
    pub cap_flag: usize,
    pub eng_caps: [u64; OTX2_CPT_MAX_ENG_TYPES],
}

extern "C" {
    pub fn otx2_cptvf_pfvf_mbox_intr(irq: i32, arg: *mut c_void) -> irqreturn_t;
    pub fn otx2_cptvf_pfvf_mbox_handler(work: *mut work_struct);
    pub fn otx2_cptvf_send_eng_grp_num_msg(
        cptvf: *mut otx2_cptvf_dev,
        eng_type: i32,
    ) -> i32;
    pub fn otx2_cptvf_send_kvf_limits_msg(cptvf: *mut otx2_cptvf_dev) -> i32;
    pub fn otx2_cpt_mbox_bbuf_init(
        cptvf: *mut otx2_cptvf_dev,
        pdev: *mut pci_dev,
    ) -> i32;
    pub fn otx2_cptvf_send_caps_msg(cptvf: *mut otx2_cptvf_dev) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
