// SPDX-License-Identifier: GPL-2.0
// C dependencies: "gtk.h", <linux/compiler.h>, "../util.h"

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct perf_error_ops {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut perf_gtk_eops: perf_error_ops;
    static mut pgctx: *mut c_void;

    fn perf_error__register(ops: *mut perf_error_ops);
    fn perf_gtk__init_helpline();
    fn gtk_ui_progress__init();
    fn perf_gtk__init_hpp();
    fn gtk_init_check(argc: *mut *mut c_int, argv: *mut *mut *mut i8) -> c_int;
    fn perf_gtk__is_active_context(ctx: *mut c_void) -> bool;
    fn perf_error__unregister(ops: *mut perf_error_ops);
    fn gtk_main_quit();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_gtk__init() -> c_int {
    unsafe {
        perf_error__register(&raw mut perf_gtk_eops);
        perf_gtk__init_helpline();
        gtk_ui_progress__init();
        perf_gtk__init_hpp();

        if gtk_init_check(core::ptr::null_mut(), core::ptr::null_mut()) != 0 {
            0
        } else {
            -1
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_gtk__exit(wait_for_ok: bool) {
    let _ = wait_for_ok;

    unsafe {
        if !perf_gtk__is_active_context(pgctx) {
            return;
        }
        perf_error__unregister(&raw mut perf_gtk_eops);
        gtk_main_quit();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
