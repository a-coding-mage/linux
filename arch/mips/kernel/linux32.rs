// SPDX-License-Identifier: GPL-2.0
/*
 * Conversion between 32-bit and 64-bit native system calls.
 *
 * Copyright (C) 2000 Silicon Graphics, Inc.
 * Written by Ulf Carlsson (ulfc@engr.sgi.com)
 */

// Kernel headers supplied by the surrounding translation unit provide these
// types, constants, globals, and external functions.

#[cfg(target_endian = "big")]
#[inline]
unsafe fn merge_64(r1: c_ulong, r2: c_ulong) -> u64 {
    ((r1 & 0xffff_ffff) << 32).wrapping_add(r2 & 0xffff_ffff)
}

#[cfg(target_endian = "little")]
#[inline]
unsafe fn merge_64(r1: c_ulong, r2: c_ulong) -> u64 {
    ((r2 & 0xffff_ffff) << 32).wrapping_add(r1 & 0xffff_ffff)
}

// SYSCALL_DEFINE4(32_truncate64, ...)
pub unsafe fn sys_32_truncate64(
    path: *const c_char,
    _dummy: c_ulong,
    a2: c_ulong,
    a3: c_ulong,
) -> c_long {
    ksys_truncate(path, merge_64(a2, a3))
}

// SYSCALL_DEFINE4(32_ftruncate64, ...)
pub unsafe fn sys_32_ftruncate64(
    fd: c_ulong,
    _dummy: c_ulong,
    a2: c_ulong,
    a3: c_ulong,
) -> c_long {
    ksys_ftruncate(fd, merge_64(a2, a3), FTRUNCATE_LFS)
}

// SYSCALL_DEFINE5(32_llseek, ...)
pub unsafe fn sys_32_llseek(
    fd: c_uint,
    offset_high: c_uint,
    offset_low: c_uint,
    result: *mut loff_t,
    origin: c_uint,
) -> c_long {
    sys_llseek(fd, offset_high, offset_low, result, origin)
}

/* From the Single Unix Spec: pread & pwrite act like lseek to pos + op +
   lseek back to original location.  They fail just like lseek does on
   non-seekable files. */
pub unsafe fn sys_32_pread(
    fd: c_ulong,
    buf: *mut c_char,
    count: usize,
    _unused: c_ulong,
    a4: c_ulong,
    a5: c_ulong,
) -> isize {
    ksys_pread64(fd, buf, count, merge_64(a4, a5))
}

pub unsafe fn sys_32_pwrite(
    fd: c_uint,
    buf: *const c_char,
    count: usize,
    _unused: u32,
    a4: u64,
    a5: u64,
) -> isize {
    ksys_pwrite64(fd, buf, count, merge_64(a4 as c_ulong, a5 as c_ulong))
}

pub unsafe fn sys_32_personality(personality_arg: c_ulong) -> c_int {
    let mut p: c_uint = (personality_arg & 0xffff_ffff) as c_uint;
    let ret: c_int;

    if personality(current_personality()) == PER_LINUX32
        && personality(p as c_ulong) == PER_LINUX
    {
        p = (p & !PER_MASK) | PER_LINUX32;
    }
    ret = sys_personality(p as c_ulong);
    if ret != -1 && personality(ret as c_ulong) == PER_LINUX32 {
        return ((ret as c_ulong & !PER_MASK) | PER_LINUX) as c_int;
    }
    ret
}

pub unsafe fn sys32_readahead(
    fd: c_int,
    _pad0: u32,
    a2: u64,
    a3: u64,
    count: usize,
) -> isize {
    ksys_readahead(fd, merge_64(a2 as c_ulong, a3 as c_ulong), count)
}

pub unsafe fn sys32_sync_file_range(
    fd: c_int,
    _pad: c_int,
    a2: c_ulong,
    a3: c_ulong,
    a4: c_ulong,
    a5: c_ulong,
    flags: c_int,
) -> c_long {
    ksys_sync_file_range(fd, merge_64(a2, a3), merge_64(a4, a5), flags)
}

pub unsafe fn sys32_fadvise64_64(
    fd: c_int,
    _pad: c_int,
    a2: c_ulong,
    a3: c_ulong,
    a4: c_ulong,
    a5: c_ulong,
    flags: c_int,
) -> c_long {
    ksys_fadvise64_64(fd, merge_64(a2, a3), merge_64(a4, a5), flags)
}

pub unsafe fn sys32_fallocate(
    fd: c_int,
    mode: c_int,
    offset_a2: c_uint,
    offset_a3: c_uint,
    len_a4: c_uint,
    len_a5: c_uint,
) -> c_long {
    ksys_fallocate(
        fd,
        mode,
        merge_64(offset_a2 as c_ulong, offset_a3 as c_ulong),
        merge_64(len_a4 as c_ulong, len_a5 as c_ulong),
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
