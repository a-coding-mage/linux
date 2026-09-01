// SPDX-License-Identifier: GPL-2.0-only

// C dependency intent: #include <uapi/linux/const.h>
// Expects __ALIGN_KERNEL! to be supplied by the translated dependency.

macro_rules! ALIGN {
    ($x:expr, $a:expr) => {
        __ALIGN_KERNEL!(($x), ($a))
    };
}

macro_rules! ALIGN_DOWN {
    ($x:expr, $a:expr) => {
        __ALIGN_KERNEL!(($x) - (($a) - 1), ($a))
    };
}

macro_rules! IS_ALIGNED {
    ($x:expr, $a:expr) => {{
        let x = $x;
        ((x) & (($a as _) - 1)) == 0
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
