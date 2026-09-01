/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translation of arch/x86/include/asm/rmwcc.h.
 *
 * The original header defines C preprocessor helpers around GCC asm goto.
 * Rust has no direct file-local equivalent for the C preprocessor's string
 * concatenation and asm-goto label operand syntax, so these are preserved as
 * macro helpers with the same externally visible names and return behavior.
 */

#[macro_export]
macro_rules! __GEN_RMWcc {
    ($fullop:expr, $var:expr, $cc:expr $(, $args:tt)*) => {{
        /*
         * C original:
         * asm goto (fullop "; j" cc " %l[cc_label]"
         *           : : "m" (var), ## __VA_ARGS__
         *           : "memory" : cc_label);
         * return 0;
         * cc_label:
         * return 1;
         */
        unsafe {
            core::arch::asm!(
                concat!($fullop, "; j", $cc, " {cc_label}"),
                in("m") $var
                $(, $args)*,
                options(nostack),
                cc_label = label {
                    return 1;
                },
            );
        }
        return 0;
    }};
}

#[macro_export]
macro_rules! GEN_UNARY_RMWcc {
    ($op:expr, $var:expr, $arg0:expr, $cc:expr) => {
        $crate::__GEN_RMWcc!(concat!($op, " ", $arg0), $var, $cc)
    };
}

#[macro_export]
macro_rules! GEN_BINARY_RMWcc {
    ($op:expr, $var:expr, $vcon:ident, $val:expr, $arg0:expr, $cc:expr) => {
        $crate::__GEN_RMWcc!(concat!($op, " %1, ", $arg0), $var, $cc, $vcon($val))
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
