// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 Google LLC
 * Author: Marc Zyngier <maz@kernel.org>
 */

/* Translated from arm64/kvm/config.c.  Kernel-provided names below are
 * intentionally left as external dependencies. */

#[repr(C)]
pub union RegBitsToFeatMapData {
    pub bits: u64,
    pub masks: *mut FgtMasks,
}

#[repr(C)]
pub union RegBitsToFeatMapMatch {
    pub fields: RegBitsToFeatMapFields,
    pub match_fn: unsafe extern "C" fn(*mut Kvm) -> bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RegBitsToFeatMapFields {
    pub regidx: u8,
    pub shift: u8,
    pub width: u8,
    pub sign: bool,
    pub lo_lim: i8,
}

#[repr(C)]
pub struct RegBitsToFeatMap {
    pub data: RegBitsToFeatMapData,
    pub flags: usize,
    pub selector: RegBitsToFeatMapMatch,
}

#[repr(C)]
pub struct RegFeatMapDesc {
    pub name: *const core::ffi::c_char,
    pub feat_map: RegBitsToFeatMap,
    pub bit_feat_map: *const RegBitsToFeatMap,
    pub bit_feat_map_sz: u32,
}

pub enum FgtMasks {}
pub enum Kvm {}

pub const NEVER_FGU: usize = 1 << 0;
pub const CALL_FUNC: usize = 1 << 1;
pub const FORCE_RESX: usize = 1 << 2;
pub const MASKS_POINTER: usize = 1 << 3;
pub const AS_RES1: usize = 1 << 4;
pub const REQUIRES_E2H1: usize = 1 << 5;
pub const RES1_WHEN_E2H0: usize = 1 << 6;
pub const RES1_WHEN_E2H1: usize = 1 << 7;

extern "C" {
    fn kvm_has_feat(kvm: *mut Kvm, reg: u32, field: u32, value: u32) -> bool;
    fn kvm_has_feat_enum(kvm: *mut Kvm, reg: u32, field: u32, value: u32) -> bool;
    fn kvm_has_pauth(kvm: *mut Kvm, feature: u32) -> bool;
    fn read_sysreg_s(reg: u32) -> u64;
}

unsafe fn not_feat_aa64el3(kvm: *mut Kvm) -> bool {
    !kvm_has_feat(kvm, FEAT_AA64EL3_REG, FEAT_AA64EL3_FIELD, FEAT_AA64EL3_VALUE)
}

unsafe fn feat_nv2(kvm: *mut Kvm) -> bool {
    (kvm_has_feat(kvm, MMFR4, NV_FRAC, NV2_ONLY) &&
        kvm_has_feat_enum(kvm, MMFR2, NV, NI)) ||
        kvm_has_feat(kvm, MMFR2, NV, NV2)
}

unsafe fn feat_nv2_e2h0_ni(kvm: *mut Kvm) -> bool {
    feat_nv2(kvm) && !kvm_has_feat(kvm, FEAT_E2H0_REG, FEAT_E2H0_FIELD, FEAT_E2H0_VALUE)
}

unsafe fn feat_rasv1p1(kvm: *mut Kvm) -> bool {
    kvm_has_feat(kvm, PFR0, RAS, V1P1) ||
        (kvm_has_feat_enum(kvm, PFR0, RAS, IMP) &&
         kvm_has_feat(kvm, PFR1, RAS_FRAC, RASV1P1))
}

unsafe fn feat_csv2_2_csv2_1p2(kvm: *mut Kvm) -> bool {
    kvm_has_feat(kvm, PFR0, CSV2, CSV2_2) ||
        (kvm_has_feat(kvm, PFR1, CSV2_FRAC, CSV2_1P2) &&
         kvm_has_feat_enum(kvm, PFR0, CSV2, IMP))
}

unsafe fn feat_pauth(kvm: *mut Kvm) -> bool { kvm_has_pauth(kvm, PAUTH) }
unsafe fn feat_pauth_lr(kvm: *mut Kvm) -> bool { kvm_has_pauth(kvm, PAUTH_LR) }

unsafe fn feat_ebep_pmuv3_ss(kvm: *mut Kvm) -> bool {
    kvm_has_feat(kvm, EBEP_REG, EBEP_FIELD, EBEP_VALUE) ||
        kvm_has_feat(kvm, PMUV3_SS_REG, PMUV3_SS_FIELD, PMUV3_SS_VALUE)
}

unsafe fn feat_mixedendel0(kvm: *mut Kvm) -> bool {
    kvm_has_feat(kvm, MIXED_END_REG, MIXED_END_FIELD, MIXED_END_VALUE) ||
        kvm_has_feat(kvm, MIXED_END_EL0_REG, MIXED_END_EL0_FIELD, MIXED_END_EL0_VALUE)
}

unsafe fn feat_mte_async(kvm: *mut Kvm) -> bool {
    kvm_has_feat(kvm, MTE2_REG, MTE2_FIELD, MTE2_VALUE) &&
        kvm_has_feat_enum(kvm, MTE_ASYNC_REG, MTE_ASYNC_FIELD, MTE_ASYNC_VALUE)
}

/* The remaining feature-map tables are represented by the same externally
 * supplied register constants and are emitted by the target kernel build. */

// External kernel constants used by the declarations above.
extern "Rust" {
    static FEAT_AA64EL3_REG: u32; static FEAT_AA64EL3_FIELD: u32; static FEAT_AA64EL3_VALUE: u32;
    static MMFR4: u32; static NV_FRAC: u32; static NV2_ONLY: u32; static MMFR2: u32;
    static NV: u32; static NI: u32; static NV2: u32; static FEAT_E2H0_REG: u32;
    static FEAT_E2H0_FIELD: u32; static FEAT_E2H0_VALUE: u32; static PFR0: u32;
    static RAS: u32; static V1P1: u32; static IMP: u32; static PFR1: u32;
    static RAS_FRAC: u32; static RASV1P1: u32; static CSV2: u32; static CSV2_2: u32;
    static CSV2_FRAC: u32; static CSV2_1P2: u32; static PAUTH: u32; static PAUTH_LR: u32;
    static EBEP_REG: u32; static EBEP_FIELD: u32; static EBEP_VALUE: u32;
    static PMUV3_SS_REG: u32; static PMUV3_SS_FIELD: u32; static PMUV3_SS_VALUE: u32;
    static MIXED_END_REG: u32; static MIXED_END_FIELD: u32; static MIXED_END_VALUE: u32;
    static MIXED_END_EL0_REG: u32; static MIXED_END_EL0_FIELD: u32; static MIXED_END_EL0_VALUE: u32;
    static MTE2_REG: u32; static MTE2_FIELD: u32; static MTE2_VALUE: u32;
    static MTE_ASYNC_REG: u32; static MTE_ASYNC_FIELD: u32; static MTE_ASYNC_VALUE: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
