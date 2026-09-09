/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

// C header guard: _ASM_RISCV_IRQFLAGS_H
// Dependency supplied externally: <asm/csr.h>

/* read interrupt enabled status */
#[inline]
pub unsafe fn arch_local_save_flags() -> usize {
    csr_read(CSR_STATUS)
}

/* unconditionally enable interrupts */
#[inline]
pub unsafe fn arch_local_irq_enable() {
    csr_set(CSR_STATUS, SR_IE);
}

/* unconditionally disable interrupts */
#[inline]
pub unsafe fn arch_local_irq_disable() {
    csr_clear(CSR_STATUS, SR_IE);
}

/* get status and disable interrupts */
#[inline]
pub unsafe fn arch_local_irq_save() -> usize {
    csr_read_clear(CSR_STATUS, SR_IE)
}

/* test flags */
#[inline]
pub unsafe fn arch_irqs_disabled_flags(flags: usize) -> i32 {
    if (flags & SR_IE) == 0 { 1 } else { 0 }
}

/* test hardware interrupt enable bit */
#[inline]
pub unsafe fn arch_irqs_disabled() -> i32 {
    arch_irqs_disabled_flags(arch_local_save_flags())
}

/* set interrupt enabled status */
#[inline]
pub unsafe fn arch_local_irq_restore(flags: usize) {
    csr_set(CSR_STATUS, flags & SR_IE);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
