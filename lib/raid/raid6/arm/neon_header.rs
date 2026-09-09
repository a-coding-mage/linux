// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::{c_int, c_ulong, c_void};

unsafe extern "C" {
    pub fn raid6_neon1_gen_syndrome_real(
        disks: c_int,
        bytes: c_ulong,
        ptrs: *mut *mut c_void,
    );
    pub fn raid6_neon1_xor_syndrome_real(
        disks: c_int,
        start: c_int,
        stop: c_int,
        bytes: c_ulong,
        ptrs: *mut *mut c_void,
    );
    pub fn raid6_neon2_gen_syndrome_real(
        disks: c_int,
        bytes: c_ulong,
        ptrs: *mut *mut c_void,
    );
    pub fn raid6_neon2_xor_syndrome_real(
        disks: c_int,
        start: c_int,
        stop: c_int,
        bytes: c_ulong,
        ptrs: *mut *mut c_void,
    );
    pub fn raid6_neon4_gen_syndrome_real(
        disks: c_int,
        bytes: c_ulong,
        ptrs: *mut *mut c_void,
    );
    pub fn raid6_neon4_xor_syndrome_real(
        disks: c_int,
        start: c_int,
        stop: c_int,
        bytes: c_ulong,
        ptrs: *mut *mut c_void,
    );
    pub fn raid6_neon8_gen_syndrome_real(
        disks: c_int,
        bytes: c_ulong,
        ptrs: *mut *mut c_void,
    );
    pub fn raid6_neon8_xor_syndrome_real(
        disks: c_int,
        start: c_int,
        stop: c_int,
        bytes: c_ulong,
        ptrs: *mut *mut c_void,
    );
    pub fn __raid6_2data_recov_neon(
        bytes: c_int,
        p: *mut u8,
        q: *mut u8,
        dp: *mut u8,
        dq: *mut u8,
        pbmul: *const u8,
        qmul: *const u8,
    );
    pub fn __raid6_datap_recov_neon(
        bytes: c_int,
        p: *mut u8,
        q: *mut u8,
        dq: *mut u8,
        qmul: *const u8,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
