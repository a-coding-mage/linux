// SPDX-License-Identifier: GPL-2.0-only
/*
 * vdso_test_getcpu.c: Sample code to test parse_vdso.c and vDSO getcpu()
 *
 * Copyright (c) 2020 Arm Ltd
 */

// C includes translated as external dependency intent:
// <stdint.h>, <elf.h>, <stdio.h>, <sys/auxv.h>, <sys/time.h>
// "kselftest.h", "parse_vdso.h", "vdso_config.h", "vdso_call.h"

use core::ffi::{c_char, c_uint, c_ulong, c_void};

type GetcpuT = unsafe extern "C" fn(*mut c_uint, *mut c_uint, *mut c_void) -> i64;

unsafe extern "C" {
    static versions: *const *const c_char;
    static names: *const *const c_char;

    fn getauxval(type_: c_ulong) -> c_ulong;
    fn vdso_init_from_sysinfo_ehdr(sysinfo_ehdr: c_ulong);
    fn vdso_sym(version: *const c_char, name: *const c_char) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> i32;
}

// Constants supplied by included headers in the original C source.
const AT_SYSINFO_EHDR: c_ulong = /* TODO: from <sys/auxv.h> */ 33;
const KSFT_SKIP: i32 = /* TODO: from "kselftest.h" */ 4;
const KSFT_FAIL: i32 = /* TODO: from "kselftest.h" */ 1;
const VDSO_VERSION: usize = /* TODO: from "vdso_config.h" */ 0;
const VDSO_NAMES: usize = /* TODO: from "vdso_config.h" */ 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: i32, _argv: *mut *mut c_char) -> i32 {
    let version: *const c_char = unsafe { *versions.add(VDSO_VERSION) };
    let name: *const *const c_char = unsafe { names.add(VDSO_NAMES) };
    let mut sysinfo_ehdr: c_ulong;
    let mut cpu: c_uint = 0;
    let mut node: c_uint = 0;
    let get_cpu: GetcpuT;
    let ret: i64;

    sysinfo_ehdr = unsafe { getauxval(AT_SYSINFO_EHDR) };
    if sysinfo_ehdr == 0 {
        unsafe {
            printf(c"AT_SYSINFO_EHDR is not present!\n".as_ptr());
        }
        return KSFT_SKIP;
    }

    unsafe {
        vdso_init_from_sysinfo_ehdr(getauxval(AT_SYSINFO_EHDR));
    }

    get_cpu = unsafe { core::mem::transmute::<*mut c_void, GetcpuT>(vdso_sym(version, *name.add(4))) };
    if (get_cpu as *const c_void).is_null() {
        unsafe {
            printf(c"Could not find %s\n".as_ptr(), *name.add(4));
        }
        return KSFT_SKIP;
    }

    // VDSO_CALL(get_cpu, 3, &cpu, &node, 0)
    ret = unsafe { get_cpu(&mut cpu, &mut node, core::ptr::null_mut()) };
    if ret == 0 {
        unsafe {
            printf(c"Running on CPU %u node %u\n".as_ptr(), cpu, node);
        }
    } else {
        unsafe {
            printf(c"%s failed\n".as_ptr(), *name.add(4));
        }
        return KSFT_FAIL;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
