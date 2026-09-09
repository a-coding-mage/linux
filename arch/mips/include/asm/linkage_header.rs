/* SPDX-License-Identifier: GPL-2.0 */

// The original header includes <asm/asm.h> only for assembler sources.
// Rust has no direct equivalent for this C preprocessor condition.

/// Define a weak system-call alias for `x` that resolves to `sys_ni_syscall`.
#[macro_export]
macro_rules! cond_syscall {
    ($x:ident) => {
        core::arch::global_asm!(concat!(
            ".weak\t", stringify!($x), "\n",
            stringify!($x), "\t=\tsys_ni_syscall"
        ));
    };
}

/// Define `alias` as an assembler alias for `name` and export it globally.
#[macro_export]
macro_rules! SYSCALL_ALIAS {
    ($alias:ident, $name:ident) => {
        core::arch::global_asm!(concat!(
            stringify!($alias), " = ", stringify!($name),
            "\n\t.globl ", stringify!($alias)
        ));
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
