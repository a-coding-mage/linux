/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from perf/util/include/asm/uaccess.h.
 * C header guards and preprocessor-only include structure are omitted.
 */

macro_rules! __get_user {
    ($src:expr, $dest:expr) => {{
        $src = *$dest;
        0
    }};
}

macro_rules! get_user {
    ($src:expr, $dest:expr) => {
        __get_user!($src, $dest)
    };
}

macro_rules! access_ok {
    ($addr:expr, $size:expr) => {{
        let _ = &$addr;
        let _ = &$size;
        1
    }};
}
