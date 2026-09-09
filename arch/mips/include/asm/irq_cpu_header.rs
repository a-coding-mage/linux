/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	include/asm-mips/irq_cpu.h
 *
 *	MIPS CPU interrupt definitions.
 *
 *	Copyright (C) 2002  Maciej W. Rozycki
 */

unsafe extern "C" {
    pub fn mips_cpu_irq_init();
}

/* Preserves the C build-time CONFIG_IRQ_DOMAIN condition. */
#[cfg(feature = "CONFIG_IRQ_DOMAIN")]
pub enum device_node {}

#[cfg(feature = "CONFIG_IRQ_DOMAIN")]
unsafe extern "C" {
    pub fn mips_cpu_irq_of_init(
        of_node: *mut device_node,
        parent: *mut device_node,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
