// SPDX-License-Identifier: GPL-2.0
/*
 * SHcompact irqflags support
 *
 * Copyright (C) 2006 - 2009 Paul Mundt
 */

// Dependency supplied by linux/irqflags.h.
// Build-time configuration may provide this constant.
extern "C" {
    static ARCH_IRQ_DISABLED: ::core::ffi::c_ulong;
}

#[inline(never)]
pub unsafe extern "C" fn arch_local_irq_restore(flags: ::core::ffi::c_ulong) {
    let mut dummy0: ::core::ffi::c_ulong;

    if flags == ARCH_IRQ_DISABLED {
        ::core::arch::asm!(
            "stc sr, {0}",
            "or #0xf0, {0}",
            "ldc {0}, sr",
            out(reg) dummy0,
            options(nostack, preserves_flags)
        );
    } else {
        let mut dummy1 = !ARCH_IRQ_DISABLED;
        ::core::arch::asm!(
            "stc sr, {0}",
            "and {1}, {0}",
            // CONFIG_CPU_HAS_SR_RB adds the r6_bank manipulation here.
            #[cfg(CONFIG_CPU_HAS_SR_RB)]
            "stc r6_bank, {1}",
            #[cfg(CONFIG_CPU_HAS_SR_RB)]
            "or {1}, {0}",
            "ldc {0}, sr",
            out(reg) dummy0,
            inout(reg) dummy1,
            options(nostack, preserves_flags)
        );
    }
}

// EXPORT_SYMBOL(arch_local_irq_restore);

#[inline(never)]
pub unsafe extern "C" fn arch_local_save_flags() -> ::core::ffi::c_ulong {
    let mut flags: ::core::ffi::c_ulong;

    ::core::arch::asm!(
        "stc sr, {0}",
        "and #0xf0, {0}",
        out(reg) flags,
        options(nostack, preserves_flags)
    );

    flags
}

// EXPORT_SYMBOL(arch_local_save_flags);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
