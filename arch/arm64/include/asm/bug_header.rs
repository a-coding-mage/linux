/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015  ARM Limited
 * Author: Dave Martin <Dave.Martin@arm.com>
 */

// The C header includes `linux/stringify.h` and `asm/asm-bug.h`; their
// definitions are supplied by the surrounding translation.

/// Emit the architecture-specific bug instruction and its flags.
#[macro_export]
macro_rules! __BUG_FLAGS {
    ($flags:expr) => {{
        unsafe {
            core::arch::asm!(stringify!(ASM_BUG_FLAGS($flags)));
        }
    }};
}

/// Trigger an unconditional bug and mark the following code unreachable.
#[macro_export]
macro_rules! BUG {
    () => {{
        $crate::__BUG_FLAGS!(0);
        ::core::unreachable!();
    }};
}

/// Emit a warning bug record. `cond_str` is retained to match the C interface.
#[macro_export]
macro_rules! __WARN_FLAGS {
    ($cond_str:expr, $flags:expr) => {{
        $crate::__BUG_FLAGS!(BUGFLAG_WARNING | ($flags));
    }};
}

// HAVE_ARCH_BUG
// The generic bug declarations from `asm-generic/bug.h` are supplied by the
// surrounding translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
