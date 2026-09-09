/* SPDX-License-Identifier: (GPL-2.0 OR Linux-OpenIB) */
/*
 * Copyright (c) 2017-2018 Mellanox Technologies. All rights reserved.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ib_signature_prot_cap {
    IB_PROT_T10DIF_TYPE_1 = 1,
    IB_PROT_T10DIF_TYPE_2 = 1 << 1,
    IB_PROT_T10DIF_TYPE_3 = 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ib_signature_guard_cap {
    IB_GUARD_T10DIF_CRC = 1,
    IB_GUARD_T10DIF_CSUM = 1 << 1,
}

/**
 * enum ib_signature_type - Signature types
 * @IB_SIG_TYPE_NONE: Unprotected.
 * @IB_SIG_TYPE_T10_DIF: Type T10-DIF
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ib_signature_type {
    IB_SIG_TYPE_NONE,
    IB_SIG_TYPE_T10_DIF,
}

/**
 * enum ib_t10_dif_bg_type - Signature T10-DIF block-guard types
 * @IB_T10DIF_CRC: Corresponds to T10-PI mandated CRC checksum rules.
 * @IB_T10DIF_CSUM: Corresponds to IP checksum rules.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ib_t10_dif_bg_type {
    IB_T10DIF_CRC,
    IB_T10DIF_CSUM,
}

/**
 * struct ib_t10_dif_domain - Parameters specific for T10-DIF
 *     domain.
 * @bg_type: T10-DIF block guard type (CRC|CSUM)
 * @pi_interval: protection information interval.
 * @bg: seed of guard computation.
 * @app_tag: application tag of guard block
 * @ref_tag: initial guard block reference tag.
 * @ref_remap: Indicate wethear the reftag increments each block
 * @app_escape: Indicate to skip block check if apptag=0xffff
 * @ref_escape: Indicate to skip block check if reftag=0xffffffff
 * @apptag_check_mask: check bitmask of application tag.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_t10_dif_domain {
    pub bg_type: ib_t10_dif_bg_type,
    pub pi_interval: u16,
    pub bg: u16,
    pub app_tag: u16,
    pub ref_tag: u32,
    pub ref_remap: bool,
    pub app_escape: bool,
    pub ref_escape: bool,
    pub apptag_check_mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ib_sig_domain_sig {
    pub dif: ib_t10_dif_domain,
}

/**
 * struct ib_sig_domain - Parameters for signature domain
 * @sig_type: specific signauture type
 * @sig: union of all signature domain attributes that may
 *     be used to set domain layout.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_sig_domain {
    pub sig_type: ib_signature_type,
    pub sig: ib_sig_domain_sig,
}

/**
 * struct ib_sig_attrs - Parameters for signature handover operation
 * @check_mask: bitmask for signature byte check (8 bytes)
 * @mem: memory domain layout descriptor.
 * @wire: wire domain layout descriptor.
 * @meta_length: metadata length
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_sig_attrs {
    pub check_mask: u8,
    pub mem: ib_sig_domain,
    pub wire: ib_sig_domain,
    pub meta_length: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ib_sig_err_type {
    IB_SIG_BAD_GUARD,
    IB_SIG_BAD_REFTAG,
    IB_SIG_BAD_APPTAG,
}

/*
 * Signature check masks (8 bytes in total) according to the T10-PI standard:
 *  -------- -------- ------------
 * | GUARD  | APPTAG |   REFTAG   |
 * |  2B    |  2B    |    4B      |
 *  -------- -------- ------------
 */
pub const IB_SIG_CHECK_GUARD: i32 = 0xc0;
pub const IB_SIG_CHECK_APPTAG: i32 = 0x30;
pub const IB_SIG_CHECK_REFTAG: i32 = 0x0f;

/*
 * struct ib_sig_err - signature error descriptor
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_sig_err {
    pub err_type: ib_sig_err_type,
    pub expected: u32,
    pub actual: u32,
    pub sig_err_offset: u64,
    pub key: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
