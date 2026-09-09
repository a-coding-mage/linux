/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Userspace interface for TDX guest driver
 *
 * Copyright (C) 2022 Intel Corporation
 */

/* Length of the REPORTDATA used in TDG.MR.REPORT TDCALL */
pub const TDX_REPORTDATA_LEN: usize = 64;

/* Length of TDREPORT used in TDG.MR.REPORT TDCALL */
pub const TDX_REPORT_LEN: usize = 1024;

/**
 * struct tdx_report_req - Request struct for TDX_CMD_GET_REPORT0 IOCTL.
 *
 * @reportdata: User buffer with REPORTDATA to be included into TDREPORT.
 *              Typically it can be some nonce provided by attestation
 *              service, so the generated TDREPORT can be uniquely verified.
 * @tdreport: User buffer to store TDREPORT output from TDCALL[TDG.MR.REPORT].
 */
#[repr(C)]
pub struct tdx_report_req {
    pub reportdata: [u8; TDX_REPORTDATA_LEN],
    pub tdreport: [u8; TDX_REPORT_LEN],
}

/*
 * TDX_CMD_GET_REPORT0 - Get TDREPORT0 (a.k.a. TDREPORT subtype 0) using
 *                       TDCALL[TDG.MR.REPORT]
 *
 * Return 0 on success, -EIO on TDCALL execution failure, and
 * standard errno on other general error cases.
 *
 * Equivalent to _IOWR('T', 1, struct tdx_report_req).  The ioctl encoding
 * follows the Linux generic ioctl layout: direction=read|write, size=1088,
 * type='T', number=1.
 */
pub const TDX_CMD_GET_REPORT0: u64 = 0xc440_5401;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
