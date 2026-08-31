// SPDX-License-Identifier: GPL-2.0
//
// Translated from lib/subcmd/pager.c. C include dependencies are represented
// by the external declarations below.

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_ulong};
use std::ptr;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fd_set {
    // Provided by <sys/select.h> in C.
    _private: [u8; 0],
}

#[repr(C)]
pub struct winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

#[repr(C)]
pub struct child_process {
    pub argv: *mut *const c_char,
    pub in_: c_int,
    pub preexec_cb: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct subcmd_config_type {
    pub pager_env: *const c_char,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut subcmd_config: subcmd_config_type;

    fn getenv(name: *const c_char) -> *mut c_char;
    fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn isatty(fd: c_int) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *mut c_void,
    ) -> c_int;
    fn raise(sig: c_int) -> c_int;
    fn atexit(function: unsafe extern "C" fn()) -> c_int;

    fn start_command(cmd: *mut child_process) -> c_int;
    fn finish_command(cmd: *mut child_process) -> c_int;
    fn sigchain_push_common(handler: unsafe extern "C" fn(c_int));
    fn sigchain_pop(signo: c_int);

    // C macros from <sys/select.h>.
    fn FD_ZERO(set: *mut fd_set);
    fn FD_SET(fd: c_int, set: *mut fd_set);
}

const X_OK: c_int = 1;
const TIOCGWINSZ: c_ulong = 0x5413;

/*
 * This is split up from the rest of git so that we can do
 * something different on Windows.
 */

static mut spawned_pager: c_int = 0;
static mut pager_columns: c_int = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pager_init(pager_env: *const c_char) {
    unsafe {
        subcmd_config.pager_env = pager_env;
    }
}

static mut forced_pager: *const c_char = ptr::null();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn force_pager(pager: *const c_char) {
    unsafe {
        forced_pager = pager;
    }
}

unsafe extern "C" fn pager_preexec() {
    /*
     * Work around bug in "less" by not starting it until we
     * have real input
     */
    let mut in_: fd_set = unsafe { std::mem::zeroed() };
    let mut exception: fd_set = unsafe { std::mem::zeroed() };

    unsafe {
        FD_ZERO(&mut in_);
        FD_ZERO(&mut exception);
        FD_SET(0, &mut in_);
        FD_SET(0, &mut exception);
        select(
            1,
            &mut in_,
            ptr::null_mut(),
            &mut exception,
            ptr::null_mut(),
        );

        setenv(c"LESS".as_ptr(), c"FRSX".as_ptr(), 0);
    }
}

static mut pager_argv: [*const c_char; 4] = [
    c"sh".as_ptr(),
    c"-c".as_ptr(),
    ptr::null(),
    ptr::null(),
];

static mut pager_process: child_process = child_process {
    argv: ptr::null_mut(),
    in_: 0,
    preexec_cb: None,
};

unsafe extern "C" fn wait_for_pager() {
    unsafe {
        fflush(stdout);
        fflush(stderr);
        /* signal EOF to pager */
        close(1);
        close(2);
        finish_command(&raw mut pager_process);
    }
}

unsafe extern "C" fn wait_for_pager_signal(signo: c_int) {
    unsafe {
        wait_for_pager();
        sigchain_pop(signo);
        raise(signo);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_pager() {
    let mut sz: winsize = unsafe { std::mem::zeroed() };
    let mut pager: *const c_char = unsafe { getenv(subcmd_config.pager_env) };

    unsafe {
        if !forced_pager.is_null() {
            pager = forced_pager;
        }
        if isatty(1) == 0 && forced_pager.is_null() {
            return;
        }
        if ioctl(1, TIOCGWINSZ, &mut sz) == 0 {
            pager_columns = sz.ws_col as c_int;
        }
        if pager.is_null() {
            pager = getenv(c"PAGER".as_ptr());
        }
        if !( !pager.is_null() || access(c"/usr/bin/pager".as_ptr(), X_OK) != 0 ) {
            pager = c"/usr/bin/pager".as_ptr();
        }
        if !( !pager.is_null() || access(c"/usr/bin/less".as_ptr(), X_OK) != 0 ) {
            pager = c"/usr/bin/less".as_ptr();
        }
        if pager.is_null() {
            pager = c"cat".as_ptr();
        }
        if *pager == 0 || strcmp(pager, c"cat".as_ptr()) == 0 {
            return;
        }

        spawned_pager = 1; /* means we are emitting to terminal */

        /* spawn the pager */
        pager_argv[2] = pager;
        pager_process.argv = (&raw mut pager_argv) as *mut *const c_char;
        pager_process.in_ = -1;
        pager_process.preexec_cb = Some(pager_preexec);

        if start_command(&raw mut pager_process) != 0 {
            return;
        }

        /* original process continues, but writes to the pipe */
        dup2(pager_process.in_, 1);
        if isatty(2) != 0 {
            dup2(pager_process.in_, 2);
        }
        close(pager_process.in_);

        /* this makes sure that the parent terminates after the pager */
        sigchain_push_common(wait_for_pager_signal);
        atexit(wait_for_pager);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pager_in_use() -> c_int {
    unsafe { spawned_pager }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pager_get_columns() -> c_int {
    let s: *mut c_char;

    unsafe {
        s = getenv(c"COLUMNS".as_ptr());
        if !s.is_null() {
            return atoi(s);
        }

        (if pager_columns != 0 { pager_columns } else { 80 }) - 2
    }
}
