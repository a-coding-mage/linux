/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: "../util/mutex.h", <stdbool.h>, <linux/compiler.h>

use core::ffi::{c_char, c_int, c_void};

use crate::util::mutex::mutex;

unsafe extern "C" {
    pub static mut ui__lock: mutex;
    pub static mut perf_gtk_handle: *mut c_void;

    pub static mut use_browser: c_int;

    pub fn setup_browser(fallback_to_pager: bool);
    pub fn exit_browser(wait_for_ok: bool);
}

// C conditional: HAVE_SLANG_SUPPORT
#[cfg(HAVE_SLANG_SUPPORT)]
unsafe extern "C" {
    pub fn ui__init() -> c_int;
    pub fn ui__exit(wait_for_ok: bool);
}

#[cfg(not(HAVE_SLANG_SUPPORT))]
#[inline]
pub unsafe extern "C" fn ui__init() -> c_int {
    -1
}

#[cfg(not(HAVE_SLANG_SUPPORT))]
#[inline]
pub unsafe extern "C" fn ui__exit(_wait_for_ok: bool) {}

unsafe extern "C" {
    pub fn ui__refresh_dimensions(force: bool);
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn stdio__config_color(opt: *const option, mode: *const c_char, unset: c_int) -> c_int;

    pub fn pthread__block_sigwinch();
    pub fn pthread__unblock_sigwinch();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
