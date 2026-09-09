/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: __u32, __u64, and the Linux ioctl encoding macros are supplied
// by the surrounding bindings/environment.

/*
 * Argument for KCOV_REMOTE_ENABLE ioctl, see Documentation/dev-tools/kcov.rst
 * and the comment before kcov_remote_start() for usage details.
 */
#[repr(C)]
pub struct kcov_remote_arg {
    pub trace_mode: u32,    /* KCOV_TRACE_PC or KCOV_TRACE_CMP */
    pub area_size: u32,     /* Length of coverage buffer in words */
    pub num_handles: u32,   /* Size of handles array */
    pub common_handle: u64,
    pub handles: [u64; 0],
}

pub const KCOV_REMOTE_MAX_HANDLES: u32 = 0x100;

// These ioctl values depend on the platform's Linux _IOC/_IOR/_IOW encoding.
#[allow(unused_macros)]
macro_rules! KCOV_INIT_TRACE {
    ($ty:ty) => { _IOR(b'c', 1, $ty) };
}
#[allow(unused_macros)]
macro_rules! KCOV_ENABLE {
    () => { _IO(b'c', 100) };
}
#[allow(unused_macros)]
macro_rules! KCOV_DISABLE {
    () => { _IO(b'c', 101) };
}
#[allow(unused_macros)]
macro_rules! KCOV_REMOTE_ENABLE {
    ($ty:ty) => { _IOW(b'c', 102, $ty) };
}

/*
 * Tracing coverage collection mode.
 * Covered PCs are collected in a per-task buffer.
 * In new KCOV version the mode is chosen by calling
 * ioctl(fd, KCOV_ENABLE, mode). In older versions the mode argument
 * was supposed to be 0 in such a call. So, for reasons of backward
 * compatibility, we have chosen the value KCOV_TRACE_PC to be 0.
 */
pub const KCOV_TRACE_PC: u32 = 0;
/* Collecting comparison operands mode. */
pub const KCOV_TRACE_CMP: u32 = 1;

/*
 * The format for the types of collected comparisons.
 *
 * Bit 0 shows whether one of the arguments is a compile-time constant.
 * Bits 1 & 2 contain log2 of the argument size, up to 8 bytes.
 */
#[inline]
pub const fn KCOV_CMP_CONST() -> u32 {
    1 << 0
}

#[inline]
pub const fn KCOV_CMP_SIZE(n: u32) -> u32 {
    n << 1
}

#[inline]
pub const fn KCOV_CMP_MASK() -> u32 {
    KCOV_CMP_SIZE(3)
}

pub const KCOV_SUBSYSTEM_COMMON: u64 = 0x00u64 << 56;
pub const KCOV_SUBSYSTEM_USB: u64 = 0x01u64 << 56;

pub const KCOV_SUBSYSTEM_MASK: u64 = 0xffu64 << 56;
pub const KCOV_INSTANCE_MASK: u64 = 0xffffffffu64;

#[inline]
pub const unsafe fn kcov_remote_handle(subsys: u64, inst: u64) -> u64 {
    if (subsys & !KCOV_SUBSYSTEM_MASK) != 0 || (inst & !KCOV_INSTANCE_MASK) != 0 {
        return 0;
    }
    subsys | inst
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
