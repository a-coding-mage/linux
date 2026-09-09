// SPDX-License-Identifier: GPL-2.0
/*
 * Naive system call dropper built on seccomp_filter.
 *
 * Copyright (c) 2012 The Chromium OS Authors <chromium-os-dev@chromium.org>
 * Author: Will Drewry <wad@chromium.org>
 *
 * The code may be used by anyone for any purpose,
 * and can serve as a starting point for developing
 * applications using prctl(PR_SET_SECCOMP, 2, ...).
 *
 * When run, returns the specified errno for the specified
 * system call number against the given architecture.
 *
 */

use std::ffi::c_char;
use std::os::raw::{c_int, c_ulong};

#[repr(C)]
struct SockFilter {
    code: u16,
    jt: u8,
    jf: u8,
    k: u32,
}

#[repr(C)]
struct SockFprog {
    len: u16,
    filter: *mut SockFilter,
}

const BPF_LD: u16 = 0x00;
const BPF_W: u16 = 0x00;
const BPF_ABS: u16 = 0x20;
const BPF_JMP: u16 = 0x05;
const BPF_JEQ: u16 = 0x10;
const BPF_K: u16 = 0x00;
const BPF_RET: u16 = 0x06;
const SECCOMP_RET_KILL: u32 = 0x00000000;
const SECCOMP_RET_ALLOW: u32 = 0x7fff0000;
const SECCOMP_RET_ERRNO: u32 = 0x00050000;
const SECCOMP_RET_DATA: u32 = 0x0000ffff;
const PR_SET_NO_NEW_PRIVS: c_int = 38;
const PR_SET_SECCOMP: c_int = 22;
const AUDIT_ARCH_I386: c_ulong = 0x40000003;
const AUDIT_ARCH_X86_64: c_ulong = 0xc000003e;

extern "C" {
    fn prctl(option: c_int, ...) -> c_int;
    fn perror(s: *const c_char);
    static mut stderr: *mut core::ffi::c_void;
    fn fprintf(stream: *mut core::ffi::c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> isize;
    fn execv(path: *const c_char, argv: *const *mut c_char) -> c_int;
}

const fn bpf_stmt(code: u16, k: u32) -> SockFilter {
    SockFilter { code, jt: 0, jf: 0, k }
}

const fn bpf_jump(code: u16, k: u32, jt: u8, jf: u8) -> SockFilter {
    SockFilter { code, jt, jf, k }
}

unsafe fn install_filter(arch: c_int, nr: c_int, error: c_int) -> c_int {
    let mut filter = [
        bpf_stmt(BPF_LD | BPF_W | BPF_ABS, 4),
        bpf_jump(BPF_JMP | BPF_JEQ | BPF_K, arch as u32, 0, 3),
        bpf_stmt(BPF_LD | BPF_W | BPF_ABS, 0),
        bpf_jump(BPF_JMP | BPF_JEQ | BPF_K, nr as u32, 0, 1),
        bpf_stmt(BPF_RET | BPF_K, SECCOMP_RET_ERRNO | ((error as u32) & SECCOMP_RET_DATA)),
        bpf_stmt(BPF_RET | BPF_K, SECCOMP_RET_ALLOW),
    ];
    let mut prog = SockFprog {
        len: (core::mem::size_of_val(&filter) / core::mem::size_of::<SockFilter>()) as u16,
        filter: filter.as_mut_ptr(),
    };
    if error == -1 {
        let kill = bpf_stmt(BPF_RET | BPF_K, SECCOMP_RET_KILL);
        filter[4] = kill;
    }
    if prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
        perror(b"prctl(NO_NEW_PRIVS)\0".as_ptr() as *const c_char);
        return 1;
    }
    if prctl(PR_SET_SECCOMP, 2, &mut prog as *mut SockFprog) != 0 {
        perror(b"prctl(PR_SET_SECCOMP)\0".as_ptr() as *const c_char);
        return 1;
    }
    0
}

unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if argc < 5 {
        fprintf(
            stderr,
            b"Usage:\ndropper <arch> <syscall_nr> <errno> <prog> [<args>]\nHint:\tAUDIT_ARCH_I386: 0x%X\n\tAUDIT_ARCH_X86_64: 0x%X\n\terrno == -1 means SECCOMP_RET_KILL\n\n\0".as_ptr() as *const c_char,
            AUDIT_ARCH_I386,
            AUDIT_ARCH_X86_64,
        );
        return 1;
    }
    if install_filter(
        strtol(*argv.add(1), core::ptr::null_mut(), 0) as c_int,
        strtol(*argv.add(2), core::ptr::null_mut(), 0) as c_int,
        strtol(*argv.add(3), core::ptr::null_mut(), 0) as c_int,
    ) != 0 {
        return 1;
    }
    execv(*argv.add(4), argv.add(4));
    printf(b"Failed to execv\n\0".as_ptr() as *const c_char);
    255
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
