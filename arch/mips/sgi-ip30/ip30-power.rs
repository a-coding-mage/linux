// SPDX-License-Identifier: GPL-2.0
/*
 * ip30-power.c: Software powerdown and reset handling for IP30 architecture.
 *
 * Copyright (C) 2004-2007 Stanislaw Skowronek <skylark@unaligned.org>
 *               2014 Joshua Kinard <linux@kumba.dev>
 *               2009 Johannes Dickgreber <tanzy@gmx.de>
 */

// Linux kernel and architecture dependencies supplied by other translation units.

use core::ffi::c_char;

#[repr(C)]
pub struct HeartRegs {
    pub mode: u64,
}

extern "C" {
    pub static mut heart_regs: *mut HeartRegs;
    pub static mut _machine_restart: Option<unsafe extern "C" fn(*mut c_char) -> !>;

    pub fn heart_read(reg: *const u64) -> u64;
    pub fn heart_write(value: u64, reg: *mut u64);
}

// HM_COLD_RST is supplied by <asm/sgi/heart.h>.

unsafe extern "C" fn ip30_machine_restart(_cmd: *mut c_char) -> ! {
    /*
     * Execute HEART cold reset
     *   Yes, it's cold-HEARTed!
     */
    unsafe {
        heart_write(
            heart_read(&(*heart_regs).mode) | HM_COLD_RST,
            &mut (*heart_regs).mode,
        );
    }
    core::hint::unreachable_unchecked()
}

#[allow(non_snake_case)]
unsafe extern "C" fn ip30_reboot_setup() -> i32 {
    unsafe {
        _machine_restart = Some(ip30_machine_restart);
    }

    0
}

// Equivalent of subsys_initcall(ip30_reboot_setup); registered by the kernel init system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
