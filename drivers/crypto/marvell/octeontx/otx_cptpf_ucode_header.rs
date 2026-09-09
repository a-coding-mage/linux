/* SPDX-License-Identifier: GPL-2.0
 * Marvell OcteonTX CPT driver
 *
 * Copyright (C) 2019 Marvell International Ltd.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

// Dependencies supplied by the Linux kernel and the surrounding project are
// intentionally left external, corresponding to the C header includes.

pub const OTX_CPT_UCODE_NAME_LENGTH: usize = 64;
pub const OTX_CPT_MAX_ETYPES_PER_GRP: usize = 1;
pub const OTX_CPT_UCODE_TAR_FILE_NAME: &str = "cpt8x-mc.tar";
pub const OTX_CPT_UCODE_ALIGNMENT: usize = 128;
pub const OTX_CPT_UCODE_SIGN_LEN: usize = 256;
pub const OTX_CPT_UCODE_VER_STR_SZ: usize = 44;
pub const OTX_CPT_MAX_ENGINES: usize = 64;
pub const OTX_CPT_ENGS_BITMASK_LEN: usize =
    OTX_CPT_MAX_ENGINES / (8 * core::mem::size_of::<core::ffi::c_ulong>());

#[repr(C)]
#[derive(Copy, Clone)]
pub enum otx_cpt_ucode_type {
    OTX_CPT_AE_UC_TYPE = 1,
    OTX_CPT_SE_UC_TYPE1 = 20,
    OTX_CPT_SE_UC_TYPE2 = 21,
    OTX_CPT_SE_UC_TYPE3 = 22,
}

#[repr(C)]
pub struct otx_cpt_bitmap {
    pub bits: [core::ffi::c_ulong; OTX_CPT_ENGS_BITMASK_LEN],
    pub size: core::ffi::c_int,
}

#[repr(C)]
pub struct otx_cpt_engines {
    pub r#type: core::ffi::c_int,
    pub count: core::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_ucode_ver_num {
    pub nn: u8,
    pub xx: u8,
    pub yy: u8,
    pub zz: u8,
}

#[repr(C)]
pub struct otx_cpt_ucode_hdr {
    pub ver_num: otx_cpt_ucode_ver_num,
    pub ver_str: [u8; OTX_CPT_UCODE_VER_STR_SZ],
    pub code_length: u32,
    pub padding: [u32; 3],
}

#[repr(C)]
pub struct otx_cpt_ucode {
    pub ver_str: [u8; OTX_CPT_UCODE_VER_STR_SZ],
    pub ver_num: otx_cpt_ucode_ver_num,
    pub filename: [core::ffi::c_char; OTX_CPT_UCODE_NAME_LENGTH],
    pub dma: dma_addr_t,
    pub align_dma: dma_addr_t,
    pub va: *mut core::ffi::c_void,
    pub align_va: *mut core::ffi::c_void,
    pub size: u32,
    pub r#type: core::ffi::c_int,
}

#[repr(C)]
pub struct tar_ucode_info_t {
    pub list: list_head,
    pub ucode: otx_cpt_ucode,
    pub ucode_ptr: *const u8,
}

#[repr(C)]
pub struct otx_cpt_engs_available {
    pub max_se_cnt: core::ffi::c_int,
    pub max_ae_cnt: core::ffi::c_int,
    pub se_cnt: core::ffi::c_int,
    pub ae_cnt: core::ffi::c_int,
}

#[repr(C)]
pub struct otx_cpt_engs_rsvd {
    pub r#type: core::ffi::c_int,
    pub count: core::ffi::c_int,
    pub offset: core::ffi::c_int,
    pub bmap: *mut core::ffi::c_ulong,
    pub ucode: *mut otx_cpt_ucode,
}

#[repr(C)]
pub struct otx_cpt_mirror_info {
    pub is_ena: core::ffi::c_int,
    pub idx: core::ffi::c_int,
    pub ref_count: core::ffi::c_int,
}

#[repr(C)]
pub struct otx_cpt_eng_grp_info {
    pub g: *mut otx_cpt_eng_grps,
    pub info_attr: device_attribute,
    pub engs: [otx_cpt_engs_rsvd; OTX_CPT_MAX_ETYPES_PER_GRP],
    pub ucode: [otx_cpt_ucode; OTX_CPT_MAX_ETYPES_PER_GRP],
    pub sysfs_info_name: [core::ffi::c_char; OTX_CPT_UCODE_NAME_LENGTH],
    pub mirror: otx_cpt_mirror_info,
    pub idx: core::ffi::c_int,
    pub is_enabled: bool,
}

#[repr(C)]
pub struct otx_cpt_eng_grps {
    pub grp: [otx_cpt_eng_grp_info; OTX_CPT_MAX_ENGINE_GROUPS],
    pub ucode_load_attr: device_attribute,
    pub avail: otx_cpt_engs_available,
    pub lock: mutex,
    pub obj: *mut core::ffi::c_void,
    pub engs_num: core::ffi::c_int,
    pub eng_types_supported: core::ffi::c_int,
    pub eng_ref_cnt: [u8; OTX_CPT_MAX_ENGINES],
    pub is_ucode_load_created: bool,
    pub is_first_try: bool,
    pub is_rdonly: bool,
}

extern "C" {
    pub fn otx_cpt_init_eng_grps(
        pdev: *mut pci_dev,
        eng_grps: *mut otx_cpt_eng_grps,
        pf_type: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn otx_cpt_cleanup_eng_grps(
        pdev: *mut pci_dev,
        eng_grps: *mut otx_cpt_eng_grps,
    );
    pub fn otx_cpt_try_create_default_eng_grps(
        pdev: *mut pci_dev,
        eng_grps: *mut otx_cpt_eng_grps,
        pf_type: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn otx_cpt_set_eng_grps_is_rdonly(
        eng_grps: *mut otx_cpt_eng_grps,
        is_rdonly: bool,
    );
    pub fn otx_cpt_uc_supports_eng_type(
        ucode: *mut otx_cpt_ucode,
        eng_type: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
