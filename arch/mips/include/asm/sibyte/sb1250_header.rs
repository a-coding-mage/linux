/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2000, 2001, 2002, 2003 Broadcom Corporation
 */

/*
 * yymmddpp: year, month, day, patch.
 * should sync with Makefile EXTRAVERSION
 */
pub const SIBYTE_RELEASE: u32 = 0x02111403;

pub const SB1250_NR_IRQS: u32 = 64;

pub const BCM1480_NR_IRQS: u32 = 128;
pub const BCM1480_NR_IRQS_HALF: u32 = 64;

pub const SB1250_DUART_MINOR_BASE: u32 = 64;

/* The following declarations are present only when not assembling. */

/* For revision/pass information */
extern "C" {
    pub static mut sb1_pass: core::ffi::c_uint;
    pub static mut soc_pass: core::ffi::c_uint;
    pub static mut soc_type: core::ffi::c_uint;
    pub static mut periph_rev: core::ffi::c_uint;
    pub static mut zbbus_mhz: core::ffi::c_uint;

    pub fn sb1250_mask_irq(cpu: core::ffi::c_int, irq: core::ffi::c_int);
    pub fn sb1250_unmask_irq(cpu: core::ffi::c_int, irq: core::ffi::c_int);

    pub fn bcm1480_time_init();
    pub fn bcm1480_mask_irq(cpu: core::ffi::c_int, irq: core::ffi::c_int);
    pub fn bcm1480_unmask_irq(cpu: core::ffi::c_int, irq: core::ffi::c_int);
}

#[macro_export]
macro_rules! AT_spin {
    () => {
        unsafe {
            core::arch::asm!(
                ".set noat",
                "li $at, 0",
                "1: beqz $at, 1b",
                ".set at",
            );
        }
    };
}

#[macro_export]
macro_rules! IOADDR {
    ($a:expr) => {
        (unsafe { (IO_BASE + ($a)) as *mut core::ffi::c_void })
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
