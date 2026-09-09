/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux::compiler_types, linux::stringify, asm::alternative,
// asm::alternative_macros, and asm::cpucaps.

pub const __LSUI_PREAMBLE: &str = ".arch_extension lsui\n";

// CONFIG_ARM64_LSUI selects the capability-aware implementation at build time.
// The referenced functions and ARM64_HAS_LSUI are supplied externally.
#[cfg(feature = "CONFIG_ARM64_LSUI")]
#[macro_export]
macro_rules! __lsui_llsc_body {
    ($op:ident, $($args:expr),* $(,)?) => {{
        if alternative_has_cap_unlikely(ARM64_HAS_LSUI) {
            __paste_lsui_operation!($op, $($args),*)
        } else {
            __paste_llsc_operation!($op, $($args),*)
        }
    }};
}

#[cfg(not(feature = "CONFIG_ARM64_LSUI"))]
#[macro_export]
macro_rules! __lsui_llsc_body {
    ($op:ident, $($args:expr),* $(,)?) => {{
        __paste_llsc_operation!($op, $($args),*)
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
