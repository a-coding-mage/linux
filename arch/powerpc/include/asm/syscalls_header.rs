/* SPDX-License-Identifier: GPL-2.0 */

// Original header guard: __ASM_POWERPC_SYSCALLS_H
// The declarations below are kernel-only and depend on Linux/PPC definitions
// supplied by the surrounding translation unit.

// Original includes:
// linux/compiler.h, linux/linkage.h, linux/types.h, linux/compat.h,
// asm/syscall.h, asm/syscalls_32.h (CONFIG_PPC64), asm/unistd.h,
// asm/ucontext.h

#[repr(C)]
pub struct rtas_args {
    _private: [u8; 0],
}

// long long munging: the 32-bit ABI passes long longs in an odd/even
// register pair. High and low parts are swapped depending on endian mode.
#[inline(always)]
pub const fn merge_64(high: u32, low: u32) -> u64 {
    ((high as u64) << 32) | (low as u64)
}

// CONFIG_ARCH_HAS_SYSCALL_WRAPPER selects the signature of sys_ni_syscall.
#[cfg(not(feature = "CONFIG_ARCH_HAS_SYSCALL_WRAPPER"))]
extern "C" {
    pub fn sys_ni_syscall() -> c_long;
}

#[cfg(feature = "CONFIG_ARCH_HAS_SYSCALL_WRAPPER")]
extern "C" {
    pub fn sys_ni_syscall(regs: *const pt_regs) -> c_long;
}

// Architecture-specific syscalls.
extern "C" {
    pub fn sys_rtas(uargs: *mut rtas_args) -> c_long;

    // CONFIG_PPC64
    pub fn sys_ppc64_personality(personality: c_ulong) -> c_long;
    // CONFIG_COMPAT
    pub fn compat_sys_ppc64_personality(personality: c_ulong) -> c_long;

    pub fn sys_swapcontext(
        old_ctx: *mut ucontext,
        new_ctx: *mut ucontext,
        ctx_size: c_long,
    ) -> c_long;
    pub fn sys_mmap(
        addr: c_ulong,
        len: usize,
        prot: c_ulong,
        flags: c_ulong,
        fd: c_ulong,
        offset: off_t,
    ) -> c_long;
    pub fn sys_mmap2(
        addr: c_ulong,
        len: usize,
        prot: c_ulong,
        flags: c_ulong,
        fd: c_ulong,
        pgoff: c_ulong,
    ) -> c_long;
    pub fn sys_switch_endian() -> c_long;

    // CONFIG_PPC32
    pub fn sys_sigreturn() -> c_long;
    pub fn sys_debug_setcontext(ctx: *mut ucontext, ndbg: c_int, dbg: *mut sig_dbg_op) -> c_long;

    pub fn sys_rt_sigreturn() -> c_long;
    pub fn sys_subpage_prot(addr: c_ulong, len: c_ulong, map: *mut u32) -> c_long;

    // CONFIG_COMPAT
    pub fn compat_sys_swapcontext(
        old_ctx: *mut ucontext32,
        new_ctx: *mut ucontext32,
        ctx_size: c_int,
    ) -> c_long;
    pub fn compat_sys_old_getrlimit(resource: c_uint, rlim: *mut compat_rlimit) -> c_long;
    pub fn compat_sys_sigreturn() -> c_long;
    pub fn compat_sys_rt_sigreturn() -> c_long;

    // CONFIG_PPC32: architecture-specific signatures required by long long munging.
    pub fn sys_ppc_pread64(fd: c_uint, ubuf: *mut c_char, count: compat_size_t,
                           reg6: u32, pos1: u32, pos2: u32) -> c_long;
    pub fn sys_ppc_pwrite64(fd: c_uint, ubuf: *const c_char, count: compat_size_t,
                            reg6: u32, pos1: u32, pos2: u32) -> c_long;
    pub fn sys_ppc_readahead(fd: c_int, r4: u32, offset1: u32, offset2: u32, count: u32) -> c_long;
    pub fn sys_ppc_truncate64(path: *const c_char, reg4: u32, len1: c_ulong, len2: c_ulong) -> c_long;
    pub fn sys_ppc_ftruncate64(fd: c_uint, reg4: u32, len1: c_ulong, len2: c_ulong) -> c_long;
    pub fn sys_ppc32_fadvise64(fd: c_int, unused: u32, offset1: u32, offset2: u32,
                               len: usize, advice: c_int) -> c_long;
    pub fn sys_ppc_sync_file_range2(fd: c_int, flags: c_uint, offset1: c_uint,
                                    offset2: c_uint, nbytes1: c_uint, nbytes2: c_uint) -> c_long;
    pub fn sys_ppc_fallocate(fd: c_int, mode: c_int, offset1: u32, offset2: u32,
                             len1: u32, len2: u32) -> c_long;

    // CONFIG_COMPAT
    pub fn compat_sys_mmap2(addr: c_ulong, len: usize, prot: c_ulong, flags: c_ulong,
                            fd: c_ulong, pgoff: c_ulong) -> c_long;
    pub fn compat_sys_ppc_pread64(fd: c_uint, ubuf: *mut c_char, count: compat_size_t,
                                  reg6: u32, pos1: u32, pos2: u32) -> c_long;
    pub fn compat_sys_ppc_pwrite64(fd: c_uint, ubuf: *const c_char, count: compat_size_t,
                                   reg6: u32, pos1: u32, pos2: u32) -> c_long;
    pub fn compat_sys_ppc_readahead(fd: c_int, r4: u32, offset1: u32, offset2: u32, count: u32) -> c_long;
    pub fn compat_sys_ppc_truncate64(path: *const c_char, reg4: u32, len1: c_ulong, len2: c_ulong) -> c_long;
    pub fn compat_sys_ppc_ftruncate64(fd: c_uint, reg4: u32, len1: c_ulong, len2: c_ulong) -> c_long;
    pub fn compat_sys_ppc32_fadvise64(fd: c_int, unused: u32, offset1: u32, offset2: u32,
                                      len: usize, advice: c_int) -> c_long;
    pub fn compat_sys_ppc_sync_file_range2(fd: c_int, flags: c_uint, offset1: c_uint,
                                           offset2: c_uint, nbytes1: c_uint, nbytes2: c_uint) -> c_long;

    // CONFIG_PPC32 || CONFIG_COMPAT
    pub fn sys_ppc_fadvise64_64(fd: c_int, advice: c_int, offset_high: u32, offset_low: u32,
                                len_high: u32, len_low: u32) -> c_long;
}

// When CONFIG_ARCH_HAS_SYSCALL_WRAPPER is enabled, syscall-table headers
// provide declarations of the form: long entry(const struct pt_regs *regs).
// __SYSCALL_WITH_COMPAT(nr, native, compat) expands to __SYSCALL(nr, native).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
