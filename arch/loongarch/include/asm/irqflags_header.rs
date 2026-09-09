/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// C header dependencies: linux/compiler.h, linux/stringify.h, asm/loongarch.h

#[inline]
pub unsafe fn arch_local_irq_enable() {
    let mut flags: u32 = CSR_CRMD_IE;
    let mask: u32 = CSR_CRMD_IE;

    core::arch::asm!(
        "csrxchg {flags}, {mask}, {reg}",
        flags = inout(reg) flags,
        mask = in(reg) mask,
        reg = const LOONGARCH_CSR_CRMD,
        options(nostack)
    );
}

#[inline]
pub unsafe fn arch_local_irq_disable() {
    let mut flags: u32 = 0;
    let mask: u32 = CSR_CRMD_IE;

    core::arch::asm!(
        "csrxchg {flags}, {mask}, {reg}",
        flags = inout(reg) flags,
        mask = in(reg) mask,
        reg = const LOONGARCH_CSR_CRMD,
        options(nostack)
    );
}

#[inline]
pub unsafe fn arch_local_irq_save() -> u32 {
    let mut flags: u32 = 0;
    let mask: u32 = CSR_CRMD_IE;

    core::arch::asm!(
        "csrxchg {flags}, {mask}, {reg}",
        flags = inout(reg) flags,
        mask = in(reg) mask,
        reg = const LOONGARCH_CSR_CRMD,
        options(nostack)
    );
    flags
}

#[inline]
pub unsafe fn arch_local_irq_restore(mut flags: usize) {
    let mask: u32 = CSR_CRMD_IE;

    core::arch::asm!(
        "csrxchg {flags}, {mask}, {reg}",
        flags = inout(reg) flags,
        mask = in(reg) mask,
        reg = const LOONGARCH_CSR_CRMD,
        options(nostack)
    );
}

#[inline]
pub unsafe fn arch_local_save_flags() -> u32 {
    let mut flags: u32;
    core::arch::asm!(
        "csrrd {flags}, {reg}",
        flags = out(reg) flags,
        reg = const LOONGARCH_CSR_CRMD,
        options(nostack)
    );
    flags
}

#[inline]
pub fn arch_irqs_disabled_flags(flags: usize) -> i32 {
    (!(flags & CSR_CRMD_IE) != 0) as i32
}

#[inline]
pub unsafe fn arch_irqs_disabled() -> i32 {
    arch_irqs_disabled_flags(arch_local_save_flags() as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
