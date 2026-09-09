/*
 * Copyright 2003 PathScale, Inc.
 * Copyright (C) 2003 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 *
 * Licensed under the GPL
 */

/* Translated from the C header __SYSDEP_X86_64_PTRACE_H. */

macro_rules! REGS_R8 {
    ($r:expr) => { ($r)[HOST_R8] };
}
macro_rules! REGS_R9 {
    ($r:expr) => { ($r)[HOST_R9] };
}
macro_rules! REGS_R10 {
    ($r:expr) => { ($r)[HOST_R10] };
}
macro_rules! REGS_R11 {
    ($r:expr) => { ($r)[HOST_R11] };
}
macro_rules! REGS_R12 {
    ($r:expr) => { ($r)[HOST_R12] };
}
macro_rules! REGS_R13 {
    ($r:expr) => { ($r)[HOST_R13] };
}
macro_rules! REGS_R14 {
    ($r:expr) => { ($r)[HOST_R14] };
}
macro_rules! REGS_R15 {
    ($r:expr) => { ($r)[HOST_R15] };
}

pub const HOST_FS_BASE: usize = 21;
pub const HOST_GS_BASE: usize = 22;
pub const HOST_DS: usize = 23;
pub const HOST_ES: usize = 24;
pub const HOST_FS: usize = 25;
pub const HOST_GS: usize = 26;

/* Also defined in asm/ptrace-x86_64.h, but not in libc headers. */
/* These are conditionally defined in C when FS_BASE is absent. */
#[cfg(not(any(FS_BASE_DEFINED)))]
pub const FS_BASE: usize = HOST_FS_BASE * core::mem::size_of::<core::ffi::c_long>();
#[cfg(not(any(FS_BASE_DEFINED)))]
pub const GS_BASE: usize = HOST_GS_BASE * core::mem::size_of::<core::ffi::c_long>();
#[cfg(not(any(FS_BASE_DEFINED)))]
pub const DS: usize = HOST_DS * core::mem::size_of::<core::ffi::c_long>();
#[cfg(not(any(FS_BASE_DEFINED)))]
pub const ES: usize = HOST_ES * core::mem::size_of::<core::ffi::c_long>();
#[cfg(not(any(FS_BASE_DEFINED)))]
pub const FS: usize = HOST_FS * core::mem::size_of::<core::ffi::c_long>();
#[cfg(not(any(FS_BASE_DEFINED)))]
pub const GS: usize = HOST_GS * core::mem::size_of::<core::ffi::c_long>();

macro_rules! UPT_R8 {
    ($r:expr) => { REGS_R8!(($r).gp) };
}
macro_rules! UPT_R9 {
    ($r:expr) => { REGS_R9!(($r).gp) };
}
macro_rules! UPT_R10 {
    ($r:expr) => { REGS_R10!(($r).gp) };
}
macro_rules! UPT_R11 {
    ($r:expr) => { REGS_R11!(($r).gp) };
}
macro_rules! UPT_R12 {
    ($r:expr) => { REGS_R12!(($r).gp) };
}
macro_rules! UPT_R13 {
    ($r:expr) => { REGS_R13!(($r).gp) };
}
macro_rules! UPT_R14 {
    ($r:expr) => { REGS_R14!(($r).gp) };
}
macro_rules! UPT_R15 {
    ($r:expr) => { REGS_R15!(($r).gp) };
}

macro_rules! UPT_SYSCALL_ARG1 {
    ($r:expr) => { UPT_DI!($r) };
}
macro_rules! UPT_SYSCALL_ARG2 {
    ($r:expr) => { UPT_SI!($r) };
}
macro_rules! UPT_SYSCALL_ARG3 {
    ($r:expr) => { UPT_DX!($r) };
}
macro_rules! UPT_SYSCALL_ARG4 {
    ($r:expr) => { UPT_R10!($r) };
}
macro_rules! UPT_SYSCALL_ARG5 {
    ($r:expr) => { UPT_R8!($r) };
}
macro_rules! UPT_SYSCALL_ARG6 {
    ($r:expr) => { UPT_R9!($r) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
