/* SPDX-License-Identifier: GPL-2.0 */
/*
 * vdso_config.h: Configuration options for vDSO tests.
 * Copyright (c) 2019 Arm Ltd.
 */

use core::ffi::c_char;

/*
 * Each architecture exports its vDSO implementation with different names
 * and a different version from the others, so we need to handle it as a
 * special case.
 */
#[cfg(target_arch = "arm")]
pub const VDSO_VERSION: i32 = 0;
#[cfg(target_arch = "arm")]
pub const VDSO_NAMES: i32 = 1;
#[cfg(target_arch = "arm")]
pub const VDSO_32BIT: i32 = 1;

#[cfg(target_arch = "aarch64")]
pub const VDSO_VERSION: i32 = 3;
#[cfg(target_arch = "aarch64")]
pub const VDSO_NAMES: i32 = 0;

#[cfg(target_arch = "powerpc64")]
pub const VDSO_VERSION: i32 = 1;
#[cfg(target_arch = "powerpc64")]
pub const VDSO_NAMES: i32 = 0;

#[cfg(target_arch = "powerpc")]
pub const VDSO_VERSION: i32 = 1;
#[cfg(target_arch = "powerpc")]
pub const VDSO_NAMES: i32 = 0;
#[cfg(target_arch = "powerpc")]
pub const VDSO_32BIT: i32 = 1;

#[cfg(target_arch = "s390x")]
pub const VDSO_VERSION: i32 = 2;
#[cfg(target_arch = "s390x")]
pub const VDSO_NAMES: i32 = 0;

#[cfg(target_arch = "mips")]
pub const VDSO_VERSION: i32 = 0;
#[cfg(target_arch = "mips")]
pub const VDSO_NAMES: i32 = 1;
#[cfg(target_arch = "mips")]
pub const VDSO_32BIT: i32 = 1;

#[cfg(any(target_arch = "sparc", target_arch = "sparc64"))]
pub const VDSO_VERSION: i32 = 0;
#[cfg(any(target_arch = "sparc", target_arch = "sparc64"))]
pub const VDSO_NAMES: i32 = 1;
#[cfg(any(target_arch = "sparc", target_arch = "sparc64"))]
pub const VDSO_32BIT: i32 = 1;

#[cfg(target_arch = "x86")]
pub const VDSO_VERSION: i32 = 0;
#[cfg(target_arch = "x86")]
pub const VDSO_NAMES: i32 = 1;
#[cfg(target_arch = "x86")]
pub const VDSO_32BIT: i32 = 1;

#[cfg(target_arch = "x86_64")]
pub const VDSO_VERSION: i32 = 0;
#[cfg(target_arch = "x86_64")]
pub const VDSO_NAMES: i32 = 1;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub const VDSO_VERSION: i32 = 5;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub const VDSO_NAMES: i32 = 1;
#[cfg(target_arch = "riscv32")]
pub const VDSO_32BIT: i32 = 1;

#[cfg(any(target_arch = "loongarch32", target_arch = "loongarch64"))]
pub const VDSO_VERSION: i32 = 6;
#[cfg(any(target_arch = "loongarch32", target_arch = "loongarch64"))]
pub const VDSO_NAMES: i32 = 1;

#[allow(dead_code)]
pub const versions: [*const c_char; 7] = [
    b"LINUX_2.6\0".as_ptr() as *const c_char,
    b"LINUX_2.6.15\0".as_ptr() as *const c_char,
    b"LINUX_2.6.29\0".as_ptr() as *const c_char,
    b"LINUX_2.6.39\0".as_ptr() as *const c_char,
    b"LINUX_4\0".as_ptr() as *const c_char,
    b"LINUX_4.15\0".as_ptr() as *const c_char,
    b"LINUX_5.10\0".as_ptr() as *const c_char,
];

#[allow(dead_code)]
pub const names: [[*const c_char; 8]; 2] = [
    [
        b"__kernel_gettimeofday\0".as_ptr() as *const c_char,
        b"__kernel_clock_gettime\0".as_ptr() as *const c_char,
        b"__kernel_time\0".as_ptr() as *const c_char,
        b"__kernel_clock_getres\0".as_ptr() as *const c_char,
        b"__kernel_getcpu\0".as_ptr() as *const c_char,
        b"__kernel_clock_gettime64\0".as_ptr() as *const c_char,
        b"__kernel_getrandom\0".as_ptr() as *const c_char,
        b"__kernel_clock_getres_time64\0".as_ptr() as *const c_char,
    ],
    [
        b"__vdso_gettimeofday\0".as_ptr() as *const c_char,
        b"__vdso_clock_gettime\0".as_ptr() as *const c_char,
        b"__vdso_time\0".as_ptr() as *const c_char,
        b"__vdso_clock_getres\0".as_ptr() as *const c_char,
        b"__vdso_getcpu\0".as_ptr() as *const c_char,
        b"__vdso_clock_gettime64\0".as_ptr() as *const c_char,
        b"__vdso_getrandom\0".as_ptr() as *const c_char,
        b"__vdso_clock_getres_time64\0".as_ptr() as *const c_char,
    ],
];
