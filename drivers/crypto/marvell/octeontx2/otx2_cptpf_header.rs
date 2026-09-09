/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (C) 2020 Marvell.
 */

// Dependencies supplied by the corresponding C headers:
// otx2_cpt_common.h, otx2_cptpf_ucode.h, and otx2_cptlf.h.

pub struct otx2_cptpf_dev;

#[repr(C)]
pub struct otx2_cptvf_info {
    /* PF pointer this VF belongs to */
    pub cptpf: *mut otx2_cptpf_dev,
    pub vfpf_mbox_work: work_struct,
    pub vf_dev: *mut pci_dev,
    pub vf_id: ::core::ffi::c_int,
    pub intr_idx: ::core::ffi::c_int,
}

#[repr(C)]
pub struct cptpf_flr_work {
    pub work: work_struct,
    pub pf: *mut otx2_cptpf_dev,
}

#[repr(C)]
pub struct otx2_cptpf_dev {
    /* CPT PF registers start address */
    pub reg_base: *mut ::core::ffi::c_void,
    /* PF-AF mbox start address */
    pub afpf_mbox_base: *mut ::core::ffi::c_void,
    /* VF-PF mbox start address */
    pub vfpf_mbox_base: *mut ::core::ffi::c_void,
    /* PCI device handle */
    pub pdev: *mut pci_dev,
    pub vf: [otx2_cptvf_info; OTX2_CPT_MAX_VFS_NUM as usize],
    /* Engine groups information */
    pub eng_grps: otx2_cpt_eng_grps,
    /* CPT LFs attached to this PF */
    pub lfs: otx2_cptlfs_info,
    /* CPT1 LFs attached to this PF */
    pub cpt1_lfs: otx2_cptlfs_info,
    /* HW capabilities for each engine type */
    pub eng_caps: [otx2_cpt_eng_caps; OTX2_CPT_MAX_ENG_TYPES as usize],
    pub is_eng_caps_discovered: bool,

    /* AF <=> PF mbox */
    pub afpf_mbox: otx2_mbox,
    pub afpf_mbox_work: work_struct,
    pub afpf_mbox_wq: *mut workqueue_struct,

    pub afpf_mbox_up: otx2_mbox,
    pub afpf_mbox_up_work: work_struct,

    /* VF <=> PF mbox */
    pub vfpf_mbox: otx2_mbox,
    pub vfpf_mbox_wq: *mut workqueue_struct,

    pub flr_wq: *mut workqueue_struct,
    pub flr_work: *mut cptpf_flr_work,
    /* serialize mailbox access */
    pub lock: mutex,

    pub cap_flag: ::core::ffi::c_ulong,
    /* RVU PF number */
    pub pf_id: u8,
    /* Maximum number of VFs supported by CPT */
    pub max_vfs: u8,
    /* Number of enabled VFs */
    pub enabled_vfs: u8,
    /* SSO PF_FUNC override bit */
    pub sso_pf_func_ovrd: u8,
    /* Kernel crypto limits */
    pub kvf_limits: u8,
    pub has_cpt1: bool,
    pub rsrc_req_blkaddr: u8,

    /* Devlink */
    pub dl: *mut devlink,
}

pub extern "C" fn otx2_cptpf_afpf_mbox_intr(
    irq: ::core::ffi::c_int,
    arg: *mut ::core::ffi::c_void,
) -> irqreturn_t;
pub extern "C" fn otx2_cptpf_afpf_mbox_handler(work: *mut work_struct);
pub extern "C" fn otx2_cptpf_afpf_mbox_up_handler(work: *mut work_struct);
pub extern "C" fn otx2_cptpf_vfpf_mbox_intr(
    irq: ::core::ffi::c_int,
    arg: *mut ::core::ffi::c_void,
) -> irqreturn_t;
pub extern "C" fn otx2_cptpf_vfpf_mbox_handler(work: *mut work_struct);

pub extern "C" fn otx2_inline_cptlf_setup(
    cptpf: *mut otx2_cptpf_dev,
    lfs: *mut otx2_cptlfs_info,
    egrp: u8,
    num_lfs: ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub extern "C" fn otx2_inline_cptlf_cleanup(lfs: *mut otx2_cptlfs_info);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
