// SPDX-License-Identifier: GPL-2.0
// C dependencies: <signal.h>, "subcmd-util.h", "sigchain.h"

use core::ffi::{c_char, c_int};

const SIGCHAIN_MAX_SIGNALS: usize = 32;

pub type sigchain_fun = Option<unsafe extern "C" fn(c_int)>;

const SIGINT: c_int = 2;
const SIGHUP: c_int = 1;
const SIGTERM: c_int = 15;
const SIGQUIT: c_int = 3;
const SIGPIPE: c_int = 13;

#[repr(C)]
struct sigchain_signal {
    old: *mut sigchain_fun,
    n: c_int,
    alloc: c_int,
}

static mut signals: [sigchain_signal; SIGCHAIN_MAX_SIGNALS] = [const {
    sigchain_signal {
        old: core::ptr::null_mut(),
        n: 0,
        alloc: 0,
    }
}; SIGCHAIN_MAX_SIGNALS];

unsafe extern "C" {
    fn die(err: *const c_char, ...);
    fn signal(signum: c_int, handler: sigchain_fun) -> sigchain_fun;
    static SIG_ERR: sigchain_fun;
}

unsafe fn check_signum(sig: c_int) {
    if sig < 1 || sig >= SIGCHAIN_MAX_SIGNALS as c_int {
        unsafe {
            die(
                c"BUG: signal out of range: %d".as_ptr(),
                sig,
            );
        }
    }
}

unsafe fn sigchain_push(sig: c_int, f: sigchain_fun) -> c_int {
    let s: *mut sigchain_signal = unsafe { signals.as_mut_ptr().offset(sig as isize) };
    unsafe {
        check_signum(sig);

        ALLOC_GROW!((*s).old, (*s).n + 1, (*s).alloc);
        *(*s).old.offset((*s).n as isize) = signal(sig, f);
        if *(*s).old.offset((*s).n as isize) == SIG_ERR {
            return -1;
        }
        (*s).n += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigchain_pop(sig: c_int) -> c_int {
    let s: *mut sigchain_signal = unsafe { signals.as_mut_ptr().offset(sig as isize) };
    unsafe {
        check_signum(sig);
        if (*s).n < 1 {
            return 0;
        }

        if signal(sig, *(*s).old.offset(((*s).n - 1) as isize)) == SIG_ERR {
            return -1;
        }
        (*s).n -= 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigchain_push_common(f: sigchain_fun) {
    unsafe {
        sigchain_push(SIGINT, f);
        sigchain_push(SIGHUP, f);
        sigchain_push(SIGTERM, f);
        sigchain_push(SIGQUIT, f);
        sigchain_push(SIGPIPE, f);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
