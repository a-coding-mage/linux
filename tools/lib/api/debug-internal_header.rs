/* SPDX-License-Identifier: GPL-2.0 */

/* Depends on debug.h for libapi_print_fn_t. */

unsafe extern "C" {
    pub static mut __pr_warn: libapi_print_fn_t;
    pub static mut __pr_info: libapi_print_fn_t;
    pub static mut __pr_debug: libapi_print_fn_t;
}

macro_rules! __pr {
    ($func:expr, $fmt:literal $(, $args:expr)* $(,)?) => {
        unsafe {
            if let Some(func) = $func {
                func(concat!("libapi: ", $fmt) $(, $args)*);
            }
        }
    };
}

macro_rules! pr_warn {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        __pr!(__pr_warn, $fmt $(, $args)*)
    };
}

macro_rules! pr_info {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        __pr!(__pr_info, $fmt $(, $args)*)
    };
}

macro_rules! pr_debug {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        __pr!(__pr_debug, $fmt $(, $args)*)
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
