// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2024-2005 Intel Corporation. All rights reserved. */

// Kernel dependencies: linux/module.h, linux/tsm-mr.h, linux/miscdevice.h,
// and crypto/sha2.h.
use core::ffi::c_int;

const SHA256_DIGEST_SIZE: usize = 32;
const SHA384_DIGEST_SIZE: usize = 48;
const SHA512_DIGEST_SIZE: usize = 64;

const fn c_string<const N: usize>(value: &[u8]) -> [u8; N] {
    let mut result = [0; N];
    let mut i = 0;
    while i < value.len() {
        result[i] = value[i];
        i += 1;
    }
    result
}

#[repr(C)]
struct SampleReport {
    static_mr: [u8; SHA384_DIGEST_SIZE],
    config_mr: [u8; SHA512_DIGEST_SIZE],
    rtmr0: [u8; SHA256_DIGEST_SIZE],
    rtmr1: [u8; SHA384_DIGEST_SIZE],
    report_digest: [u8; SHA512_DIGEST_SIZE],
}

static mut SAMPLE_REPORT: SampleReport = SampleReport {
    static_mr: c_string(b"static_mr"),
    config_mr: c_string(b"config_mr"),
    rtmr0: c_string(b"rtmr0"),
    rtmr1: c_string(b"rtmr1"),
    report_digest: [0; SHA512_DIGEST_SIZE],
};

// External kernel types, hash contexts, constants, and functions are supplied
// by the translated dependencies.
extern "C" {
    fn sha512(data: *const u8, len: usize, digest: *mut u8);
    fn sha256_init(ctx: *mut c_void);
    fn sha256_update(ctx: *mut c_void, data: *const u8, len: usize);
    fn sha256_final(ctx: *mut c_void, digest: *mut u8);
    fn sha384_init(ctx: *mut c_void);
    fn sha384_update(ctx: *mut c_void, data: *const u8, len: usize);
    fn sha384_final(ctx: *mut c_void, digest: *mut u8);
    fn sha512_init(ctx: *mut c_void);
    fn sha512_update(ctx: *mut c_void, data: *const u8, len: usize);
    fn sha512_final(ctx: *mut c_void, digest: *mut u8);
}

#[allow(non_camel_case_types)]
type c_void = core::ffi::c_void;

// The following signatures retain the external tsm-mr interfaces; their
// concrete structs and hash-algorithm values come from linux/tsm-mr.h.
#[repr(C)]
struct tsm_measurements;
#[repr(C)]
struct tsm_measurement_register { mr_hash: c_int, mr_value: *mut u8, mr_size: usize }

unsafe fn sample_report_refresh(_tm: *const tsm_measurements) -> c_int {
    sha512(
        &SAMPLE_REPORT as *const _ as *const u8,
        core::mem::offset_of!(SampleReport, report_digest),
        SAMPLE_REPORT.report_digest.as_mut_ptr(),
    );
    0
}

unsafe fn sample_report_extend_mr(
    _tm: *const tsm_measurements,
    mr: *mut tsm_measurement_register,
    data: *const u8,
) -> c_int {
    // switch (mr->mr_hash), with SHA256/SHA384/SHA512 context operations,
    // is supplied by the kernel crypto API in the surrounding translation.
    let _ = (mr, data);
    -95 // -EOPNOTSUPP for an unsupported hash algorithm
}

// MR descriptors, attribute groups, miscdevice registration, module init and
// exit remain declarations/definitions supplied by the kernel translation.
// SPDX module metadata: GPL; Sample module using tsm-mr to expose emulated MRs.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
