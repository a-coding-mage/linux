// SPDX-License-Identifier: GPL-2.0-or-later
//
// C compiler variadic-argument builtins are supplied by the surrounding
// translation environment.

pub type va_list = __builtin_va_list;

macro_rules! va_start {
    ($v:expr, $l:expr) => {
        __builtin_va_start!($v, $l)
    };
}

macro_rules! va_end {
    ($v:expr) => {
        __builtin_va_end!($v)
    };
}

macro_rules! va_arg {
    ($v:expr, $T:ty) => {
        __builtin_va_arg!($v, $T)
    };
}

macro_rules! va_copy {
    ($d:expr, $s:expr) => {
        __builtin_va_copy!($d, $s)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
