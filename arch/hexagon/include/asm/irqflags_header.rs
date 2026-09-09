/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * IRQ support for the Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the original C includes:
// <asm/hexagon_vm.h>
// <linux/types.h>

use core::ffi::c_ulong;

extern "C" {
    fn __vmgetie() -> c_ulong;
    fn __vmsetie(value: c_ulong) -> c_ulong;
}

// These constants are supplied by asm/hexagon_vm.h.
// const VM_INT_ENABLE: c_ulong = ...;
// const VM_INT_DISABLE: c_ulong = ...;

pub unsafe fn arch_local_save_flags() -> c_ulong {
    __vmgetie()
}

pub unsafe fn arch_local_irq_save() -> c_ulong {
    __vmsetie(VM_INT_DISABLE)
}

pub unsafe fn arch_irqs_disabled_flags(flags: c_ulong) -> bool {
    !flags
}

pub unsafe fn arch_irqs_disabled() -> bool {
    !__vmgetie()
}

pub unsafe fn arch_local_irq_enable() {
    __vmsetie(VM_INT_ENABLE);
}

pub unsafe fn arch_local_irq_disable() {
    __vmsetie(VM_INT_DISABLE);
}

pub unsafe fn arch_local_irq_restore(flags: c_ulong) {
    __vmsetie(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
