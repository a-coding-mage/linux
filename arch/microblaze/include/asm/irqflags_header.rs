/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// Dependencies supplied by the surrounding translation unit:
// `MSR_IE` is defined by asm/registers.h. The C header also includes
// linux/types.h for the integer and bool types used here.

// CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR selects the corresponding C
// implementation. These cfg branches preserve that build-time condition.

#[cfg(CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR)]
#[inline(always)]
pub unsafe fn arch_local_irq_save() -> usize {
    let flags: usize;
    core::arch::asm!(
        "msrclr {flags}, {msr_ie}",
        "nop",
        flags = out(reg) flags,
        msr_ie = const MSR_IE,
        options(nostack)
    );
    flags
}

#[cfg(CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR)]
#[inline(always)]
pub unsafe fn arch_local_irq_disable() {
    // this uses r0 without declaring it - is that correct?
    core::arch::asm!(
        "msrclr r0, {msr_ie}",
        "nop",
        msr_ie = const MSR_IE,
        options(nostack)
    );
}

#[cfg(CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR)]
#[inline(always)]
pub unsafe fn arch_local_irq_enable() {
    // this uses r0 without declaring it - is that correct?
    core::arch::asm!(
        "msrset r0, {msr_ie}",
        "nop",
        msr_ie = const MSR_IE,
        options(nostack)
    );
}

#[cfg(not(CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR))]
#[inline(always)]
pub unsafe fn arch_local_irq_save() -> usize {
    let flags: usize;
    let tmp: usize;
    core::arch::asm!(
        "mfs {flags}, rmsr",
        "nop",
        "andi {tmp}, {flags}, {not_msr_ie}",
        "mts rmsr, {tmp}",
        "nop",
        flags = out(reg) flags,
        tmp = out(reg) tmp,
        not_msr_ie = const !MSR_IE,
        options(nostack)
    );
    flags
}

#[cfg(not(CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR))]
#[inline(always)]
pub unsafe fn arch_local_irq_disable() {
    let tmp: usize;
    core::arch::asm!(
        "mfs {tmp}, rmsr",
        "nop",
        "andi {tmp}, {tmp}, {not_msr_ie}",
        "mts rmsr, {tmp}",
        "nop",
        tmp = inout(reg) tmp,
        not_msr_ie = const !MSR_IE,
        options(nostack)
    );
}

#[cfg(not(CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR))]
#[inline(always)]
pub unsafe fn arch_local_irq_enable() {
    let tmp: usize;
    core::arch::asm!(
        "mfs {tmp}, rmsr",
        "nop",
        "ori {tmp}, {tmp}, {msr_ie}",
        "mts rmsr, {tmp}",
        "nop",
        tmp = inout(reg) tmp,
        msr_ie = const MSR_IE,
        options(nostack)
    );
}

#[inline(always)]
pub unsafe fn arch_local_save_flags() -> usize {
    let flags: usize;
    core::arch::asm!(
        "mfs {flags}, rmsr",
        "nop",
        flags = out(reg) flags,
        options(nostack)
    );
    flags
}

#[inline(always)]
pub unsafe fn arch_local_irq_restore(flags: usize) {
    core::arch::asm!(
        "mts rmsr, {flags}",
        "nop",
        flags = in(reg) flags,
        options(nostack)
    );
}

#[inline(always)]
pub unsafe fn arch_irqs_disabled_flags(flags: usize) -> bool {
    (flags & MSR_IE) == 0
}

#[inline(always)]
pub unsafe fn arch_irqs_disabled() -> bool {
    arch_irqs_disabled_flags(arch_local_save_flags())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
