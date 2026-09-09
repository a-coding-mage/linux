/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header `asm/lse.h`.
// The included headers provide the atomic operations, alternative patching,
// CPU capability, compiler, export, and stringification dependencies used here.

/// Assembly preamble used by LSE alternatives.
pub const __LSE_PREAMBLE: &str = ".arch_extension lse\n";

/// Select the LSE implementation when the CPU capability is available, and
/// otherwise select the LL/SC implementation.
#[macro_export]
macro_rules! __lse_ll_sc_body {
    ($op:ident $(, $arg:expr)*) => {{
        if alternative_has_cap_likely(ARM64_HAS_LSE_ATOMICS) {
            __lse_$op($($arg),*)
        } else {
            __ll_sc_$op($($arg),*)
        }
    }};
}

/* In-line patching at runtime */
#[macro_export]
macro_rules! ARM64_LSE_ATOMIC_INSN {
    ($llsc:expr, $lse:expr, $cap:expr) => {
        ALTERNATIVE!($llsc, concat!(__LSE_PREAMBLE, $lse), $cap)
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
