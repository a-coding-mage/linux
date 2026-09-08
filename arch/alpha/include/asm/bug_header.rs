/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: linux/linkage.h

// CONFIG_BUG is a build-time condition from the original header.  The Rust
// equivalent is retained here as a feature gate.
#[cfg(feature = "CONFIG_BUG")]
mod config_bug {
    // Dependency intent: asm/pal.h supplies PAL_bugchk.

    /* ??? Would be nice to use .gprel32 here, but we can't be sure that the
       function loaded the GP, so this could fail in modules.  */
    #[macro_export]
    macro_rules! BUG {
        () => {{
            unsafe {
                core::arch::asm!(
                    "call_pal {pal_bugchk}  # bugchk\n\t",
                    ".long {line}\n\t.8byte {file}",
                    pal_bugchk = const PAL_bugchk,
                    line = const line!(),
                    file = const file!(),
                    options(nostack, preserves_flags)
                );
                core::hint::unreachable_unchecked();
            }
        }};
    }

    // Marker corresponding to HAVE_ARCH_BUG.
    pub const HAVE_ARCH_BUG: bool = true;
}

// Dependency intent: asm-generic/bug.h

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
