/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Userspace interface for AMD SEV and SNP guest driver.
 *
 * Copyright (C) 2021 Advanced Micro Devices, Inc.
 *
 * Author: Brijesh Singh <brijesh.singh@amd.com>
 *
 * SEV API specification is available at: https://developer.amd.com/sev/
 */

// Dependency: linux/types.h

pub const SNP_REPORT_USER_DATA_SIZE: usize = 64;

#[repr(C)]
pub struct snp_report_req {
    /* user data that should be included in the report */
    pub user_data: [u8; SNP_REPORT_USER_DATA_SIZE],

    /* The vmpl level to be included in the report */
    pub vmpl: u32,

    /* Must be zero filled */
    pub rsvd: [u8; 28],
}

#[repr(C)]
pub struct snp_report_resp {
    /* response data, see SEV-SNP spec for the format */
    pub data: [u8; 4000],
}

#[repr(C)]
pub struct snp_derived_key_req {
    pub root_key_select: u32,
    pub rsvd: u32,
    pub guest_field_select: u64,
    pub vmpl: u32,
    pub guest_svn: u32,
    pub tcb_version: u64,
}

#[repr(C)]
pub struct snp_derived_key_resp {
    /* response data, see SEV-SNP spec for the format */
    pub data: [u8; 64],
}

#[repr(C)]
pub union snp_guest_request_ioctl__bindgen_ty_1 {
    pub exitinfo2: u64,
    pub __bindgen_anon_1: snp_guest_request_ioctl__bindgen_ty_1__bindgen_ty_1,
}

#[repr(C)]
pub struct snp_guest_request_ioctl__bindgen_ty_1__bindgen_ty_1 {
    pub fw_error: u32,
    pub vmm_error: u32,
}

#[repr(C)]
pub struct snp_guest_request_ioctl {
    /* message version number (must be non-zero) */
    pub msg_version: u8,

    /* Request and response structure address */
    pub req_data: u64,
    pub resp_data: u64,

    /* bits[63:32]: VMM error code, bits[31:0] firmware error code (see psp-sev.h) */
    pub __bindgen_anon_1: snp_guest_request_ioctl__bindgen_ty_1,
}

#[repr(C)]
pub struct snp_ext_report_req {
    pub data: snp_report_req,

    /* where to copy the certificate blob */
    pub certs_address: u64,

    /* length of the certificate blob */
    pub certs_len: u32,
}

pub const SNP_GUEST_REQ_IOC_TYPE: u8 = b'S';

/* Get SNP attestation report */
pub const SNP_GET_REPORT: usize = _IOWR(SNP_GUEST_REQ_IOC_TYPE, 0x0, snp_guest_request_ioctl);

/* Get a derived key from the root */
pub const SNP_GET_DERIVED_KEY: usize = _IOWR(SNP_GUEST_REQ_IOC_TYPE, 0x1, snp_guest_request_ioctl);

/* Get SNP extended report as defined in the GHCB specification version 2. */
pub const SNP_GET_EXT_REPORT: usize = _IOWR(SNP_GUEST_REQ_IOC_TYPE, 0x2, snp_guest_request_ioctl);

/* Guest message request EXIT_INFO_2 constants */
pub const SNP_GUEST_FW_ERR_MASK: u64 = GENMASK_ULL(31, 0);
pub const SNP_GUEST_VMM_ERR_SHIFT: u32 = 32;

macro_rules! SNP_GUEST_VMM_ERR {
    ($x:expr) => {
        (($x as u64) << SNP_GUEST_VMM_ERR_SHIFT)
    };
}

macro_rules! SNP_GUEST_FW_ERR {
    ($x:expr) => {
        (($x) & SNP_GUEST_FW_ERR_MASK)
    };
}

macro_rules! SNP_GUEST_ERR {
    ($vmm_err:expr, $fw_err:expr) => {
        SNP_GUEST_VMM_ERR!($vmm_err) | SNP_GUEST_FW_ERR!($fw_err)
    };
}

pub const SNP_GUEST_VMM_ERR_INVALID_LEN: u32 = 1;
pub const SNP_GUEST_VMM_ERR_BUSY: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
