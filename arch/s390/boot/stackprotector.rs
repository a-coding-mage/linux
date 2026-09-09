// SPDX-License-Identifier: GPL-2.0

/// Prefix a format string with the stack-protector boot diagnostic label.
macro_rules! boot_fmt {
    ($fmt:literal) => {
        concat!("stackprot: ", $fmt)
    };
}

// The C source includes `boot.h` and the shared kernel stack-protector
// implementation. Their declarations and implementation are supplied by
// other translation units.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
