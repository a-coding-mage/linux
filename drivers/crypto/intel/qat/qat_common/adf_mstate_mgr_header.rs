/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2024 Intel Corporation */

pub const ADF_MSTATE_ID_LEN: usize = 8;

pub const ADF_MSTATE_ETRB_IDS: &str = "ETRBAR";
pub const ADF_MSTATE_MISCB_IDS: &str = "MISCBAR";
pub const ADF_MSTATE_EXTB_IDS: &str = "EXTBAR";
pub const ADF_MSTATE_GEN_IDS: &str = "GENER";
pub const ADF_MSTATE_CONFIG_IDS: &str = "CONFIG";
pub const ADF_MSTATE_SECTION_NUM: usize = 5;

pub const ADF_MSTATE_BANK_IDX_IDS: &str = "bnk";

pub const ADF_MSTATE_ETR_REGS_IDS: &str = "mregs";
pub const ADF_MSTATE_VINTSRC_IDS: &str = "visrc";
pub const ADF_MSTATE_VINTMSK_IDS: &str = "vimsk";
pub const ADF_MSTATE_SLA_IDS: &str = "sla";
pub const ADF_MSTATE_IOV_INIT_IDS: &str = "iovinit";
pub const ADF_MSTATE_COMPAT_VER_IDS: &str = "compver";
pub const ADF_MSTATE_GEN_CAP_IDS: &str = "gencap";
pub const ADF_MSTATE_GEN_SVCMAP_IDS: &str = "svcmap";
pub const ADF_MSTATE_GEN_EXTDC_IDS: &str = "extdc";
pub const ADF_MSTATE_VINTSRC_PF2VM_IDS: &str = "vispv";
pub const ADF_MSTATE_VINTMSK_PF2VM_IDS: &str = "vimpv";
pub const ADF_MSTATE_VM2PF_IDS: &str = "vm2pf";
pub const ADF_MSTATE_PF2VM_IDS: &str = "pf2vm";

#[repr(C)]
pub struct adf_mstate_mgr {
    pub buf: *mut u8,
    pub state: *mut u8,
    pub size: u32,
    pub n_sects: u32,
}

#[repr(C)]
pub struct adf_mstate_preh {
    pub magic: u32,
    pub version: u32,
    pub preh_len: u16,
    pub n_sects: u16,
    pub size: u32,
}

#[repr(C)]
pub struct adf_mstate_vreginfo {
    pub addr: *mut core::ffi::c_void,
    pub size: u32,
}

#[repr(C)]
pub struct adf_mstate_sect_h;

pub type adf_mstate_preamble_checker = unsafe extern "C" fn(
    preamble: *mut adf_mstate_preh,
    opa: *mut core::ffi::c_void,
) -> i32;
pub type adf_mstate_populate = unsafe extern "C" fn(
    sub_mgr: *mut adf_mstate_mgr,
    buf: *mut u8,
    size: u32,
    opa: *mut core::ffi::c_void,
) -> i32;
pub type adf_mstate_action = unsafe extern "C" fn(
    sub_mgr: *mut adf_mstate_mgr,
    buf: *mut u8,
    size: u32,
    opa: *mut core::ffi::c_void,
) -> i32;

unsafe extern "C" {
    pub fn adf_mstate_mgr_new(buf: *mut u8, size: u32) -> *mut adf_mstate_mgr;
    pub fn adf_mstate_mgr_destroy(mgr: *mut adf_mstate_mgr);
    pub fn adf_mstate_mgr_init(mgr: *mut adf_mstate_mgr, buf: *mut u8, size: u32);
    pub fn adf_mstate_mgr_init_from_parent(
        mgr: *mut adf_mstate_mgr,
        p_mgr: *mut adf_mstate_mgr,
    );
    pub fn adf_mstate_mgr_init_from_psect(
        mgr: *mut adf_mstate_mgr,
        p_sect: *mut adf_mstate_sect_h,
    );
    pub fn adf_mstate_mgr_init_from_remote(
        mgr: *mut adf_mstate_mgr,
        buf: *mut u8,
        size: u32,
        checker: adf_mstate_preamble_checker,
        opaque: *mut core::ffi::c_void,
    ) -> i32;
    pub fn adf_mstate_preamble_add(mgr: *mut adf_mstate_mgr) -> *mut adf_mstate_preh;
    pub fn adf_mstate_preamble_update(mgr: *mut adf_mstate_mgr) -> i32;
    pub fn adf_mstate_state_size(mgr: *mut adf_mstate_mgr) -> u32;
    pub fn adf_mstate_state_size_from_remote(mgr: *mut adf_mstate_mgr) -> u32;
    pub fn adf_mstate_sect_update(
        p_mgr: *mut adf_mstate_mgr,
        curr_mgr: *mut adf_mstate_mgr,
        sect: *mut adf_mstate_sect_h,
    );
    pub fn adf_mstate_sect_add_vreg(
        mgr: *mut adf_mstate_mgr,
        id: *const core::ffi::c_char,
        info: *mut adf_mstate_vreginfo,
    ) -> *mut adf_mstate_sect_h;
    pub fn adf_mstate_sect_add(
        mgr: *mut adf_mstate_mgr,
        id: *const core::ffi::c_char,
        populate: adf_mstate_populate,
        opaque: *mut core::ffi::c_void,
    ) -> *mut adf_mstate_sect_h;
    pub fn adf_mstate_sect_lookup(
        mgr: *mut adf_mstate_mgr,
        id: *const core::ffi::c_char,
        action: adf_mstate_action,
        opaque: *mut core::ffi::c_void,
    ) -> *mut adf_mstate_sect_h;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
