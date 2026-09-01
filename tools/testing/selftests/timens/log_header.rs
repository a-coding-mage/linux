/* SPDX-License-Identifier: GPL-2.0 */

#[macro_export]
macro_rules! pr_msg {
    ($fmt:literal, $lvl:expr $(, $arg:expr)* $(,)?) => {
        ksft_print_msg!(
            concat!("[%s] (%s:%d)\t", $fmt, "\n"),
            $lvl,
            file!(),
            line!()
            $(, $arg)*
        )
    };
}

#[macro_export]
macro_rules! pr_p {
    ($func:ident, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $func!(concat!($fmt, ": %m") $(, $arg)*)
    };
}

#[macro_export]
macro_rules! pr_err {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        ksft_test_result_error!(concat!($fmt, "\n") $(, $arg)*);
        -1
    }};
}

#[macro_export]
macro_rules! pr_fail {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        ksft_test_result_fail!($fmt $(, $arg)*);
        -1
    }};
}

#[macro_export]
macro_rules! pr_perror {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        pr_p!(pr_err, $fmt $(, $arg)*)
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
