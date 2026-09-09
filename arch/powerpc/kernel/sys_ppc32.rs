// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sys_ppc32.c: 32-bit system calls with complex calling conventions.
 *
 * 32-bit system calls with 64-bit arguments pass those in register pairs.
 * This must be specially dealt with on 64-bit kernels. The compat_arg_u64_dual
 * in generic compat syscalls is not always usable because the register
 * pairing is constrained depending on preceding arguments.
 *
 * An analogous problem exists on 32-bit kernels with ARCH_HAS_SYSCALL_WRAPPER,
 * the defined system call functions take the pt_regs as an argument, and there
 * is a mapping macro which maps registers to arguments
 * (SC_POWERPC_REGS_TO_ARGS) which also does not deal with these 64-bit
 * arguments.
 *
 * This file contains these system calls.
 */

// C headers and syscall-definition macros are supplied by the surrounding
// kernel translation. CONFIG_PPC32 selects native versus compat syscall ABI.

extern "C" {
    fn merge_64(high: u32, low: u32) -> u64;
    fn ksys_pread64(fd: u32, ubuf: *mut i8, count: compat_size_t, pos: u64) -> isize;
    fn ksys_pwrite64(fd: u32, ubuf: *const i8, count: compat_size_t, pos: u64) -> isize;
    fn ksys_readahead(fd: i32, offset: u64, count: u32) -> isize;
    fn ksys_truncate(path: *const i8, length: u64) -> isize;
    fn ksys_ftruncate(fd: u32, length: u64, flags: u32) -> isize;
    fn ksys_fadvise64_64(fd: i32, offset: u64, len: usize, advice: i32) -> isize;
    fn ksys_sync_file_range(fd: i32, offset: i64, nbytes: i64, flags: u32) -> isize;
    fn ksys_fallocate(fd: i32, mode: i32, offset: u64, len: u64) -> isize;
}

// PPC32_SYSCALL_DEFINE* expands to SYSCALL_DEFINE* for CONFIG_PPC32 and to
// COMPAT_SYSCALL_DEFINE* otherwise. The functions below retain the resulting
// syscall entry points and argument layout.

pub unsafe fn ppc_pread64(
    fd: u32,
    ubuf: *mut i8,
    count: compat_size_t,
    _reg6: u32,
    pos1: u32,
    pos2: u32,
) -> isize {
    ksys_pread64(fd, ubuf, count, merge_64(pos1, pos2))
}

pub unsafe fn ppc_pwrite64(
    fd: u32,
    ubuf: *const i8,
    count: compat_size_t,
    _reg6: u32,
    pos1: u32,
    pos2: u32,
) -> isize {
    ksys_pwrite64(fd, ubuf, count, merge_64(pos1, pos2))
}

pub unsafe fn ppc_readahead(
    fd: i32,
    _r4: u32,
    offset1: u32,
    offset2: u32,
    count: u32,
) -> isize {
    ksys_readahead(fd, merge_64(offset1, offset2), count)
}

pub unsafe fn ppc_truncate64(
    path: *const i8,
    _reg4: u32,
    len1: usize,
    len2: usize,
) -> isize {
    ksys_truncate(path, merge_64(len1 as u32, len2 as u32))
}

pub unsafe fn ppc_ftruncate64(
    fd: u32,
    _reg4: u32,
    len1: usize,
    len2: usize,
) -> isize {
    ksys_ftruncate(fd, merge_64(len1 as u32, len2 as u32), FTRUNCATE_LFS)
}

pub unsafe fn ppc32_fadvise64(
    fd: i32,
    _unused: u32,
    offset1: u32,
    offset2: u32,
    len: usize,
    advice: i32,
) -> isize {
    ksys_fadvise64_64(fd, merge_64(offset1, offset2), len, advice)
}

pub unsafe fn ppc_sync_file_range2(
    fd: i32,
    flags: u32,
    offset1: u32,
    offset2: u32,
    nbytes1: u32,
    nbytes2: u32,
) -> isize {
    let offset: i64 = merge_64(offset1, offset2) as i64;
    let nbytes: i64 = merge_64(nbytes1, nbytes2) as i64;

    ksys_sync_file_range(fd, offset, nbytes, flags)
}

// Present only when CONFIG_PPC32 is enabled.
#[cfg(CONFIG_PPC32)]
pub unsafe fn ppc_fallocate(
    fd: i32,
    mode: i32,
    offset1: u32,
    offset2: u32,
    len1: u32,
    len2: u32,
) -> isize {
    ksys_fallocate(
        fd,
        mode,
        merge_64(offset1, offset2),
        merge_64(len1, len2),
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
