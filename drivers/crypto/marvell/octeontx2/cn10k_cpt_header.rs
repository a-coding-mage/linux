/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (C) 2021 Marvell.
 */

// Dependencies supplied by the corresponding CPT headers are intentionally
// referenced here rather than implemented in this translation.

pub const CN10K_CPT_HW_CTX_SIZE: u64 = 256;

#[repr(C)]
pub union cn10k_cpt_hw_ctx {
    pub u: u64,
    pub w0: cn10k_cpt_hw_ctx_w0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn10k_cpt_hw_ctx_w0 {
    // C bitfields: reserved_0_47:48, ctx_push_sz:7, reserved_55:1,
    // ctx_hdr_sz:2, aop_valid:1, reserved_59:1, ctx_sz:4.
    pub bits: u64,
}

#[repr(C)]
pub struct cn10k_cpt_errata_ctx {
    pub hw_ctx: *mut cn10k_cpt_hw_ctx,
    pub cptr_dma: u64,
}

#[repr(C)]
pub union otx2_cpt_res_s {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct cn10k_cpt_res_s {
    pub compcode: u8,
    pub uc_compcode: u8,
}

#[repr(C)]
pub struct cn9k_cpt_res_s {
    pub compcode: u8,
    pub uc_compcode: u8,
}

#[repr(C)]
pub struct otx2_cptpf_dev {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct otx2_cptvf_dev {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct pci_dev {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct otx2_cptlfs_info {
    _opaque: [u8; 0],
}

#[inline]
pub unsafe fn cn10k_cpt_get_compcode(result: *mut otx2_cpt_res_s) -> u8 {
    (*(result as *mut cn10k_cpt_res_s)).compcode
}

#[inline]
pub unsafe fn cn10k_cpt_get_uc_compcode(result: *mut otx2_cpt_res_s) -> u8 {
    (*(result as *mut cn10k_cpt_res_s)).uc_compcode
}

#[inline]
pub unsafe fn otx2_cpt_get_compcode(result: *mut otx2_cpt_res_s) -> u8 {
    (*(result as *mut cn9k_cpt_res_s)).compcode
}

#[inline]
pub unsafe fn otx2_cpt_get_uc_compcode(result: *mut otx2_cpt_res_s) -> u8 {
    (*(result as *mut cn9k_cpt_res_s)).uc_compcode
}

extern "C" {
    pub fn cn10k_cptpf_lmtst_init(cptpf: *mut otx2_cptpf_dev) -> i32;
    pub fn cn10k_cptvf_lmtst_init(cptvf: *mut otx2_cptvf_dev) -> i32;
    pub fn cn10k_cpt_lmtst_free(pdev: *mut pci_dev, lfs: *mut otx2_cptlfs_info);
    pub fn cn10k_cpt_ctx_flush(pdev: *mut pci_dev, cptr: u64, inval: bool);
    pub fn cn10k_cpt_hw_ctx_init(
        pdev: *mut pci_dev,
        er_ctx: *mut cn10k_cpt_errata_ctx,
    ) -> i32;
    pub fn cn10k_cpt_hw_ctx_clear(
        pdev: *mut pci_dev,
        er_ctx: *mut cn10k_cpt_errata_ctx,
    );
    pub fn cn10k_cpt_hw_ctx_set(hctx: *mut cn10k_cpt_hw_ctx, ctx_sz: u16);
    pub fn cptvf_hw_ops_get(cptvf: *mut otx2_cptvf_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
