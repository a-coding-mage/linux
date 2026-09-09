/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: _X86_IRQFLAGS_H_ */
/* Dependency: <asm/processor-flags.h> */

/* The C __ASSEMBLER__ and CONFIG_PARAVIRT conditions are preserved here as
 * source-level intent; this translation contains the non-assembler routines. */

use core::arch::asm;

extern "C" {
    fn x86_idle_clear_cpu_buffers();
}

/* Dependency: X86_EFLAGS_IF is supplied by asm/processor-flags.h. */

/// Save the processor flags.
pub unsafe fn native_save_fl() -> usize {
    let flags: usize;

    /*
     * "=rm" is safe here, because "pop" adjusts the stack before
     * it evaluates its effective address -- this is part of the
     * documented behavior of the "pop" instruction.
     */
    asm!(
        "# __raw_save_flags",
        "pushf",
        "pop {0}",
        out(reg) flags,
        options(nostack, preserves_flags)
    );

    flags
}

pub unsafe fn native_irq_disable() {
    asm!("cli", options(nostack, preserves_flags));
}

pub unsafe fn native_irq_enable() {
    asm!("sti", options(nostack, preserves_flags));
}

pub unsafe fn native_safe_halt() {
    x86_idle_clear_cpu_buffers();
    asm!("sti", "hlt", options(nostack, preserves_flags));
}

pub unsafe fn native_halt() {
    x86_idle_clear_cpu_buffers();
    asm!("hlt", options(nostack, preserves_flags));
}

pub unsafe fn native_irqs_disabled_flags(flags: usize) -> i32 {
    if (flags & X86_EFLAGS_IF) == 0 { 1 } else { 0 }
}

pub unsafe fn native_local_irq_save() -> usize {
    let flags = native_save_fl();
    native_irq_disable();
    flags
}

pub unsafe fn native_local_irq_restore(flags: usize) {
    if native_irqs_disabled_flags(flags) == 0 {
        native_irq_enable();
    }
}

/* CONFIG_PARAVIRT is not resolved in this file. */

/* Used in the idle loop; sti takes one instruction cycle to complete. */
pub unsafe fn arch_safe_halt() {
    native_safe_halt();
}

/* Used when interrupts are already enabled or to shutdown the processor. */
pub unsafe fn halt() {
    native_halt();
}

/* CONFIG_PARAVIRT_XXL is not resolved in this file. */

pub unsafe fn arch_local_save_flags() -> usize {
    native_save_fl()
}

pub unsafe fn arch_local_irq_disable() {
    native_irq_disable();
}

pub unsafe fn arch_local_irq_enable() {
    native_irq_enable();
}

/* For spinlocks, etc. */
pub unsafe fn arch_local_irq_save() -> usize {
    let flags = arch_local_save_flags();
    arch_local_irq_disable();
    flags
}

pub unsafe fn arch_irqs_disabled_flags(flags: usize) -> i32 {
    if (flags & X86_EFLAGS_IF) == 0 { 1 } else { 0 }
}

pub unsafe fn arch_irqs_disabled() -> i32 {
    let flags = arch_local_save_flags();
    arch_irqs_disabled_flags(flags)
}

pub unsafe fn arch_local_irq_restore(flags: usize) {
    if arch_irqs_disabled_flags(flags) == 0 {
        arch_local_irq_enable();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
