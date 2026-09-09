/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding architecture/kernel translation:
// asm/break.h, linux/stringify.h, linux/objtool.h, and asm-generic/bug.h.

// When CONFIG_DEBUG_BUGVERBOSE is not enabled, no location data is emitted.
// With CONFIG_DEBUG_BUGVERBOSE, the corresponding assembly emits the source
// file string and line number in the bug table.

// When CONFIG_GENERIC_BUG is not enabled, no generic bug-table entry is
// emitted. Otherwise the corresponding assembly emits the entry and optional
// verbose location data.

#[macro_export]
macro_rules! asm_bug_flags {
    ($cond_str:expr, $flags:expr) => {
        __bug_entry!($cond_str, $flags);
        // C: break BRK_BUG;
        unsafe { core::arch::asm!("break {0}", const BRK_BUG) };
    };
}

#[macro_export]
macro_rules! asm_bug {
    () => {
        asm_bug_flags!("", 0)
    };
}

#[macro_export]
macro_rules! __bug_flags {
    ($cond_str:expr, $flags:expr, $extra:expr) => {
        unsafe {
            core::arch::asm!(
                stringify!(asm_bug_flags!($cond_str, $flags)),
                options(nostack, preserves_flags),
            );
        }
        let _ = $extra;
    };
}

#[macro_export]
macro_rules! __warn_flags {
    ($cond_str:expr, $flags:expr) => {{
        instrumentation_begin!();
        __bug_flags!(
            $cond_str,
            BUGFLAG_WARNING | ($flags),
            annotate_reachable!(10001)
        );
        instrumentation_end!();
    }};
}

#[macro_export]
macro_rules! bug {
    () => {{
        instrumentation_begin!();
        __bug_flags!("", 0, "");
        unreachable!();
    }};
}

// C declaration marker: HAVE_ARCH_BUG
pub const HAVE_ARCH_BUG: bool = true;

// __BUGVERBOSE_LOCATION and __BUG_ENTRY are represented by the conditional
// assembly portions above; their external symbols and configuration are
// provided by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
