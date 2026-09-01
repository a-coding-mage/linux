// SPDX-License-Identifier: LGPL-2.1
// C dependencies: "trace/beauty/beauty.h", <signal.h>

use core::ffi::{c_char, c_int};

extern "C" {
    fn scnprintf(bf: *mut c_char, size: usize, fmt: *const c_char, ...) -> usize;
}

#[repr(C)]
pub struct syscall_arg {
    pub val: u64,
    pub show_string_prefix: bool,
}

extern "C" {
    static SIGABRT: c_int;
    static SIGALRM: c_int;
    static SIGBUS: c_int;
    static SIGCHLD: c_int;
    static SIGCONT: c_int;
    static SIGFPE: c_int;
    static SIGHUP: c_int;
    static SIGILL: c_int;
    static SIGINT: c_int;
    static SIGIO: c_int;
    static SIGKILL: c_int;
    static SIGPIPE: c_int;
    static SIGPROF: c_int;
    static SIGPWR: c_int;
    static SIGQUIT: c_int;
    static SIGSEGV: c_int;
    static SIGSTOP: c_int;
    static SIGSYS: c_int;
    static SIGTERM: c_int;
    static SIGTRAP: c_int;
    static SIGTSTP: c_int;
    static SIGTTIN: c_int;
    static SIGTTOU: c_int;
    static SIGURG: c_int;
    static SIGUSR1: c_int;
    static SIGUSR2: c_int;
    static SIGVTALRM: c_int;
    static SIGWINCH: c_int;
    static SIGXCPU: c_int;
    static SIGXFSZ: c_int;

    // Present only when the corresponding C preprocessor symbol is defined.
    static SIGEMT: c_int;
    static SIGSTKFLT: c_int;
    static SIGSWI: c_int;
}

