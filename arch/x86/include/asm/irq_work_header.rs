/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by asm/cpufeature.h in the C source.
#[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
unsafe extern "C" {
    static X86_FEATURE_APIC: u32;
    fn boot_cpu_has(feature: u32) -> bool;
}

#[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
#[inline]
pub unsafe fn arch_irq_work_has_interrupt() -> bool {
    boot_cpu_has(X86_FEATURE_APIC)
}

#[cfg(not(feature = "CONFIG_X86_LOCAL_APIC"))]
#[inline]
pub fn arch_irq_work_has_interrupt() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
