// SPDX-License-Identifier: GPL-2.0-only

// Translated from the C preprocessor macro. The declarations and implementation
// of `instance`, `large_src`, and `strscpy` are supplied by the included test
// support in the original source.
macro_rules! TEST {
    () => {
        strscpy(
            instance.buf,
            large_src,
            core::mem::size_of_val(&instance.buf) + 1,
        )
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
