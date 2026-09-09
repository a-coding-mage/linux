/* SPDX-License-Identifier: GPL-2.0-only */

// Rust translation of the non-assembler portion of asm/vdso/processor.h.
// Dependencies supplied by asm/cputable.h and asm/feature-fixups.h remain
// external to this translation.

/* Macros for adjusting thread priority (hardware multi-threading) */
#[cfg(CONFIG_PPC64)]
#[macro_export]
macro_rules! HMT_very_low {
    () => {{ unsafe { core::arch::asm!("or 31, 31, 31 // very low priority", options(nostack, preserves_flags)); } }};
}

#[cfg(CONFIG_PPC64)]
#[macro_export]
macro_rules! HMT_low {
    () => {{ unsafe { core::arch::asm!("or 1, 1, 1 // low priority", options(nostack, preserves_flags)); } }};
}

#[cfg(CONFIG_PPC64)]
#[macro_export]
macro_rules! HMT_medium_low {
    () => {{ unsafe { core::arch::asm!("or 6, 6, 6 // medium low priority", options(nostack, preserves_flags)); } }};
}

#[cfg(CONFIG_PPC64)]
#[macro_export]
macro_rules! HMT_medium {
    () => {{ unsafe { core::arch::asm!("or 2, 2, 2 // medium priority", options(nostack, preserves_flags)); } }};
}

#[cfg(CONFIG_PPC64)]
#[macro_export]
macro_rules! HMT_medium_high {
    () => {{ unsafe { core::arch::asm!("or 5, 5, 5 // medium high priority", options(nostack, preserves_flags)); } }};
}

#[cfg(CONFIG_PPC64)]
#[macro_export]
macro_rules! HMT_high {
    () => {{ unsafe { core::arch::asm!("or 3, 3, 3 // high priority", options(nostack, preserves_flags)); } }};
}

#[cfg(not(CONFIG_PPC64))]
#[macro_export]
macro_rules! HMT_very_low { () => {}; }

#[cfg(not(CONFIG_PPC64))]
#[macro_export]
macro_rules! HMT_low { () => {}; }

#[cfg(not(CONFIG_PPC64))]
#[macro_export]
macro_rules! HMT_medium_low { () => {}; }

#[cfg(not(CONFIG_PPC64))]
#[macro_export]
macro_rules! HMT_medium { () => {}; }

#[cfg(not(CONFIG_PPC64))]
#[macro_export]
macro_rules! HMT_medium_high { () => {}; }

#[cfg(not(CONFIG_PPC64))]
#[macro_export]
macro_rules! HMT_high { () => {}; }

#[cfg(CONFIG_PPC64)]
#[macro_export]
macro_rules! cpu_relax {
    () => {{
        // ASM_FTR_IFCLR:
        // Pre-POWER10 uses low ; medium priority nops; POWER10 onward uses
        // pause_short (wait 2,0), selected by CPU_FTR_ARCH_31.
        unsafe {
            core::arch::asm!(
                "1: or 1,1,1 ; or 2,2,2\n\t",
                "2: .long {wait_instruction}",
                wait_instruction = const 0,
                options(nostack)
            );
        }
    }};
}

#[cfg(not(CONFIG_PPC64))]
#[macro_export]
macro_rules! cpu_relax {
    () => {{
        // C barrier() dependency supplied by the surrounding translation.
        $crate::barrier();
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
