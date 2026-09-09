/*
 * Xtensa IRQ flags handling functions
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 * Copyright (C) 2015 Cadence Design Systems Inc.
 */

// Translated from the C header.  The following names are supplied by the
// corresponding Xtensa processor and Linux type definitions.

#[inline]
pub unsafe fn arch_local_save_flags() -> usize {
    let flags: usize;
    core::arch::asm!("rsr {0}, ps", out(reg) flags);
    flags
}

#[inline]
pub unsafe fn arch_local_irq_save() -> usize {
    let flags: usize;

    // C build condition: XTENSA_FAKE_NMI.
    #[cfg(feature = "xtensa_fake_nmi")]
    {
        // C build condition: CONFIG_DEBUG_MISC &&
        // (LOCKLEVEL | TOPLEVEL) >= XCHAL_DEBUGLEVEL.
        #[cfg(feature = "config_debug_misc")]
        {
            let tmp: usize;
            core::arch::asm!(
                "rsr {0}, ps\n\t\
                 extui {1}, {0}, 0, 4\n\t\
                 bgei {1}, {2}, 1f\n\t\
                 rsil {0}, {2}\n\t\
                 1:",
                out(reg) flags,
                out(reg) tmp,
                const LOCKLEVEL,
                options(nostack)
            );
        }
        #[cfg(not(feature = "config_debug_misc"))]
        {
            core::arch::asm!(
                "rsr {0}, ps\n\t\
                 or {0}, {0}, {1}\n\t\
                 xsr {0}, ps\n\t\
                 rsync",
                out(reg) flags,
                in(reg) LOCKLEVEL,
                options(nostack)
            );
        }
    }

    #[cfg(not(feature = "xtensa_fake_nmi"))]
    {
        core::arch::asm!("rsil {0}, {1}", out(reg) flags, const LOCKLEVEL);
    }

    flags
}

#[inline]
pub unsafe fn arch_local_irq_disable() {
    arch_local_irq_save();
}

#[inline]
pub unsafe fn arch_local_irq_enable() {
    let flags: usize;
    core::arch::asm!("rsil {0}, 0", out(reg) flags, options(nostack));
}

#[inline]
pub unsafe fn arch_local_irq_restore(flags: usize) {
    core::arch::asm!("wsr {0}, ps; rsync", in(reg) flags, options(nostack));
}

#[inline]
pub unsafe fn arch_irqs_disabled_flags(flags: usize) -> bool {
    // C compile-time condition:
    // XCHAL_EXCM_LEVEL < LOCKLEVEL || (1 << PS_EXCM_BIT) < LOCKLEVEL
    // emits an error if true.
    (flags & (PS_INTLEVEL_MASK | (1usize << PS_EXCM_BIT))) >= LOCKLEVEL
}

#[inline]
pub unsafe fn arch_irqs_disabled() -> bool {
    arch_irqs_disabled_flags(arch_local_save_flags())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
