/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_void};

/* Forward declarations/dependencies supplied by other translated files. */
pub enum maps {}
pub enum option {}
pub enum perf_sample {}
pub enum thread {}

#[repr(C)]
pub struct unwind_entry {
    pub ms: map_symbol,
    pub ip: u64,
}

pub type unwind_entry_cb_t =
    Option<unsafe extern "C" fn(entry: *mut unwind_entry, arg: *mut c_void) -> c_int>;

unsafe extern "C" {
    /* From "debug.h"; pr_warning_once is macro-like in C and an external dependency here. */
    fn pr_warning_once(fmt: *const c_char);

    pub fn unwind__configure(var: *const c_char, value: *const c_char, cb: *mut c_void) -> c_int;
    pub fn unwind__option(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;

    /*
     * When best_effort is set, don't report errors and fail silently. This could
     * be expanded in the future to be more permissive about things other than
     * error messages.
     */
    pub fn unwind__get_entries(
        cb: unwind_entry_cb_t,
        arg: *mut c_void,
        thread: *mut thread,
        data: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
    ) -> c_int;
}

/* Original C condition: #ifdef HAVE_LIBDW_SUPPORT. */
#[cfg(HAVE_LIBDW_SUPPORT)]
unsafe extern "C" {
    pub fn libdw__get_entries(
        cb: unwind_entry_cb_t,
        arg: *mut c_void,
        thread: *mut thread,
        data: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
    ) -> c_int;
}

/* Original C fallback condition: #ifndef HAVE_LIBDW_SUPPORT. */
#[cfg(not(HAVE_LIBDW_SUPPORT))]
pub unsafe fn libdw__get_entries(
    cb: unwind_entry_cb_t,
    arg: *mut c_void,
    thread: *mut thread,
    data: *mut perf_sample,
    max_stack: c_int,
    best_effort: bool,
) -> c_int {
    let _ = cb;
    let _ = arg;
    let _ = thread;
    let _ = data;
    let _ = max_stack;
    let _ = best_effort;

    pr_warning_once(
        b"Error: libdw dwarf unwinding not built into perf\n\0".as_ptr() as *const c_char,
    );
    0
}

/* Original C condition: #ifdef HAVE_LIBUNWIND_SUPPORT. */
#[cfg(HAVE_LIBUNWIND_SUPPORT)]
unsafe extern "C" {
    /* libunwind specific */
    pub fn libunwind__get_entries(
        cb: unwind_entry_cb_t,
        arg: *mut c_void,
        thread: *mut thread,
        data: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
    ) -> c_int;
    pub fn unwind__prepare_access(maps: *mut maps, e_machine: u16) -> c_int;
    pub fn unwind__flush_access(maps: *mut maps);
    pub fn unwind__finish_access(maps: *mut maps);
}

/* Original C fallback condition: #ifndef HAVE_LIBUNWIND_SUPPORT. */
#[cfg(not(HAVE_LIBUNWIND_SUPPORT))]
pub unsafe fn libunwind__get_entries(
    cb: unwind_entry_cb_t,
    arg: *mut c_void,
    thread: *mut thread,
    data: *mut perf_sample,
    max_stack: c_int,
    best_effort: bool,
) -> c_int {
    let _ = cb;
    let _ = arg;
    let _ = thread;
    let _ = data;
    let _ = max_stack;
    let _ = best_effort;

    pr_warning_once(
        b"Error: libunwind dwarf unwinding not built into perf\n\0".as_ptr() as *const c_char,
    );
    0
}

#[cfg(not(HAVE_LIBUNWIND_SUPPORT))]
pub unsafe fn unwind__prepare_access(maps: *mut maps, e_machine: u16) -> c_int {
    let _ = maps;
    let _ = e_machine;
    0
}

#[cfg(not(HAVE_LIBUNWIND_SUPPORT))]
pub unsafe fn unwind__flush_access(maps: *mut maps) {
    let _ = maps;
}

#[cfg(not(HAVE_LIBUNWIND_SUPPORT))]
pub unsafe fn unwind__finish_access(maps: *mut maps) {
    let _ = maps;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
