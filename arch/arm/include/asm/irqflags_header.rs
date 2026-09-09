/* SPDX-License-Identifier: GPL-2.0 */

// CPU interrupt mask handling.
// The C header's __KERNEL__ guard and build-time architecture conditions are
// preserved here as conditional Rust items.

#[cfg(feature = "CONFIG_CPU_V7M")]
pub const IRQMASK_REG_NAME_R: &str = "primask";
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const IRQMASK_REG_NAME_W: &str = "primask";
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const IRQMASK_I_BIT: usize = 1;

#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const IRQMASK_REG_NAME_R: &str = "cpsr";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const IRQMASK_REG_NAME_W: &str = "cpsr_c";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const IRQMASK_I_BIT: usize = PSR_I_BIT;

#[cfg(feature = "LINUX_ARM_ARCH_GE_6")]
#[inline]
pub unsafe fn arch_local_irq_save() -> usize {
    let flags: usize;
    core::arch::asm!(
        "mrs {0}, {1}",
        "cpsid i",
        out(reg) flags,
        const IRQMASK_REG_NAME_R,
        options(nostack)
    );
    flags
}

#[cfg(feature = "LINUX_ARM_ARCH_GE_6")]
#[inline]
pub unsafe fn arch_local_irq_enable() {
    core::arch::asm!("cpsie i", options(nostack));
}

#[cfg(feature = "LINUX_ARM_ARCH_GE_6")]
#[inline]
pub unsafe fn arch_local_irq_disable() {
    core::arch::asm!("cpsid i", options(nostack));
}

#[cfg(feature = "LINUX_ARM_ARCH_GE_6")]
#[inline]
pub unsafe fn local_fiq_enable() {
    core::arch::asm!("cpsie f", options(nostack));
}

#[cfg(feature = "LINUX_ARM_ARCH_GE_6")]
#[inline]
pub unsafe fn local_fiq_disable() {
    core::arch::asm!("cpsid f", options(nostack));
}

#[cfg(all(feature = "LINUX_ARM_ARCH_GE_6", not(feature = "CONFIG_CPU_V7M")))]
#[inline]
pub unsafe fn local_abt_enable() {
    core::arch::asm!("cpsie a", options(nostack));
}

#[cfg(all(feature = "LINUX_ARM_ARCH_GE_6", not(feature = "CONFIG_CPU_V7M")))]
#[inline]
pub unsafe fn local_abt_disable() {
    core::arch::asm!("cpsid a", options(nostack));
}

#[cfg(all(feature = "LINUX_ARM_ARCH_GE_6", feature = "CONFIG_CPU_V7M"))]
#[inline]
pub fn local_abt_enable() {}

#[cfg(all(feature = "LINUX_ARM_ARCH_GE_6", feature = "CONFIG_CPU_V7M"))]
#[inline]
pub fn local_abt_disable() {}

#[cfg(not(feature = "LINUX_ARM_ARCH_GE_6"))]
#[inline]
pub unsafe fn arch_local_irq_save() -> usize {
    let flags: usize;
    let temp: usize;
    core::arch::asm!(
        "mrs {flags}, cpsr",
        "orr {temp}, {flags}, #128",
        "msr cpsr_c, {temp}",
        flags = out(reg) flags,
        temp = out(reg) temp,
        options(nostack)
    );
    flags
}

#[cfg(not(feature = "LINUX_ARM_ARCH_GE_6"))]
#[inline]
pub unsafe fn arch_local_irq_enable() {
    let temp: usize;
    core::arch::asm!(
        "mrs {temp}, cpsr",
        "bic {temp}, {temp}, #128",
        "msr cpsr_c, {temp}",
        temp = out(reg) temp,
        options(nostack)
    );
}

#[cfg(not(feature = "LINUX_ARM_ARCH_GE_6"))]
#[inline]
pub unsafe fn arch_local_irq_disable() {
    let temp: usize;
    core::arch::asm!(
        "mrs {temp}, cpsr",
        "orr {temp}, {temp}, #128",
        "msr cpsr_c, {temp}",
        temp = out(reg) temp,
        options(nostack)
    );
}

#[cfg(not(feature = "LINUX_ARM_ARCH_GE_6"))]
#[inline]
pub unsafe fn local_fiq_enable() {
    let temp: usize;
    core::arch::asm!(
        "mrs {temp}, cpsr",
        "bic {temp}, {temp}, #64",
        "msr cpsr_c, {temp}",
        temp = out(reg) temp,
        options(nostack)
    );
}

#[cfg(not(feature = "LINUX_ARM_ARCH_GE_6"))]
#[inline]
pub unsafe fn local_fiq_disable() {
    let temp: usize;
    core::arch::asm!(
        "mrs {temp}, cpsr",
        "orr {temp}, {temp}, #64",
        "msr cpsr_c, {temp}",
        temp = out(reg) temp,
        options(nostack)
    );
}

#[cfg(not(feature = "LINUX_ARM_ARCH_GE_6"))]
#[inline]
pub fn local_abt_enable() {}

#[cfg(not(feature = "LINUX_ARM_ARCH_GE_6"))]
#[inline]
pub fn local_abt_disable() {}

#[inline]
pub unsafe fn arch_local_save_flags() -> usize {
    let flags: usize;
    core::arch::asm!("mrs {flags}, {reg}", flags = out(reg) flags, reg = const IRQMASK_REG_NAME_R, options(nostack));
    flags
}

#[inline]
pub unsafe fn arch_local_irq_restore(flags: usize) {
    core::arch::asm!("msr {reg}, {flags}", reg = const IRQMASK_REG_NAME_W, flags = in(reg) flags, options(nostack));
}

#[inline]
pub fn arch_irqs_disabled_flags(flags: usize) -> usize {
    flags & IRQMASK_I_BIT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
