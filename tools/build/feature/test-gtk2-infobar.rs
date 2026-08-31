// SPDX-License-Identifier: GPL-2.0
// C source used GCC diagnostics around <gtk/gtk.h> for -Wstrict-prototypes.

use std::os::raw::{c_char, c_int, c_void};

unsafe extern "C" {
    fn gtk_init(argc: *mut c_int, argv: *mut *mut *mut c_char);
    fn gtk_info_bar_new() -> *mut c_void;
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| {
            std::ffi::CString::new(arg)
                .expect("argument contains interior NUL byte")
                .into_raw()
        })
        .collect();
    args.push(std::ptr::null_mut());

    let mut argc = (args.len() - 1) as c_int;
    let mut argv = args.as_mut_ptr();

    unsafe {
        gtk_init(&mut argc, &mut argv);
        gtk_info_bar_new();
    }

    for arg in args.into_iter().take(argc as usize) {
        if !arg.is_null() {
            unsafe {
                let _ = std::ffi::CString::from_raw(arg);
            }
        }
    }
}
