// SPDX-License-Identifier: GPL-2.0
// C source included <gtk/gtk.h> with GCC strict-prototypes diagnostics adjusted.

use core::ffi::{c_char, c_int};

extern "C" {
    fn gtk_init(argc: *mut c_int, argv: *mut *mut *mut c_char);
}

pub unsafe fn main(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    gtk_init(&mut argc, &mut argv);

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
