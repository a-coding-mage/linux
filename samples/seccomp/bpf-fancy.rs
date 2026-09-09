// SPDX-License-Identifier: GPL-2.0
/*
 * Seccomp BPF example using a macro-based generator.
 *
 * Copyright (c) 2012 The Chromium OS Authors <chromium-os-dev@chromium.org>
 * Author: Will Drewry <wad@chromium.org>
 *
 * The code may be used by anyone for any purpose,
 * and can serve as a starting point for developing
 * applications using prctl(PR_ATTACH_SECCOMP_FILTER).
 */

// C headers and "bpf-helper.h" supply the following types, constants, macros,
// and functions in the surrounding build environment.

#[repr(C)]
pub struct bpf_labels {
    pub count: usize,
}

#[repr(C)]
pub struct sock_filter {
    pub code: u16,
    pub jt: u8,
    pub jf: u8,
    pub k: u32,
}

#[repr(C)]
pub struct sock_fprog {
    pub len: u16,
    pub filter: *mut sock_filter,
}

extern "C" {
    fn bpf_resolve_jumps(labels: *mut bpf_labels, filter: *mut sock_filter, count: usize);
    fn prctl(option: i32, ...) -> i32;
    fn perror(s: *const i8);
    fn syscall(number: libc::c_long, ...) -> libc::c_long;
    fn strlen(s: *const i8) -> usize;
}

const PR_SET_NO_NEW_PRIVS: i32 = 38;

pub unsafe fn main(_argc: i32, _argv: *mut *mut i8) -> i32 {
    let mut l = bpf_labels { count: 0 };
    static MSG1: &[u8] = b"Please type something: \0";
    static MSG2: &[u8] = b"You typed: \0";
    let mut buf = [0u8; 256];
    let mut filter: [sock_filter; 37] = [
        // TODO: LOAD_SYSCALL_NR(arch) and enforce an arch
        LOAD_SYSCALL_NR!(),
        SYSCALL!(__NR_exit, ALLOW!),
        SYSCALL!(__NR_exit_group, ALLOW!),
        SYSCALL!(__NR_write, JUMP!(&mut l, write_fd)),
        SYSCALL!(__NR_read, JUMP!(&mut l, read)),
        DENY!(),

        LABEL!(&mut l, read),
        ARG!(0),
        JNE!(STDIN_FILENO, DENY!()),
        ARG!(1),
        JNE!(buf.as_mut_ptr() as usize as u64, DENY!()),
        ARG!(2),
        JGE!(buf.len(), DENY!()),
        ALLOW!(),

        LABEL!(&mut l, write_fd),
        ARG!(0),
        JEQ!(STDOUT_FILENO, JUMP!(&mut l, write_buf)),
        JEQ!(STDERR_FILENO, JUMP!(&mut l, write_buf)),
        DENY!(),

        LABEL!(&mut l, write_buf),
        ARG!(1),
        JEQ!(MSG1.as_ptr() as usize as u64, JUMP!(&mut l, msg1_len)),
        JEQ!(MSG2.as_ptr() as usize as u64, JUMP!(&mut l, msg2_len)),
        JEQ!(buf.as_mut_ptr() as usize as u64, JUMP!(&mut l, buf_len)),
        DENY!(),

        LABEL!(&mut l, msg1_len),
        ARG!(2),
        JLT!(MSG1.len(), ALLOW!()),
        DENY!(),

        LABEL!(&mut l, msg2_len),
        ARG!(2),
        JLT!(MSG2.len(), ALLOW!()),
        DENY!(),

        LABEL!(&mut l, buf_len),
        ARG!(2),
        JLT!(buf.len(), ALLOW!()),
        DENY!(),
    ];
    let mut prog = sock_fprog {
        filter: filter.as_mut_ptr(),
        len: filter.len() as u16,
    };
    let mut bytes: libc::ssize_t;

    bpf_resolve_jumps(&mut l, filter.as_mut_ptr(), filter.len());

    if prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
        perror(b"prctl(NO_NEW_PRIVS)\0".as_ptr() as *const i8);
        return 1;
    }

    if prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &mut prog) != 0 {
        perror(b"prctl(SECCOMP)\0".as_ptr() as *const i8);
        return 1;
    }
    syscall(__NR_write, STDOUT_FILENO, MSG1.as_ptr(), strlen(MSG1.as_ptr() as *const i8));
    bytes = syscall(__NR_read, STDIN_FILENO, buf.as_mut_ptr(), buf.len() - 1) as libc::ssize_t;
    bytes = if bytes > 0 { bytes } else { 0 };
    syscall(__NR_write, STDERR_FILENO, MSG2.as_ptr(), strlen(MSG2.as_ptr() as *const i8));
    syscall(__NR_write, STDERR_FILENO, buf.as_ptr(), bytes);
    // Now get killed
    syscall(__NR_write, STDERR_FILENO, MSG2.as_ptr(), strlen(MSG2.as_ptr() as *const i8) + 2);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
