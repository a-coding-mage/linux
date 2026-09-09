// SPDX-License-Identifier: GPL-2.0
/*
 * ip22-hpc.c: Routines for generic manipulation of the HPC controllers.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1998 Ralf Baechle
 */

// Dependencies supplied by the surrounding kernel translation.

pub static mut hpc3c0: *mut hpc3_regs = core::ptr::null_mut();
pub static mut hpc3c1: *mut hpc3_regs = core::ptr::null_mut();

pub static mut sgioc: *mut sgioc_regs = core::ptr::null_mut();

/* We need software copies of these because they are write only. */
pub static mut sgi_ioc_reset: u8 = 0;
pub static mut sgi_ioc_write: u8 = 0;

extern "C" {
    static mut system_type: *mut core::ffi::c_char;
    static mut sgint: *mut sgint_regs;

    fn ioremap(offset: usize, size: usize) -> *mut core::ffi::c_void;
    fn ip22_is_fullhouse() -> bool;
}

pub unsafe fn sgihpc_init() {
    /* ioremap can't fail */
    hpc3c0 = ioremap(HPC3_CHIP0_BASE, core::mem::size_of::<hpc3_regs>())
        as *mut hpc3_regs;
    hpc3c1 = ioremap(HPC3_CHIP1_BASE, core::mem::size_of::<hpc3_regs>())
        as *mut hpc3_regs;
    /* IOC lives in PBUS PIO channel 6 */
    sgioc = (*hpc3c0).pbus_extregs[6] as *mut sgioc_regs;

    (*hpc3c0).pbus_piocfg[6][0] |= HPC3_PIOCFG_DS16;
    if ip22_is_fullhouse() {
        /* Full House comes with INT2 which lives in PBUS PIO
         * channel 4 */
        sgint = (*hpc3c0).pbus_extregs[4] as *mut sgint_regs;
        system_type = b"SGI Indigo2\0".as_ptr() as *mut core::ffi::c_char;
    } else {
        /* Guiness comes with INT3 which is part of IOC */
        sgint = core::ptr::addr_of_mut!((*sgioc).int3);
        system_type = b"SGI Indy\0".as_ptr() as *mut core::ffi::c_char;
    }

    sgi_ioc_reset = SGIOC_RESET_PPORT
        | SGIOC_RESET_KBDMOUSE
        | SGIOC_RESET_EISA
        | SGIOC_RESET_ISDN
        | SGIOC_RESET_LC0OFF;

    sgi_ioc_write = SGIOC_WRITE_EASEL
        | SGIOC_WRITE_NTHRESH
        | SGIOC_WRITE_TPSPEED
        | SGIOC_WRITE_EPSEL
        | SGIOC_WRITE_U0AMODE
        | SGIOC_WRITE_U1AMODE;

    (*sgioc).reset = sgi_ioc_reset;
    (*sgioc).write = sgi_ioc_write;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
