// Translated from perf/ui/tui/setup.c.
// C includes referenced errno, signal, bool, stdlib, termios, unistd, linux/kernel,
// optional execinfo backtrace support, and perf UI/slang headers.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type sighandler_t = unsafe extern "C" fn(c_int);

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct fd_set {
    pub fds_bits: [c_long; 16],
}

#[repr(C)]
pub struct termios {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_error_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

const EINTR: c_int = 4;
const SIGSEGV: c_int = 11;
const SIGFPE: c_int = 8;
const SIGINT: c_int = 2;
const SIGQUIT: c_int = 3;
const SIGTERM: c_int = 15;
const SIGTSTP: c_int = 20;
const SIGCONT: c_int = 18;
const SIGSTOP: c_int = 19;
const SIGWINCH: c_int = 28;
const TCSADRAIN: c_int = 1;

const K_TIMER: c_int = 0x1000;
const K_RESIZE: c_int = 0x1001;
const K_ERROR: c_int = 0x1002;
const K_ESC: c_int = 0x1b;
const SL_KEY_UNTAB: c_int = 0x101;

const PERF_COLOR_RESET: &[u8] = b"\x1b[0m";

static mut ui__need_resize: c_int = 0;

unsafe extern "C" {
    static mut errno: c_int;
    static mut stdout: *mut c_void;

    static mut perf_tui_eops: perf_error_ops;
    static mut tui_helpline__set: bool_;
    static mut ui__lock: mutex;
    static mut ui_helpline__last_msg: *const c_char;
    static mut SLang_TT_Read_FD: c_int;

    fn hist_browser__init_hpp();

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_trylock(lock: *mut mutex) -> c_int;

    fn pthread__unblock_sigwinch();
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *mut timeval,
    ) -> c_int;
    fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> isize;
    fn raise(sig: c_int) -> c_int;
    fn psignal(sig: c_int, s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;

    fn SLtt_get_screen_size();
    fn SLsmg_reinit_smg();
    fn SLang_getkey() -> c_int;
    fn SLang_ungetkey(key: c_int);
    fn SLkp_getkey() -> c_int;
    fn SLutf8_enable(mode: c_int);
    fn SLtt_get_terminfo();
    fn SLsmg_init_smg() -> c_int;
    fn SLang_init_tty(a: c_int, b: c_int, c: c_int) -> c_int;
    fn SLtty_set_suspend_state(state: bool_);
    fn SLkp_init() -> c_int;
    fn SLkp_define_keysym(seq: *const c_char, keysym: c_int);
    fn SLtt_set_cursor_visibility(state: c_int);
    fn SLsmg_refresh();
    fn SLsmg_reset_smg();
    fn SLang_reset_tty();

    fn pr_err(format: *const c_char, ...);
    fn perf_error__register(ops: *mut perf_error_ops);
    fn perf_error__unregister(ops: *mut perf_error_ops);
    fn ui_helpline__init();
    fn ui_browser__init();
    fn tui_progress__init();
    fn ui__question_window(
        title: *const c_char,
        text: *const c_char,
        exit_msg: *const c_char,
        delay_secs: c_int,
    );

    // Present only when HAVE_BACKTRACE_SUPPORT is enabled in the original build.
    fn backtrace(buffer: *mut *mut c_void, size: c_int) -> c_int;
    fn __dump_stack(file: *mut c_void, stackdump: *mut *mut c_void, size: size_t);
}

unsafe fn FD_ZERO(set: *mut fd_set) {
    unsafe {
        ptr::write_bytes(set as *mut u8, 0, size_of::<fd_set>());
    }
}

unsafe fn FD_SET(fd: c_int, set: *mut fd_set) {
    let bits_per_word = 8 * size_of::<c_long>() as c_int;
    let index = (fd / bits_per_word) as usize;
    let bit = fd % bits_per_word;
    unsafe {
        (*set).fds_bits[index] |= (1 as c_long) << bit;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui__refresh_dimensions(force: bool_) {
    unsafe {
        if force || ui__need_resize != 0 {
            ui__need_resize = 0;
            mutex_lock(&raw mut ui__lock);
            SLtt_get_screen_size();
            SLsmg_reinit_smg();
            mutex_unlock(&raw mut ui__lock);
        }
    }
}

unsafe extern "C" fn ui__sigwinch(_sig: c_int) {
    unsafe {
        ui__need_resize = 1;
    }
}

unsafe fn ui__setup_sigwinch() {
    static mut done: bool_ = false;

    unsafe {
        if done {
            return;
        }

        done = true;
        pthread__unblock_sigwinch();
        signal(SIGWINCH, ui__sigwinch);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui__getch(delay_secs: c_int) -> c_int {
    unsafe {
        let mut timeout: timeval = core::mem::zeroed();
        let mut read_set: fd_set = core::mem::zeroed();
        let mut err: c_int;
        let key: c_int;
        let ptimeout: *mut timeval = if delay_secs != 0 {
            &mut timeout
        } else {
            ptr::null_mut()
        };

        ui__setup_sigwinch();

        FD_ZERO(&mut read_set);
        FD_SET(0, &mut read_set);

        if delay_secs != 0 {
            timeout.tv_sec = delay_secs as c_long;
            timeout.tv_usec = 0;
        }

        err = select(1, &mut read_set, ptr::null_mut(), ptr::null_mut(), ptimeout);

        if err == 0 {
            return K_TIMER;
        }

        if err == -1 {
            if errno == EINTR {
                return K_RESIZE;
            }
            return K_ERROR;
        }

        key = SLang_getkey();
        if key != K_ESC {
            return key;
        }

        FD_ZERO(&mut read_set);
        FD_SET(0, &mut read_set);
        timeout.tv_sec = 0;
        timeout.tv_usec = 20;
        err = select(1, &mut read_set, ptr::null_mut(), ptr::null_mut(), &mut timeout);
        if err == 0 {
            return K_ESC;
        }

        SLang_ungetkey(key);
        SLkp_getkey()
    }
}

// Original C condition:
// #ifdef HAVE_BACKTRACE_SUPPORT
unsafe extern "C" fn ui__signal_backtrace(sig: c_int) {
    unsafe {
        let mut stackdump: [*mut c_void; 32] = [ptr::null_mut(); 32];
        let size: size_t;

        ui__exit(false);
        psignal(sig, c"perf".as_ptr());

        printf(c"-------- backtrace --------\n".as_ptr());
        size = backtrace(stackdump.as_mut_ptr(), stackdump.len() as c_int) as size_t;
        __dump_stack(stdout, stackdump.as_mut_ptr(), size);

        exit(0);
    }
}
// Original C #else used: #define ui__signal_backtrace ui__signal

unsafe extern "C" fn ui__signal(sig: c_int) {
    unsafe {
        ui__exit(false);
        psignal(sig, c"perf".as_ptr());
        exit(0);
    }
}

unsafe extern "C" fn ui__sigcont(sig: c_int) {
    static mut tty: termios = termios { _private: [] };

    unsafe {
        if sig == SIGTSTP {
            while tcgetattr(SLang_TT_Read_FD, &raw mut tty) == -1 && errno == EINTR {}
            while write(
                SLang_TT_Read_FD,
                PERF_COLOR_RESET.as_ptr() as *const c_void,
                PERF_COLOR_RESET.len(),
            ) == -1
                && errno == EINTR
            {}
            raise(SIGSTOP);
        } else {
            while tcsetattr(SLang_TT_Read_FD, TCSADRAIN, &raw const tty) == -1 && errno == EINTR {}
            raise(SIGWINCH);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui__init() -> c_int {
    unsafe {
        let mut err: c_int;

        SLutf8_enable(-1);
        SLtt_get_terminfo();
        SLtt_get_screen_size();

        err = SLsmg_init_smg();
        if err < 0 {
            return err;
        }
        err = SLang_init_tty(-1, 0, 0);
        if err < 0 {
            return err;
        }
        SLtty_set_suspend_state(true);

        err = SLkp_init();
        if err < 0 {
            pr_err(c"TUI initialization failed.\n".as_ptr());
            return err;
        }

        SLkp_define_keysym(c"^(kB)".as_ptr(), SL_KEY_UNTAB);

        signal(SIGSEGV, ui__signal_backtrace);
        signal(SIGFPE, ui__signal_backtrace);
        signal(SIGINT, ui__signal);
        signal(SIGQUIT, ui__signal);
        signal(SIGTERM, ui__signal);
        signal(SIGTSTP, ui__sigcont);
        signal(SIGCONT, ui__sigcont);

        perf_error__register(&raw mut perf_tui_eops);

        ui_helpline__init();
        ui_browser__init();
        tui_progress__init();

        hist_browser__init_hpp();
        err
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui__exit(wait_for_ok: bool_) {
    unsafe {
        if wait_for_ok && tui_helpline__set {
            ui__question_window(
                c"Fatal Error".as_ptr(),
                ui_helpline__last_msg,
                c"Press any key...".as_ptr(),
                0,
            );
        }

        SLtt_set_cursor_visibility(1);
        if mutex_trylock(&raw mut ui__lock) != 0 {
            SLsmg_refresh();
            SLsmg_reset_smg();
            mutex_unlock(&raw mut ui__lock);
        }
        SLang_reset_tty();
        perf_error__unregister(&raw mut perf_tui_eops);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
