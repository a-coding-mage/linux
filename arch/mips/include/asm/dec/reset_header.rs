/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	arch/mips/include/asm/dec/reset.h
 *
 *	DECstation/DECsystem halt/reset support.
 *
 *	Copyright (C) 2026  Maciej W. Rozycki
 */

// Dependency intent from <linux/compiler_attributes.h> is represented by Rust's
// diverging return type on the non-returning declarations below.

extern "C" {
    pub fn dec_machine_restart(command: *mut core::ffi::c_char) -> !;
    pub fn dec_machine_halt() -> !;
    pub fn dec_machine_power_off() -> !;
    pub fn dec_intr_halt(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
}

// `irqreturn_t` is supplied by the surrounding kernel interface.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
