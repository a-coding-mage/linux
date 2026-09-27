/* SPDX-License-Identifier: GPL-2.0 */

// `asmlinkage` denotes the platform's syscall calling convention in C.
// The declarations below preserve the external C ABI; architecture-specific
// calling-convention details remain supplied by the target build.
extern "C" {
    pub fn old_mmap(
        addr: kernel::ffi::c_ulong,
        len: kernel::ffi::c_ulong,
        prot: kernel::ffi::c_ulong,
        flags: kernel::ffi::c_ulong,
        fd: kernel::ffi::c_int,
        off: kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_int;

    pub fn sys_mmap2(
        addr: kernel::ffi::c_ulong,
        len: kernel::ffi::c_ulong,
        prot: kernel::ffi::c_ulong,
        flags: kernel::ffi::c_ulong,
        fd: kernel::ffi::c_ulong,
        pgoff: kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_long;

    pub fn sys_cacheflush(
        addr: kernel::ffi::c_ulong,
        len: kernel::ffi::c_ulong,
        op: kernel::ffi::c_int,
    ) -> kernel::ffi::c_int;
}

// C dependency: <asm/syscalls_32.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
