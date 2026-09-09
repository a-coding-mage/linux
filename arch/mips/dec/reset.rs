// SPDX-License-Identifier: GPL-2.0
/*
 * Reset a DECstation machine.
 *
 * Copyright (C) 199x  the Anonymous
 * Copyright (C) 2001, 2002, 2003  Maciej W. Rozycki
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/interrupt.h, linux/linkage.h, asm/addrspace.h, and asm/dec/reset.h.

type NoretFuncT = unsafe extern "C" fn() -> !;

#[inline]
unsafe fn back_to_prom() -> ! {
	// CKSEG1ADDR(0x1fc00000) is the uncached PROM address used by the C source.
	let func: NoretFuncT = core::mem::transmute(0x1fc00000usize);

	func();
}

pub unsafe extern "C" fn dec_machine_restart(_command: *mut core::ffi::c_char) -> ! {
	back_to_prom();
}

pub unsafe extern "C" fn dec_machine_halt() -> ! {
	back_to_prom();
}

pub unsafe extern "C" fn dec_machine_power_off() -> ! {
	/* DECstations don't have a software power switch */
	back_to_prom();
}

pub unsafe extern "C" fn dec_intr_halt(
	_irq: core::ffi::c_int,
	_dev_id: *mut core::ffi::c_void,
) -> irqreturn_t {
	dec_machine_halt();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
