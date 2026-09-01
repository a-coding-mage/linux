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

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
