/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (C) 2020 Marvell.
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const OTX2_CPT_MAX_ETYPES_PER_GRP: usize = 2;
pub const OTX2_CPT_UCODE_SIGN_LEN: usize = 256;
pub const OTX2_CPT_UCODE_VER_STR_SZ: usize = 44;
pub const OTX2_CPT_MAX_ENGINES: usize = 144;
pub const OTX2_CPT_ENGS_BITMASK_LEN: usize =
    (OTX2_CPT_MAX_ENGINES + (usize::BITS as usize) - 1) / (usize::BITS as usize);
pub const OTX2_CPT_UCODE_SZ: usize = 64 * 1024;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum otx2_cpt_ucode_type {
    OTX2_CPT_AE_UC_TYPE = 1,
    OTX2_CPT_SE_UC_TYPE1 = 20,
    OTX2_CPT_SE_UC_TYPE2 = 21,
    OTX2_CPT_SE_UC_TYPE3 = 22,
    OTX2_CPT_IE_UC_TYPE1 = 30,
    OTX2_CPT_IE_UC_TYPE2 = 31,
    OTX2_CPT_IE_UC_TYPE3 = 32,
}

#[repr(C)]
pub struct otx2_cpt_bitmap {
    pub bits: [usize; OTX2_CPT_ENGS_BITMASK_LEN],
    pub size: i32,
}

#[repr(C)]
pub struct otx2_cpt_engines {
    pub type_: i32,
    pub count: i32,
}

#[repr(C)]
pub struct otx2_cpt_ucode_ver_num {
    pub nn: u8,
    pub xx: u8,
    pub yy: u8,
    pub zz: u8,
}

#[repr(C)]
pub struct otx2_cpt_ucode_hdr {
    pub ver_num: otx2_cpt_ucode_ver_num,
    pub ver_str: [u8; OTX2_CPT_UCODE_VER_STR_SZ],
    pub code_length: u32,
    pub padding: [u32; 3],
}

#[repr(C)]
pub struct otx2_cpt_ucode {
    pub ver_str: [u8; OTX2_CPT_UCODE_VER_STR_SZ + 1],
    pub ver_num: otx2_cpt_ucode_ver_num,
    pub filename: [i8; OTX2_CPT_NAME_LENGTH],
    pub dma: u64,
    pub va: *mut core::ffi::c_void,
    pub size: u32,
    pub type_: i32,
}

#[repr(C)]
pub struct otx2_cpt_uc_info_t {
    pub list: list_head,
    pub ucode: otx2_cpt_ucode,
    pub fw: *const firmware,
}

#[repr(C)]
pub struct otx2_cpt_engs_available {
    pub max_se_cnt: i32,
    pub max_ie_cnt: i32,
    pub max_ae_cnt: i32,
    pub se_cnt: i32,
    pub ie_cnt: i32,
    pub ae_cnt: i32,
}

#[repr(C)]
pub struct otx2_cpt_engs_rsvd {
    pub type_: i32,
    pub count: i32,
    pub offset: i32,
    pub bmap: *mut usize,
    pub ucode: *mut otx2_cpt_ucode,
}

#[repr(C)]
pub struct otx2_cpt_mirror_info {
    pub is_ena: i32,
    pub idx: i32,
    pub ref_count: i32,
}

#[repr(C)]
pub struct otx2_cpt_eng_grp_info {
    pub g: *mut otx2_cpt_eng_grps,
    pub engs: [otx2_cpt_engs_rsvd; OTX2_CPT_MAX_ETYPES_PER_GRP],
    pub ucode: [otx2_cpt_ucode; OTX2_CPT_MAX_ETYPES_PER_GRP],
    pub mirror: otx2_cpt_mirror_info,
    pub idx: i32,
    pub is_enabled: bool,
}

#[repr(C)]
pub struct otx2_cpt_eng_grps {
    pub lock: mutex,
    pub grp: [otx2_cpt_eng_grp_info; OTX2_CPT_MAX_ENGINE_GROUPS],
    pub avail: otx2_cpt_engs_available,
    pub obj: *mut core::ffi::c_void,
    pub engs_num: i32,
    pub eng_ref_cnt: [u8; OTX2_CPT_MAX_ENGINES],
    pub is_grps_created: bool,
    pub rid: u16,
}

pub struct pci_dev;
pub struct otx2_cptpf_dev;
pub struct devlink_param_gset_ctx;
pub struct list_head;
pub struct firmware;
pub struct mutex;

unsafe extern "C" {
    pub fn otx2_cpt_init_eng_grps(
        pdev: *mut pci_dev,
        eng_grps: *mut otx2_cpt_eng_grps,
    ) -> i32;
    pub fn otx2_cpt_cleanup_eng_grps(
        pdev: *mut pci_dev,
        eng_grps: *mut otx2_cpt_eng_grps,
    );
    pub fn otx2_cpt_create_eng_grps(
        cptpf: *mut otx2_cptpf_dev,
        eng_grps: *mut otx2_cpt_eng_grps,
    ) -> i32;
    pub fn otx2_cpt_disable_all_cores(cptpf: *mut otx2_cptpf_dev) -> i32;
    pub fn otx2_cpt_get_eng_grp(
        eng_grps: *mut otx2_cpt_eng_grps,
        eng_type: i32,
    ) -> i32;
    pub fn otx2_cpt_discover_eng_capabilities(cptpf: *mut otx2_cptpf_dev) -> i32;
    pub fn otx2_cpt_dl_custom_egrp_create(
        cptpf: *mut otx2_cptpf_dev,
        ctx: *mut devlink_param_gset_ctx,
    ) -> i32;
    pub fn otx2_cpt_dl_custom_egrp_delete(
        cptpf: *mut otx2_cptpf_dev,
        ctx: *mut devlink_param_gset_ctx,
    ) -> i32;
    pub fn find_engines_by_type(
        eng_grp: *mut otx2_cpt_eng_grp_info,
        eng_type: i32,
    ) -> *mut otx2_cpt_engs_rsvd;
}

/* Supplied by otx2_cpt_common.h and otx2_cpt_hw_types.h. */
extern "Rust" {
    static OTX2_CPT_NAME_LENGTH: usize;
    static OTX2_CPT_MAX_ENGINE_GROUPS: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
