/* SPDX-License-Identifier: GPL-2.0+ */

macro_rules! __repeat_10000_3 {
    ($f:ident, $x:tt) => {
        $f!(::kernel::macros::paste!([<$x 0>]));
        $f!(::kernel::macros::paste!([<$x 1>]));
        $f!(::kernel::macros::paste!([<$x 2>]));
        $f!(::kernel::macros::paste!([<$x 3>]));
        $f!(::kernel::macros::paste!([<$x 4>]));
        $f!(::kernel::macros::paste!([<$x 5>]));
        $f!(::kernel::macros::paste!([<$x 6>]));
        $f!(::kernel::macros::paste!([<$x 7>]));
        $f!(::kernel::macros::paste!([<$x 8>]));
        $f!(::kernel::macros::paste!([<$x 9>]));
    };
}

macro_rules! __repeat_10000_2 {
    ($f:ident, $x:tt) => {
        __repeat_10000_3!($f, ::kernel::macros::paste!([<$x 0>]));
        __repeat_10000_3!($f, ::kernel::macros::paste!([<$x 1>]));
        __repeat_10000_3!($f, ::kernel::macros::paste!([<$x 2>]));
        __repeat_10000_3!($f, ::kernel::macros::paste!([<$x 3>]));
        __repeat_10000_3!($f, ::kernel::macros::paste!([<$x 4>]));
        __repeat_10000_3!($f, ::kernel::macros::paste!([<$x 5>]));
        __repeat_10000_3!($f, ::kernel::macros::paste!([<$x 6>]));
        __repeat_10000_3!($f, ::kernel::macros::paste!([<$x 7>]));
        __repeat_10000_3!($f, ::kernel::macros::paste!([<$x 8>]));
        __repeat_10000_3!($f, ::kernel::macros::paste!([<$x 9>]));
    };
}

macro_rules! __repeat_10000_1 {
    ($f:ident, $x:tt) => {
        __repeat_10000_2!($f, ::kernel::macros::paste!([<$x 0>]));
        __repeat_10000_2!($f, ::kernel::macros::paste!([<$x 1>]));
        __repeat_10000_2!($f, ::kernel::macros::paste!([<$x 2>]));
        __repeat_10000_2!($f, ::kernel::macros::paste!([<$x 3>]));
        __repeat_10000_2!($f, ::kernel::macros::paste!([<$x 4>]));
        __repeat_10000_2!($f, ::kernel::macros::paste!([<$x 5>]));
        __repeat_10000_2!($f, ::kernel::macros::paste!([<$x 6>]));
        __repeat_10000_2!($f, ::kernel::macros::paste!([<$x 7>]));
        __repeat_10000_2!($f, ::kernel::macros::paste!([<$x 8>]));
        __repeat_10000_2!($f, ::kernel::macros::paste!([<$x 9>]));
    };
}

macro_rules! repeat_10000 {
    ($f:ident) => {
        __repeat_10000_1!($f, 0);
        __repeat_10000_1!($f, 1);
        __repeat_10000_1!($f, 2);
        __repeat_10000_1!($f, 3);
        __repeat_10000_1!($f, 4);
        __repeat_10000_1!($f, 5);
        __repeat_10000_1!($f, 6);
        __repeat_10000_1!($f, 7);
        __repeat_10000_1!($f, 8);
        __repeat_10000_1!($f, 9);
    };
}

macro_rules! declare_return {
    ($i:ident) => {
        unsafe extern "C" {
            fn $i() -> ::core::ffi::c_int;
        }
    };
}

repeat_10000!(declare_return);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
