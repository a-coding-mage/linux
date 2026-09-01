/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies removed from executable Rust:
// - <linux/compiler.h> supplied unlikely()
// - <stdio.h> supplied fprintf(stderr, ...)

#[macro_export]
macro_rules! __WARN_printf {
    ($($arg:tt)*) => {{
        eprint!($($arg)*);
    }};
}

#[macro_export]
macro_rules! WARN {
    ($condition:expr, $($format:tt)*) => {{
        let __ret_warn_on: i32 = if $condition { 1 } else { 0 };
        if __ret_warn_on != 0 {
            $crate::__WARN_printf!($($format)*);
        }
        __ret_warn_on != 0
    }};
}

#[macro_export]
macro_rules! WARN_ON {
    ($condition:expr) => {{
        let __ret_warn_on: i32 = if $condition { 1 } else { 0 };
        if __ret_warn_on != 0 {
            $crate::__WARN_printf!(
                "assertion failed at {}:{}\n",
                file!(),
                line!()
            );
        }
        __ret_warn_on != 0
    }};
}

#[macro_export]
macro_rules! WARN_ON_ONCE {
    ($condition:expr) => {{
        static mut __warned: i32 = 0;
        let __ret_warn_once: i32 = if $condition { 1 } else { 0 };

        unsafe {
            if __ret_warn_once != 0 && __warned == 0 {
                __warned = true as i32;
                $crate::WARN_ON!(true);
            }
        }
        __ret_warn_once != 0
    }};
}

#[macro_export]
macro_rules! WARN_ONCE {
    ($condition:expr, $($format:tt)*) => {{
        static mut __warned: i32 = 0;
        let __ret_warn_once: i32 = if $condition { 1 } else { 0 };

        unsafe {
            if __ret_warn_once != 0 {
                if $crate::WARN!(__warned == 0, $($format)*) {
                    __warned = 1;
                }
            }
        }
        __ret_warn_once != 0
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
