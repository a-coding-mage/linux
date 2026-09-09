/* SPDX-License-Identifier: GPL-2.0 */

// `asmlinkage` denotes the platform's syscall calling convention in C.
// The declarations below preserve the external C ABI; architecture-specific
// calling-convention details remain supplied by the target build.
extern "C" {
    pub fn old_mmap(
        addr: core::ffi::c_ulong,
        len: core::ffi::c_ulong,
        prot: core::ffi::c_ulong,
        flags: core::ffi::c_ulong,
        fd: core::ffi::c_int,
        off: core::ffi::c_ulong,
    ) -> core::ffi::c_int;

    pub fn sys_mmap2(
        addr: core::ffi::c_ulong,
        len: core::ffi::c_ulong,
        prot: core::ffi::c_ulong,
        flags: core::ffi::c_ulong,
        fd: core::ffi::c_ulong,
        pgoff: core::ffi::c_ulong,
    ) -> core::ffi::c_long;

    pub fn sys_cacheflush(
        addr: core::ffi::c_ulong,
        len: core::ffi::c_ulong,
        op: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

// C dependency: <asm/syscalls_32.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
