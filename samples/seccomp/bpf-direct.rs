// SPDX-License-Identifier: GPL-2.0
/*
 * Seccomp filter example for x86 (32-bit and 64-bit) with BPF macros
 *
 * Copyright (c) 2012 The Chromium OS Authors <chromium-os-dev@chromium.org>
 * Author: Will Drewry <wad@chromium.org>
 *
 * The code may be used by anyone for any purpose,
 * and can serve as a starting point for developing
 * applications using prctl(PR_SET_SECCOMP, 2, ...).
 */

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod supported {
    // The following names are supplied by the system headers and other files.
    use core::ffi::{c_char, c_int, c_void};

    type SizeT = usize;
    type SSizeT = isize;

    #[repr(C)]
    struct SigInfo {
        si_code: c_int,
        _rest: [u8; 128 - core::mem::size_of::<c_int>()],
    }

    #[repr(C)]
    struct UContext;
    #[repr(C)]
    struct SigSet {
        _data: [u8; 128],
    }
    #[repr(C)]
    struct SigAction {
        sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut SigInfo, *mut c_void)>,
        sa_mask: SigSet,
        sa_flags: usize,
        sa_restorer: Option<unsafe extern "C" fn()>,
    }

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

    extern "C" {
        fn memset(s: *mut c_void, c: c_int, n: SizeT) -> *mut c_void;
        fn sigemptyset(set: *mut SigSet) -> c_int;
        fn sigaddset(set: *mut SigSet, signum: c_int) -> c_int;
        fn sigaction(signum: c_int, act: *const SigAction, oldact: *mut SigAction) -> c_int;
        fn sigprocmask(how: c_int, set: *const SigSet, oldset: *mut SigSet) -> c_int;
        fn perror(s: *const c_char);
        fn prctl(option: c_int, ...) -> c_int;
        fn write(fd: c_int, buf: *const c_void, count: SizeT) -> SSizeT;
        fn syscall(number: isize, ...) -> SSizeT;
    }

    const SYS_SECCOMP: c_int = 1;
    const PR_SET_NO_NEW_PRIVS: c_int = 38;
    const SIGSYS: c_int = 31;
    const SIG_UNBLOCK: c_int = 2;
    const SA_SIGINFO: usize = 4;
    const STDIN_FILENO: c_int = 0;
    const STDOUT_FILENO: c_int = 1;
    const STDERR_FILENO: c_int = 2;
    const SECCOMP_MODE_FILTER: c_int = 2;
    const SECCOMP_RET_ALLOW: u32 = 0x7fff0000;
    const SECCOMP_RET_KILL: u32 = 0x00000000;
    const SECCOMP_RET_TRAP: u32 = 0x00030000;
    const BPF_LD: u16 = 0x00;
    const BPF_W: u16 = 0x00;
    const BPF_ABS: u16 = 0x20;
    const BPF_JMP: u16 = 0x05;
    const BPF_JEQ: u16 = 0x10;
    const BPF_K: u16 = 0x00;
    const BPF_RET: u16 = 0x06;

    #[cfg(target_arch = "x86")]
    const REG_RESULT: usize = 11;
    #[cfg(target_arch = "x86")]
    const REG_SYSCALL: usize = 11;
    #[cfg(target_arch = "x86")]
    const REG_ARG0: usize = 8;
    #[cfg(target_arch = "x86")]
    const REG_ARG1: usize = 9;
    #[cfg(target_arch = "x86")]
    const REG_ARG2: usize = 10;
    #[cfg(target_arch = "x86")]
    const REG_ARG3: usize = 6;
    #[cfg(target_arch = "x86")]
    const REG_ARG4: usize = 7;
    #[cfg(target_arch = "x86")]
    const REG_ARG5: usize = 5;

    #[cfg(target_arch = "x86_64")]
    const REG_RESULT: usize = 13;
    #[cfg(target_arch = "x86_64")]
    const REG_SYSCALL: usize = 13;
    #[cfg(target_arch = "x86_64")]
    const REG_ARG0: usize = 8;
    #[cfg(target_arch = "x86_64")]
    const REG_ARG1: usize = 9;
    #[cfg(target_arch = "x86_64")]
    const REG_ARG2: usize = 12;
    #[cfg(target_arch = "x86_64")]
    const REG_ARG3: usize = 14;
    #[cfg(target_arch = "x86_64")]
    const REG_ARG4: usize = 15;
    #[cfg(target_arch = "x86_64")]
    const REG_ARG5: usize = 17;

    const SYS_NR: u32 = 0;
    const SYS_ARG0: u32 = 16;

    #[repr(C)]
    struct MachineContext {
        gregs: [isize; 32],
    }
    #[repr(C)]
    struct Context {
        _prefix: [u8; 40],
        uc_mcontext: MachineContext,
    }

    unsafe extern "C" fn emulator(_nr: c_int, info: *mut SigInfo, void_context: *mut c_void) {
        let ctx = void_context as *mut Context;
        if (*info).si_code != SYS_SECCOMP || ctx.is_null() {
            return;
        }
        let syscall = (*ctx).uc_mcontext.gregs[REG_SYSCALL];
        let buf = (*ctx).uc_mcontext.gregs[REG_ARG1] as *const c_void;
        let len = (*ctx).uc_mcontext.gregs[REG_ARG2] as SizeT;
        if syscall != 1 || (*ctx).uc_mcontext.gregs[REG_ARG0] != STDERR_FILENO as isize {
            return;
        }
        // Redirect stderr messages to stdout. Doesn't handle EINTR, etc
        (*ctx).uc_mcontext.gregs[REG_RESULT] = -1;
        if write(STDOUT_FILENO, b"[ERR] \0".as_ptr() as *const c_void, 6) > 0 {
            let bytes = write(STDOUT_FILENO, buf, len);
            (*ctx).uc_mcontext.gregs[REG_RESULT] = bytes;
        }
    }

    unsafe fn install_emulator() -> c_int {
        let mut act: SigAction = core::mem::zeroed();
        let mut mask: SigSet = core::mem::zeroed();
        sigemptyset(&mut mask);
        sigaddset(&mut mask, SIGSYS);
        act.sa_sigaction = Some(emulator);
        act.sa_flags = SA_SIGINFO;
        if sigaction(SIGSYS, &act, core::ptr::null_mut()) < 0 {
            perror(b"sigaction\0".as_ptr() as *const c_char);
            return -1;
        }
        if sigprocmask(SIG_UNBLOCK, &mask, core::ptr::null_mut()) != 0 {
            perror(b"sigprocmask\0".as_ptr() as *const c_char);
            return -1;
        }
        0
    }

    unsafe fn install_filter() -> c_int {
        // BPF_STMT/BPF_JUMP and syscall-number constants are supplied by system headers.
        let mut filter = [
            SockFilter { code: BPF_LD | BPF_W | BPF_ABS, jt: 0, jf: 0, k: SYS_NR },
            SockFilter { code: BPF_JMP | BPF_JEQ | BPF_K, jt: 0, jf: 1, k: 15 },
            SockFilter { code: BPF_RET | BPF_K, jt: 0, jf: 0, k: SECCOMP_RET_ALLOW },
            SockFilter { code: BPF_JMP | BPF_JEQ | BPF_K, jt: 0, jf: 1, k: 231 },
            SockFilter { code: BPF_RET | BPF_K, jt: 0, jf: 0, k: SECCOMP_RET_ALLOW },
            SockFilter { code: BPF_JMP | BPF_JEQ | BPF_K, jt: 0, jf: 1, k: 60 },
            SockFilter { code: BPF_RET | BPF_K, jt: 0, jf: 0, k: SECCOMP_RET_ALLOW },
            SockFilter { code: BPF_JMP | BPF_JEQ | BPF_K, jt: 1, jf: 0, k: 0 },
            SockFilter { code: BPF_JMP | BPF_JEQ | BPF_K, jt: 3, jf: 2, k: 1 },
            SockFilter { code: BPF_LD | BPF_W | BPF_ABS, jt: 0, jf: 0, k: SYS_ARG0 },
            SockFilter { code: BPF_JMP | BPF_JEQ | BPF_K, jt: 4, jf: 0, k: STDIN_FILENO as u32 },
            SockFilter { code: BPF_RET | BPF_K, jt: 0, jf: 0, k: SECCOMP_RET_KILL },
            SockFilter { code: BPF_LD | BPF_W | BPF_ABS, jt: 0, jf: 0, k: SYS_ARG0 },
            SockFilter { code: BPF_JMP | BPF_JEQ | BPF_K, jt: 1, jf: 0, k: STDOUT_FILENO as u32 },
            SockFilter { code: BPF_JMP | BPF_JEQ | BPF_K, jt: 1, jf: 2, k: STDERR_FILENO as u32 },
            SockFilter { code: BPF_RET | BPF_K, jt: 0, jf: 0, k: SECCOMP_RET_ALLOW },
            SockFilter { code: BPF_RET | BPF_K, jt: 0, jf: 0, k: SECCOMP_RET_TRAP },
            SockFilter { code: BPF_RET | BPF_K, jt: 0, jf: 0, k: SECCOMP_RET_KILL },
        ];
        let prog = SockFprog { len: filter.len() as u16, filter: filter.as_mut_ptr() };
        if prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
            perror(b"prctl(NO_NEW_PRIVS)\0".as_ptr() as *const c_char);
            return 1;
        }
        if prctl(22, SECCOMP_MODE_FILTER, &prog as *const SockFprog) != 0 {
            perror(b"prctl\0".as_ptr() as *const c_char);
            return 1;
        }
        0
    }

    pub unsafe fn main() -> c_int {
        let mut buf = [0u8; 4096];
        let mut bytes: SSizeT = 0;
        if install_emulator() != 0 || install_filter() != 0 { return 1; }
        syscall(1, STDOUT_FILENO, b"OHAI! WHAT IS YOUR NAME? ".as_ptr(), 26);
        bytes = syscall(0, STDIN_FILENO, buf.as_mut_ptr(), buf.len());
        syscall(1, STDOUT_FILENO, b"HELLO, ".as_ptr(), 7);
        syscall(1, STDOUT_FILENO, buf.as_ptr(), bytes);
        syscall(1, STDERR_FILENO, b"Error message going to STDERR\n".as_ptr(), 31);
        0
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn main() -> i32 {
    // This sample is x86-only. Since kernel samples are compiled with the
    // host toolchain, a non-x86 host uses only the main() below.
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
