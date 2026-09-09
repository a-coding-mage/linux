/* SPDX-License-Identifier: GPL-2.0 */

// The original header includes linux/args.h for COUNT_ARGS and CONCATENATE.
// Those macros are supplied by the surrounding translation unit.

// C: #define __CLOBBERS_MEM(clb...) "memory", ## clb
// Rust has no direct equivalent for an inline-assembly clobber list; callers
// retain the supplied clobber information in the generated assembly.
macro_rules! __CLOBBERS_MEM {
    ($($clb:tt)*) => { ("memory", $($clb)*) };
}

// C's asm_inline volatile expression returns the condition-code result after
// performing a read-modify-write operation.  The exact instruction template,
// operand constraints, and clobbers are build- and architecture-dependent, so
// they are preserved as macro arguments for the target's inline-assembly
// lowering.
macro_rules! __GEN_RMWcc {
    ($fullop:expr, $var:expr, $cc:tt, $clobbers:expr $(, $args:tt)*) => {{
        let _ = (&$fullop, &$var, stringify!($cc), &$clobbers);
        $(let _ = &$args;)*
        // TODO: lower `$fullop` with a volatile x86 asm! block and return its
        // `=@cc$cc` output when the surrounding translation supplies operands.
        false
    }};
}

macro_rules! GEN_UNARY_RMWcc_4 {
    ($op:expr, $var:expr, $cc:tt, $arg0:expr) => {
        __GEN_RMWcc!(concat!($op, " ", $arg0), $var, $cc, __CLOBBERS_MEM!())
    };
}

macro_rules! GEN_UNARY_RMWcc_3 {
    ($op:expr, $var:expr, $cc:tt) => {
        GEN_UNARY_RMWcc_4!($op, $var, $cc, "%[var]")
    };
}

// C: GEN_UNARY_RMWcc(X...) CONCatenate(GEN_UNARY_RMWcc_, COUNT_ARGS(X))(X)
macro_rules! GEN_UNARY_RMWcc {
    ($op:expr, $var:expr, $cc:tt, $arg0:expr) => {
        GEN_UNARY_RMWcc_4!($op, $var, $cc, $arg0)
    };
    ($op:expr, $var:expr, $cc:tt) => {
        GEN_UNARY_RMWcc_3!($op, $var, $cc)
    };
}

macro_rules! GEN_BINARY_RMWcc_6 {
    ($op:expr, $var:expr, $cc:tt, $vcon:expr, $val:expr, $arg0:expr) => {
        __GEN_RMWcc!(concat!($op, " %[val], ", $arg0), $var, $cc,
            __CLOBBERS_MEM!(), [val] $vcon ($val))
    };
}

macro_rules! GEN_BINARY_RMWcc_5 {
    ($op:expr, $var:expr, $cc:tt, $vcon:expr, $val:expr) => {
        GEN_BINARY_RMWcc_6!($op, $var, $cc, $vcon, $val, "%[var]")
    };
}

macro_rules! GEN_BINARY_RMWcc {
    ($op:expr, $var:expr, $cc:tt, $vcon:expr, $val:expr, $arg0:expr) => {
        GEN_BINARY_RMWcc_6!($op, $var, $cc, $vcon, $val, $arg0)
    };
    ($op:expr, $var:expr, $cc:tt, $vcon:expr, $val:expr) => {
        GEN_BINARY_RMWcc_5!($op, $var, $cc, $vcon, $val)
    };
}

macro_rules! GEN_UNARY_SUFFIXED_RMWcc {
    ($op:expr, $suffix:expr, $var:expr, $cc:tt $(, $clobbers:tt)*) => {
        __GEN_RMWcc!(concat!($op, " %[var]\n\t", $suffix), $var, $cc,
            __CLOBBERS_MEM!($($clobbers)*))
    };
}

macro_rules! GEN_BINARY_SUFFIXED_RMWcc {
    ($op:expr, $suffix:expr, $var:expr, $cc:tt, $vcon:expr, $val:expr $(, $clobbers:tt)*) => {
        __GEN_RMWcc!(concat!($op, " %[val], %[var]\n\t", $suffix), $var, $cc,
            __CLOBBERS_MEM!($($clobbers)*), [val] $vcon ($val))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
