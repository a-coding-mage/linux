/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994, 95, 96, 97, 98, 99, 2003 by Ralf Baechle
 * Copyright (C) 1996 by Paul M. Antoine
 * Copyright (C) 1999 Silicon Graphics
 * Copyright (C) 2000 MIPS Technologies, Inc.
 */

// Dependencies supplied by the surrounding kernel build:
// asm/irqflags.h, asm/hazards.h, linux/compiler.h, linux/preempt.h,
// linux/export.h, and linux/stringify.h.

#[cfg(not(CONFIG_CPU_HAS_DIEI))]
extern "C" {
    fn preempt_disable_notrace();
    fn preempt_enable_notrace();
}

#[cfg(not(CONFIG_CPU_HAS_DIEI))]
#[no_mangle]
pub unsafe extern "C" fn arch_local_irq_disable() {
    preempt_disable_notrace();

    // For cli() the hazard sequence ensures the new status value has arrived.
    // The exact hazard instruction is supplied by asm/hazards.h.
    core::arch::asm!(
        ".set push",
        ".set noat",
        "mfc0 $1, $12",
        "ori $1, 0x1f",
        "xori $1, 0x1f",
        ".set noreorder",
        "mtc0 $1, $12",
        "__irq_disable_hazard",
        ".set pop",
        options(nostack)
    );

    preempt_enable_notrace();
}

// EXPORT_SYMBOL(arch_local_irq_disable);

#[cfg(not(CONFIG_CPU_HAS_DIEI))]
#[no_mangle]
pub unsafe extern "C" fn arch_local_irq_save() -> usize {
    let mut flags: usize;

    preempt_disable_notrace();

    core::arch::asm!(
        ".set push",
        ".set reorder",
        ".set noat",
        "mfc0 {flags}, $12",
        "ori $1, {flags}, 0x1f",
        "xori $1, 0x1f",
        ".set noreorder",
        "mtc0 $1, $12",
        "__irq_disable_hazard",
        ".set pop",
        flags = out(reg) flags,
        options(nostack)
    );

    preempt_enable_notrace();

    flags
}

// EXPORT_SYMBOL(arch_local_irq_save);

#[cfg(not(CONFIG_CPU_HAS_DIEI))]
#[no_mangle]
pub unsafe extern "C" fn arch_local_irq_restore(flags: usize) {
    let mut tmp1: usize;

    preempt_disable_notrace();

    core::arch::asm!(
        ".set push",
        ".set noreorder",
        ".set noat",
        "mfc0 $1, $12",
        "andi {flags}, 1",
        "ori $1, 0x1f",
        "xori $1, 0x1f",
        "or {flags}, $1",
        "mtc0 {flags}, $12",
        "__irq_disable_hazard",
        ".set pop",
        flags = inout(reg) flags => tmp1,
        options(nostack)
    );

    preempt_enable_notrace();
}

// EXPORT_SYMBOL(arch_local_irq_restore);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
