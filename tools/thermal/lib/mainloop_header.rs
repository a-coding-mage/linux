/* SPDX-License-Identifier: LGPL-2.1+ */
/* Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org> */

use core::ffi::c_void;

pub type mainloop_callback_t = Option<unsafe extern "C" fn(fd: i32, data: *mut c_void) -> i32>;

unsafe extern "C" {
    pub fn mainloop(timeout: u32) -> i32;
    pub fn mainloop_add(fd: i32, cb: mainloop_callback_t, data: *mut c_void) -> i32;
    pub fn mainloop_del(fd: i32) -> i32;
    pub fn mainloop_exit();
    pub fn mainloop_init() -> i32;
    pub fn mainloop_fini();
}
