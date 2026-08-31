/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_void};

// C dependencies from <stdio.h> and <stdlib.h> used by CHECK.
unsafe extern "C" {
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn exit(status: c_int) -> !;
}

#[macro_export]
macro_rules! CHECK {
    ($condition:expr, $tag:expr, $format:expr $(, $arg:expr)* $(,)?) => {{
        let __ret: c_int = (!!$condition) as c_int;
        if __ret != 0 {
            // Rust has no exact equivalent of C's __func__; module_path!()
            // preserves the diagnostic intent available at macro expansion.
            unsafe {
                printf(
                    b"%s(%d):FAIL:%s \0".as_ptr() as *const c_char,
                    concat!(module_path!(), "\0").as_ptr() as *const c_char,
                    line!() as c_int,
                    $tag,
                );
                printf($format $(, $arg)*);
                exit(-1);
            }
        }
    }};
}

unsafe extern "C" {
    pub static mut skips: c_int;
}

pub type retry_for_error_fn = Option<unsafe extern "C" fn(err: c_int) -> bool>;

unsafe extern "C" {
    pub fn map_update_retriable(
        map_fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: c_int,
        attempts: c_int,
        need_retry: retry_for_error_fn,
    ) -> c_int;
}
