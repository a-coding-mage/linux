// SPDX-License-Identifier: GPL-2.0

/*
 * Indirect stringification.  Doing two levels allows the parameter to be a
 * macro itself.  For example, compile with -DFOO=bar, __stringify(FOO)
 * converts to "bar".
 */

macro_rules! __stringify_1 {
    ($($x:tt)*) => {
        stringify!($($x)*)
    };
}

macro_rules! __stringify {
    ($($x:tt)*) => {
        __stringify_1!($($x)*)
    };
}
