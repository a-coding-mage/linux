/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	include/asm-mips/dec/ioasic.h
 *
 *	DEC I/O ASIC access operations.
 *
 *	Copyright (C) 2000, 2002, 2003  Maciej W. Rozycki
 */

// Dependency equivalents supplied by the surrounding translation unit:
// linux/spinlock.h and linux/types.h

extern "C" {
    pub static mut ioasic_ssr_lock: spinlock_t;

    pub static mut ioasic_base: *mut u32;

    pub fn init_ioasic_irqs(base: ::core::ffi::c_int);

    pub fn dec_ioasic_clocksource_init() -> ::core::ffi::c_int;
}

pub unsafe fn ioasic_write(reg: ::core::ffi::c_uint, v: u32) {
    ::core::ptr::write_volatile(ioasic_base.add((reg / 4) as usize), v);
}

pub unsafe fn ioasic_read(reg: ::core::ffi::c_uint) -> u32 {
    ::core::ptr::read_volatile(ioasic_base.add((reg / 4) as usize))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
