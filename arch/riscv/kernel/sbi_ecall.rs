// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Rivos Inc. */

use core::arch::asm;

// Declarations supplied by the SBI and tracing headers.
pub const SBI_EXT_BASE: usize = 0x10;

#[repr(C)]
pub struct sbiret {
    pub error: usize,
    pub value: usize,
}

extern "C" {
    fn sbi_ecall(
        ext: usize,
        fid: ::core::ffi::c_int,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> sbiret;
    fn sbi_err_map_linux_errno(error: usize) -> isize;
    fn trace_sbi_call(ext: ::core::ffi::c_int, fid: ::core::ffi::c_int);
    fn trace_sbi_return(ext: ::core::ffi::c_int, error: usize, value: usize);
}

// EXPORT_SYMBOL(__sbi_base_ecall);
pub unsafe fn __sbi_base_ecall(fid: ::core::ffi::c_int) -> isize {
    let ret = sbi_ecall(SBI_EXT_BASE, fid, 0, 0, 0, 0, 0, 0);
    if ret.error == 0 {
        ret.value as isize
    } else {
        sbi_err_map_linux_errno(ret.error)
    }
}

// EXPORT_SYMBOL(__sbi_ecall);
pub unsafe fn __sbi_ecall(
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    fid: ::core::ffi::c_int,
    ext: ::core::ffi::c_int,
) -> sbiret {
    trace_sbi_call(ext, fid);

    let mut a0 = arg0 as usize;
    let mut a1 = arg1 as usize;
    let a2 = arg2 as usize;
    let a3 = arg3 as usize;
    let a4 = arg4 as usize;
    let a5 = arg5 as usize;
    let a6 = fid as usize;
    let a7 = ext as usize;

    asm!(
        "ecall",
        inlateout("a0") a0,
        inlateout("a1") a1,
        in("a2") a2,
        in("a3") a3,
        in("a4") a4,
        in("a5") a5,
        in("a6") a6,
        in("a7") a7,
        options(nostack)
    );

    let ret = sbiret {
        error: a0,
        value: a1,
    };

    trace_sbi_return(ext, ret.error, ret.value);

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