unsafe fn scnprintf_signum_name(
    bf: *mut c_char,
    size: usize,
    show_prefix: bool,
    name: *const c_char,
) -> usize {
    let prefix = b"SIG\0".as_ptr() as *const c_char;
    let empty = b"\0".as_ptr() as *const c_char;

    unsafe {
        scnprintf(
            bf,
            size,
            b"%s%s\0".as_ptr() as *const c_char,
            if show_prefix { prefix } else { empty },
            name,
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_signum(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let show_prefix = unsafe { (*arg).show_string_prefix };
    let sig = unsafe { (*arg).val as c_int };

    unsafe {
        if sig == SIGHUP {
            return scnprintf_signum_name(bf, size, show_prefix, b"HUP\0".as_ptr() as *const c_char);
        }
        if sig == SIGINT {
            return scnprintf_signum_name(bf, size, show_prefix, b"INT\0".as_ptr() as *const c_char);
        }
        if sig == SIGQUIT {
            return scnprintf_signum_name(bf, size, show_prefix, b"QUIT\0".as_ptr() as *const c_char);
        }
        if sig == SIGILL {
            return scnprintf_signum_name(bf, size, show_prefix, b"ILL\0".as_ptr() as *const c_char);
        }
        if sig == SIGTRAP {
            return scnprintf_signum_name(bf, size, show_prefix, b"TRAP\0".as_ptr() as *const c_char);
        }
        if sig == SIGABRT {
            return scnprintf_signum_name(bf, size, show_prefix, b"ABRT\0".as_ptr() as *const c_char);
        }
        if sig == SIGBUS {
            return scnprintf_signum_name(bf, size, show_prefix, b"BUS\0".as_ptr() as *const c_char);
        }
        if sig == SIGFPE {
            return scnprintf_signum_name(bf, size, show_prefix, b"FPE\0".as_ptr() as *const c_char);
        }
        if sig == SIGKILL {
            return scnprintf_signum_name(bf, size, show_prefix, b"KILL\0".as_ptr() as *const c_char);
        }
        if sig == SIGUSR1 {
            return scnprintf_signum_name(bf, size, show_prefix, b"USR1\0".as_ptr() as *const c_char);
        }
        if sig == SIGSEGV {
            return scnprintf_signum_name(bf, size, show_prefix, b"SEGV\0".as_ptr() as *const c_char);
        }
        if sig == SIGUSR2 {
            return scnprintf_signum_name(bf, size, show_prefix, b"USR2\0".as_ptr() as *const c_char);
        }
        if sig == SIGPIPE {
            return scnprintf_signum_name(bf, size, show_prefix, b"PIPE\0".as_ptr() as *const c_char);
        }
        if sig == SIGALRM {
            return scnprintf_signum_name(bf, size, show_prefix, b"ALRM\0".as_ptr() as *const c_char);
        }
        if sig == SIGTERM {
            return scnprintf_signum_name(bf, size, show_prefix, b"TERM\0".as_ptr() as *const c_char);
        }
        if sig == SIGCHLD {
            return scnprintf_signum_name(bf, size, show_prefix, b"CHLD\0".as_ptr() as *const c_char);
        }
        if sig == SIGCONT {
            return scnprintf_signum_name(bf, size, show_prefix, b"CONT\0".as_ptr() as *const c_char);
        }
        if sig == SIGSTOP {
            return scnprintf_signum_name(bf, size, show_prefix, b"STOP\0".as_ptr() as *const c_char);
        }
        if sig == SIGTSTP {
            return scnprintf_signum_name(bf, size, show_prefix, b"TSTP\0".as_ptr() as *const c_char);
        }
        if sig == SIGTTIN {
            return scnprintf_signum_name(bf, size, show_prefix, b"TTIN\0".as_ptr() as *const c_char);
        }
        if sig == SIGTTOU {
            return scnprintf_signum_name(bf, size, show_prefix, b"TTOU\0".as_ptr() as *const c_char);
        }
        if sig == SIGURG {
            return scnprintf_signum_name(bf, size, show_prefix, b"URG\0".as_ptr() as *const c_char);
        }
        if sig == SIGXCPU {
            return scnprintf_signum_name(bf, size, show_prefix, b"XCPU\0".as_ptr() as *const c_char);
        }
        if sig == SIGXFSZ {
            return scnprintf_signum_name(bf, size, show_prefix, b"XFSZ\0".as_ptr() as *const c_char);
        }
        if sig == SIGVTALRM {
            return scnprintf_signum_name(bf, size, show_prefix, b"VTALRM\0".as_ptr() as *const c_char);
        }
        if sig == SIGPROF {
            return scnprintf_signum_name(bf, size, show_prefix, b"PROF\0".as_ptr() as *const c_char);
        }
        if sig == SIGWINCH {
            return scnprintf_signum_name(bf, size, show_prefix, b"WINCH\0".as_ptr() as *const c_char);
        }
        if sig == SIGIO {
            return scnprintf_signum_name(bf, size, show_prefix, b"IO\0".as_ptr() as *const c_char);
        }
        if sig == SIGPWR {
            return scnprintf_signum_name(bf, size, show_prefix, b"PWR\0".as_ptr() as *const c_char);
        }
        if sig == SIGSYS {
            return scnprintf_signum_name(bf, size, show_prefix, b"SYS\0".as_ptr() as *const c_char);
        }

        // C conditional: #ifdef SIGEMT
        if sig == SIGEMT {
            return scnprintf_signum_name(bf, size, show_prefix, b"EMT\0".as_ptr() as *const c_char);
        }
        // C conditional: #ifdef SIGSTKFLT
        if sig == SIGSTKFLT {
            return scnprintf_signum_name(bf, size, show_prefix, b"STKFLT\0".as_ptr() as *const c_char);
        }
        // C conditional: #ifdef SIGSWI
        if sig == SIGSWI {
            return scnprintf_signum_name(bf, size, show_prefix, b"SWI\0".as_ptr() as *const c_char);
        }

        scnprintf(
            bf,
            size,
            b"%#x\0".as_ptr() as *const c_char,
            sig,
        )
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
