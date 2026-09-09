/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the translated kernel headers:
// linux/types.h, linux/preempt.h, asm/thread_info.h, and asm/entry.h.

#[inline]
pub unsafe fn arch_local_save_flags() -> usize {
    let mut flags: usize;
    core::arch::asm!("movew %sr, {0}", out(reg) flags, options(nostack, preserves_flags));
    flags
}

#[inline]
pub unsafe fn arch_local_irq_disable() {
    // CONFIG_COLDFIRE selects the following implementation.
    #[cfg(feature = "CONFIG_COLDFIRE")]
    core::arch::asm!(
        "move %sr, %d0",
        "ori.l #0x0700, %d0",
        "move %d0, %sr",
        options(nostack)
    );

    // Non-ColdFire implementation.
    #[cfg(not(feature = "CONFIG_COLDFIRE"))]
    core::arch::asm!("oriw #0x0700, %sr", options(nostack));
}

#[inline]
pub unsafe fn arch_local_irq_enable() {
    // CONFIG_COLDFIRE selects the following implementation.
    #[cfg(feature = "CONFIG_COLDFIRE")]
    core::arch::asm!(
        "move %sr, %d0",
        "andi.l #0xf8ff, %d0",
        "move %d0, %sr",
        options(nostack)
    );

    // On CONFIG_MMU builds, the non-ColdFire instruction is executed only
    // when MACH_IS_Q40 || !hardirq_count().
    #[cfg(not(feature = "CONFIG_COLDFIRE"))]
    {
        #[cfg(feature = "CONFIG_MMU")]
        if MACH_IS_Q40 || unsafe { !hardirq_count() } {
            core::arch::asm!("andiw {0}, %sr", const ALLOWINT, options(nostack));
        }

        #[cfg(not(feature = "CONFIG_MMU"))]
        core::arch::asm!("andiw {0}, %sr", const ALLOWINT, options(nostack));
    }
}

#[inline]
pub unsafe fn arch_local_irq_save() -> usize {
    let flags = arch_local_save_flags();
    arch_local_irq_disable();
    flags
}

#[inline]
pub unsafe fn arch_local_irq_restore(flags: usize) {
    core::arch::asm!("movew {0}, %sr", in(reg) flags, options(nostack));
}

#[inline]
pub unsafe fn arch_irqs_disabled_flags(flags: usize) -> bool {
    if MACH_IS_ATARI {
        /* Ignore HSYNC = ipl 2 on Atari */
        return (flags & !(ALLOWINT | 0x200)) != 0;
    }
    (flags & !ALLOWINT) != 0
}

#[inline]
pub unsafe fn arch_irqs_disabled() -> bool {
    arch_irqs_disabled_flags(arch_local_save_flags())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
