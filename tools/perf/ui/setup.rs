// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source: perf/ui/setup.c
// C includes referenced dependencies from dlfcn.h, signal.h, unistd.h,
// subcmd/pager.h, util/debug.h, util/hist.h, and ui.h.

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigset_t {
    _private: [u8; 0],
}

pub static mut ui__lock: mutex = mutex { _private: [] };
pub static mut perf_gtk_handle: *mut c_void = ptr::null_mut();
pub static mut use_browser: c_int = -1;

const PERF_GTK_DSO: &[u8] = b"libperf-gtk.so\0";

// Original C uses the build-time LIBDIR macro here.
const LIBDIR: &[u8] = b"LIBDIR\0";

const RTLD_LAZY: c_int = 1;
const PATH_MAX: usize = 4096;
const SIGWINCH: c_int = 28;
const SIG_BLOCK: c_int = 0;
const SIG_UNBLOCK: c_int = 1;

unsafe extern "C" {
    static mut dump_trace: bool;
    static mut perf_use_color_default: c_int;

    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn isatty(fd: c_int) -> c_int;
    fn sleep(seconds: c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn perf_config_colorbool(var: *const c_char, value: *const c_char, stdout_is_tty: c_int) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn setup_pager();
    fn ui__init() -> c_int;
    fn ui__exit(wait_for_ok: bool);
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    fn pthread_sigmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
}

// Original C builds alternate GTK support implementations under HAVE_GTK2_SUPPORT.
// This translation preserves the supported branch behavior directly.
unsafe fn setup_gtk_browser() -> c_int {
    let mut perf_ui_init: Option<unsafe extern "C" fn() -> c_int>;

    if !perf_gtk_handle.is_null() {
        return 0;
    }

    perf_gtk_handle = dlopen(PERF_GTK_DSO.as_ptr() as *const c_char, RTLD_LAZY);
    if perf_gtk_handle.is_null() {
        let mut buf = [0 as c_char; PATH_MAX];
        scnprintf(
            buf.as_mut_ptr(),
            buf.len(),
            c"%s/%s".as_ptr(),
            LIBDIR.as_ptr() as *const c_char,
            PERF_GTK_DSO.as_ptr() as *const c_char,
        );
        perf_gtk_handle = dlopen(buf.as_ptr(), RTLD_LAZY);
    }
    if perf_gtk_handle.is_null() {
        return -1;
    }

    perf_ui_init = core::mem::transmute::<*mut c_void, Option<unsafe extern "C" fn() -> c_int>>(
        dlsym(perf_gtk_handle, c"perf_gtk__init".as_ptr()),
    );
    if perf_ui_init.is_none() {
        dlclose(perf_gtk_handle);
        return -1;
    }

    if perf_ui_init.unwrap()() == 0 {
        return 0;
    }

    dlclose(perf_gtk_handle);
    -1
}

unsafe fn exit_gtk_browser(wait_for_ok: bool) {
    let perf_ui_exit: Option<unsafe extern "C" fn(bool)>;

    if perf_gtk_handle.is_null() {
        return;
    }

    perf_ui_exit = core::mem::transmute::<*mut c_void, Option<unsafe extern "C" fn(bool)>>(
        dlsym(perf_gtk_handle, c"perf_gtk__exit".as_ptr()),
    );
    if perf_ui_exit.is_some() {
        perf_ui_exit.unwrap()(wait_for_ok);
    }

    dlclose(perf_gtk_handle);

    perf_gtk_handle = ptr::null_mut();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdio__config_color(
    _opt: *const option,
    mode: *const c_char,
    _unset: c_int,
) -> c_int {
    perf_use_color_default = perf_config_colorbool(c"color.ui".as_ptr(), mode, -1);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_browser(fallback_to_pager: bool) {
    mutex_init(&raw mut ui__lock);
    if use_browser < 2 && (isatty(1) == 0 || dump_trace) {
        use_browser = 0;
    }

    /* default to TUI */
    if use_browser < 0 {
        use_browser = 1;
    }

    match use_browser {
        2 => {
            if setup_gtk_browser() == 0 {
                return;
            }
            printf(
                c"GTK browser requested but could not find %s\n".as_ptr(),
                PERF_GTK_DSO.as_ptr() as *const c_char,
            );
            sleep(1);
            use_browser = 1;
            if ui__init() == 0 {
                return;
            }
            use_browser = 0;
            if fallback_to_pager {
                setup_pager();
            }
        }
        1 => {
            if ui__init() == 0 {
                return;
            }
            use_browser = 0;
            if fallback_to_pager {
                setup_pager();
            }
        }
        _ => {
            use_browser = 0;
            if fallback_to_pager {
                setup_pager();
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exit_browser(wait_for_ok: bool) {
    match use_browser {
        2 => {
            exit_gtk_browser(wait_for_ok);
        }
        1 => {
            ui__exit(wait_for_ok);
        }
        _ => {}
    }
    mutex_destroy(&raw mut ui__lock);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pthread__block_sigwinch() {
    let mut set = core::mem::MaybeUninit::<sigset_t>::uninit();

    sigemptyset(set.as_mut_ptr());
    sigaddset(set.as_mut_ptr(), SIGWINCH);
    pthread_sigmask(SIG_BLOCK, set.as_ptr(), ptr::null_mut());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pthread__unblock_sigwinch() {
    let mut set = core::mem::MaybeUninit::<sigset_t>::uninit();

    sigemptyset(set.as_mut_ptr());
    sigaddset(set.as_mut_ptr(), SIGWINCH);
    pthread_sigmask(SIG_UNBLOCK, set.as_ptr(), ptr::null_mut());
}
