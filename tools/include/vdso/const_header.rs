/* SPDX-License-Identifier: GPL-2.0 */

/* Depends on definitions corresponding to <uapi/linux/const.h>. */

macro_rules! UL {
    ($x:expr) => {
        _UL!($x)
    };
}

macro_rules! ULL {
    ($x:expr) => {
        _ULL!($x)
    };
}
