// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2023 SUSE
 * Authors: Libor Pechacek <lpechacek@suse.cz>
 *          Marcos Paulo de Souza <mpdesouza@suse.com>
 */

use std::ffi::{c_char, c_int, c_long};

unsafe extern "C" {
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
    fn syscall(number: c_long, ...) -> c_long;
    fn printf(format: *const c_char, ...) -> c_int;
}

const SIGHUP: c_int = 1;
const SIGINT: c_int = 2;
const SYS_GETPID: c_long = 39;

static mut stop: c_int = 0;
static mut sig_int: c_int = 0;

extern "C" fn hup_handler(_signum: c_int) {
    unsafe {
        stop = 1;
    }
}

extern "C" fn int_handler(_signum: c_int) {
    unsafe {
        stop = 1;
        sig_int = 1;
    }
}

unsafe fn main_0(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut count: c_long = 0;

    signal(SIGHUP, hup_handler);
    signal(SIGINT, int_handler);

    while stop == 0 {
        syscall(SYS_GETPID);
        count += 1;
    }

    if sig_int != 0 {
        printf(c"%ld iterations done\n".as_ptr(), count);
    }

    0
}

fn main() {
    unsafe {
        main_0(0, std::ptr::null_mut());
    }
}
