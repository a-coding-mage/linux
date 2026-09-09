/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2010 Thomas Chou <thomas@wytron.com.tw>
 */

// Dependency supplied by asm/registers.h in the original header:
// RDCTL, WRCTL, CTL_FSTATUS, and STATUS_PIE.

pub static inline fn arch_local_save_flags() -> usize {
    unsafe { RDCTL(CTL_FSTATUS) }
}

/*
 * This will restore ALL status register flags, not only the interrupt
 * mask flag.
 */
pub static inline fn arch_local_irq_restore(flags: usize) {
    unsafe { WRCTL(CTL_FSTATUS, flags) };
}

pub static inline fn arch_local_irq_disable() {
    let flags: usize;

    flags = arch_local_save_flags();
    arch_local_irq_restore(flags & !STATUS_PIE);
}

pub static inline fn arch_local_irq_enable() {
    let flags: usize;

    flags = arch_local_save_flags();
    arch_local_irq_restore(flags | STATUS_PIE);
}

pub static inline fn arch_irqs_disabled_flags(flags: usize) -> i32 {
    ((flags & STATUS_PIE) == 0) as i32
}

pub static inline fn arch_irqs_disabled() -> i32 {
    arch_irqs_disabled_flags(arch_local_save_flags())
}

pub static inline fn arch_local_irq_save() -> usize {
    let flags: usize;

    flags = arch_local_save_flags();
    arch_local_irq_restore(flags & !STATUS_PIE);
    flags
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
