/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the architecture barrier header.
// The C header includes <asm-generic/barrier.h>; its declarations are supplied
// by the corresponding Rust dependency.

/// Memory barrier: emit the OpenRISC `l.msync` instruction and clobber memory.
#[macro_export]
macro_rules! mb {
    () => {{
        unsafe {
            core::arch::asm!("l.msync", options(nostack));
        }
    }};
}

/// No-op: emit the OpenRISC `l.nop` instruction.
#[macro_export]
macro_rules! nop {
    () => {{
        unsafe {
            core::arch::asm!("l.nop", options(nostack));
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
