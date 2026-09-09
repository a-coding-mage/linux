/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
 * Copyright (C) 2005 Jeff Dike (jdike@karaya.com)
 */

/* Dependencies supplied by the surrounding translation unit are intentionally
 * left external, as in the original header. */

pub const FUTEX_IN_CHILD: i32 = 0;
pub const FUTEX_IN_KERN: i32 = 1;

#[repr(C)]
pub struct StubInitData {
    pub seccomp: i32,
    pub stub_start: ::core::ffi::c_ulong,
    pub stub_code_fd: i32,
    pub stub_code_offset: ::core::ffi::c_ulong,
    pub stub_data_fd: i32,
    pub stub_data_offset: ::core::ffi::c_ulong,
    pub signal_handler: ::core::ffi::c_ulong,
    pub signal_restorer: ::core::ffi::c_ulong,
}

/* STUB_NEXT_SYSCALL(s): advance by the command length of the current syscall. */
#[macro_export]
macro_rules! STUB_NEXT_SYSCALL {
    ($s:expr) => {
        (($s as *mut StubSyscall as *mut u8)
            .wrapping_add(($s).cmd_len as usize)) as *mut StubSyscall
    };
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum StubSyscallType {
    StubSyscallUnset = 0,
    StubSyscallMmap,
    StubSyscallMunmap,
}

#[repr(C)]
pub struct StubSyscallMem {
    pub addr: ::core::ffi::c_ulong,
    pub length: ::core::ffi::c_ulong,
    pub offset: ::core::ffi::c_ulong,
    pub fd: i32,
    pub prot: i32,
}

#[repr(C)]
pub struct StubSyscall {
    pub mem: StubSyscallMem,
    pub syscall: StubSyscallType,
}

#[repr(C, align(16))]
pub struct StubSyscallDataArray<const N: usize>(pub [StubSyscall; N]);

#[repr(C, align(UM_KERN_PAGE_SIZE))]
pub struct StubSignalStack(pub [u8; UM_KERN_PAGE_SIZE]);

#[repr(C)]
pub struct StubData {
    pub err: ::core::ffi::c_long,
    pub syscall_data_len: i32,
    /* 128 leaves enough room for additional fields in the struct */
    pub syscall_data:
        StubSyscallDataArray<{ (UM_KERN_PAGE_SIZE - 128) / ::core::mem::size_of::<StubSyscall>() }>,

    /* data shared with signal handler (only used in seccomp mode) */
    pub restart_wait: i16,
    pub futex: u32,
    pub signal: i32,
    pub si_offset: u16,
    pub mctx_offset: u16,

    /* seccomp architecture specific state restore */
    pub arch_data: StubDataArch,

    /* Stack for our signal handlers and for calling into . */
    pub sigstack: StubSignalStack,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
