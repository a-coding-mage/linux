/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Rust translation of the MIPS IRQ flags header.
 */

// The following items depend on the corresponding kernel configuration and
// external MIPS hazard/compiler definitions.

#[cfg(CONFIG_CPU_HAS_DIEI)]
#[inline]
pub unsafe fn arch_local_irq_disable() {
    core::arch::asm!(
        ".set push",
        ".set noat",
        "di",
        "{irq_disable_hazard}",
        ".set pop",
        irq_disable_hazard = sym __irq_disable_hazard,
        options(nostack, preserves_flags)
    );
}

#[cfg(CONFIG_CPU_HAS_DIEI)]
#[inline]
pub unsafe fn arch_local_irq_save() -> usize {
    let flags: usize;
    core::arch::asm!(
        ".set push",
        ".set reorder",
        ".set noat",
        // CONFIG_CPU_LOONGSON64 || CONFIG_CPU_LOONGSON32 uses:
        //   mfc0 {flags}, $12; di
        "di {flags}",
        "andi {flags}, 1",
        "{irq_disable_hazard}",
        ".set pop",
        flags = lateout(reg) flags,
        irq_disable_hazard = sym __irq_disable_hazard,
        options(nostack)
    );
    flags
}

#[cfg(CONFIG_CPU_HAS_DIEI)]
#[inline]
pub unsafe fn arch_local_irq_restore(flags: usize) {
    let mut tmp1 = flags;
    core::arch::asm!(
        ".set push",
        ".set noreorder",
        ".set noat",
        // CONFIG_IRQ_MIPS_CPU uses the slow race-free sequence below.
        // Without it, the original uses mfc0 $1,$12; ins $1,{flags},0,1;
        // mtc0 $1,$12 instead.
        "beqz {flags}, 1f",
        "di",
        "ei",
        "1:",
        "{irq_disable_hazard}",
        ".set pop",
        flags = inout(reg) tmp1,
        irq_disable_hazard = sym __irq_disable_hazard,
        options(nostack)
    );
}

#[cfg(not(CONFIG_CPU_HAS_DIEI))]
extern "C" {
    pub fn arch_local_irq_disable();
    pub fn arch_local_irq_save() -> usize;
    pub fn arch_local_irq_restore(flags: usize);
}

#[inline]
pub unsafe fn arch_local_irq_enable() {
    core::arch::asm!(
        ".set push",
        ".set reorder",
        ".set noat",
        // Without CONFIG_CPU_HAS_DIEI, the original uses:
        // mfc0 $1,$12; ori $1,0x1f; xori $1,0x1e; mtc0 $1,$12.
        "ei",
        "{irq_enable_hazard}",
        ".set pop",
        irq_enable_hazard = sym __irq_enable_hazard,
        options(nostack, preserves_flags)
    );
}

#[inline]
pub unsafe fn arch_local_save_flags() -> usize {
    let flags: usize;
    core::arch::asm!(
        ".set push",
        ".set reorder",
        "mfc0 {flags}, $12",
        ".set pop",
        flags = lateout(reg) flags,
        options(nostack)
    );
    flags
}

#[inline]
pub fn arch_irqs_disabled_flags(flags: usize) -> i32 {
    if (flags & 1) == 0 { 1 } else { 0 }
}

#[inline]
pub unsafe fn arch_irqs_disabled() -> i32 {
    arch_irqs_disabled_flags(arch_local_save_flags())
}

// Assembly-only IRQ tracing macros from the original header. They are kept as
// configuration notes because Rust has no direct equivalent for preprocessor
// assembly macro definitions in this header translation.
// CONFIG_TRACE_IRQFLAGS:
//   TRACE_IRQS_ON        => CLI; jal trace_hardirqs_on
//   TRACE_IRQS_ON_RELOAD => TRACE_IRQS_ON; TRACE_IRQS_RELOAD_REGS
//   TRACE_IRQS_OFF       => jal trace_hardirqs_off
// Otherwise all three macros expand to nothing.

extern "C" {
    fn __irq_disable_hazard();
    fn __irq_enable_hazard();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
