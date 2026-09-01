/* SPDX-License-Identifier: GPL-2.0-or-later */

/* From <objtool/warn.h>: ERROR_GLIBC and ERROR are external dependencies. */

macro_rules! snprintf_check {
    ($str:expr, $size:expr, $format:expr, $($args:expr),+ $(,)?) => {{
        let mut __ret = unsafe { snprintf($str, $size, $format, $($args),+) };

        if __ret < 0 {
            ERROR_GLIBC("snprintf");
        } else if (__ret as usize) >= ($size as usize) {
            ERROR(concat!("snprintf() failed for '", $format, "'"), $($args),+);
        } else {
            __ret = 0;
        }

        __ret
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
