// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Common PowerQUICC II code.
 *
 * Author: Scott Wood <scottwood@freescale.com>
 * Copyright (c) 2007 Freescale Semiconductor
 *
 * Based on code by Vitaly Bordug <vbordug@ru.mvista.com>
 * pq2_restart fix by Wade Farnsworth <wfarnsworth@mvista.com>
 * Copyright (c) 2006 MontaVista Software, Inc.
 */

use core::ffi::c_char;

// Declarations supplied by the Linux PowerPC and platform dependencies.
#[repr(C)]
pub struct Cpm2Clkrst {
    pub car_rmr: u32,
    pub res: [u8; 1],
}

#[repr(C)]
pub struct Cpm2Imm { 
    pub im_clkrst: Cpm2Clkrst,
}

extern "C" {
    pub static mut cpm2_immr: *mut Cpm2Imm;
    fn local_irq_disable();
    fn setbits32(addr: *mut u32, bits: u32);
    fn mtmsr(value: usize);
    fn mfmsr() -> usize;
    fn in_8(addr: *const u8) -> u8;
    fn panic(message: *const u8) -> !;
}

// MSR_ME, MSR_EE, MSR_IR, and MSR_DR are supplied by asm/processor.h.

const RMR_CSRE: u32 = 0x00000001;

pub unsafe fn pq2_restart(_cmd: *mut c_char) -> ! {
    local_irq_disable();
    setbits32(
        &mut (*cpm2_immr).im_clkrst.car_rmr,
        RMR_CSRE,
    );

    /* Clear the ME,EE,IR & DR bits in MSR to cause checkstop */
    mtmsr(mfmsr() & !(MSR_ME | MSR_EE | MSR_IR | MSR_DR));
    in_8((*cpm2_immr).cast::<u8>().add(
        core::mem::offset_of!(Cpm2Imm, im_clkrst)
            + core::mem::offset_of!(Cpm2Clkrst, res),
    ));

    panic(b"Restart failed\n\0".as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
