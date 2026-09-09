// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/renesas/sdk7780/irq.c
 *
 * Renesas Technology Europe SDK7780 Support.
 *
 * Copyright (C) 2008  Nicholas Beck <nbeck@mpc-data.co.uk>
 */

// Dependencies supplied by the Linux SH platform and SDK7780 headers.

use core::ffi::c_char;

#[repr(C)]
pub struct IntcVect {
    pub irq: i32,
    pub vect: i32,
}

#[repr(C)]
pub struct IntcMaskReg {
    pub index: i32,
    pub addr: usize,
    pub width: i32,
    pub p: [i32; 16],
}

#[repr(C)]
pub struct IntcDesc {
    pub name: *const c_char,
    pub vectors: *mut IntcVect,
    pub priorities: *mut core::ffi::c_void,
    pub mask_registers: *mut IntcMaskReg,
    pub sense_registers: *mut core::ffi::c_void,
    pub ack_registers: *mut core::ffi::c_void,
}

extern "C" {
    pub static FPGA_IRQ0MR: usize;
    pub static FPGA_IMSR: usize;
    pub static IRQ_ETHERNET: i32;
    pub static IRQ_MODE_IRL3210: i32;

    pub fn __raw_writew(value: u16, address: usize);
    pub fn plat_irq_setup_pins(mode: i32);
    pub fn register_intc_controller(desc: *mut IntcDesc);
    pub fn printk(format: *const c_char, ...);
}

const KERN_INFO: &[u8] = b"<6>\0";

enum {
    UNUSED = 0,
    /* board specific interrupt sources */
    SMC91C111,
}

static mut FPGA_VECTORS: [IntcVect; 1] = [IntcVect {
    irq: SMC91C111,
    vect: unsafe { IRQ_ETHERNET },
}];

static mut FPGA_MASK_REGISTERS: [IntcMaskReg; 1] = [IntcMaskReg {
    index: 0,
    addr: unsafe { FPGA_IRQ0MR },
    width: 16,
    p: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, SMC91C111, 0, 0, 0, 0],
}];

static mut FPGA_INTC_DESC: IntcDesc = IntcDesc {
    name: b"sdk7780-irq\0" as *const u8 as *const c_char,
    vectors: unsafe { FPGA_VECTORS.as_mut_ptr() },
    priorities: core::ptr::null_mut(),
    mask_registers: unsafe { FPGA_MASK_REGISTERS.as_mut_ptr() },
    sense_registers: core::ptr::null_mut(),
    ack_registers: core::ptr::null_mut(),
};

pub unsafe extern "C" fn init_sdk7780_IRQ() {
    static MESSAGE: &[u8] = b"<6>Using SDK7780 interrupt controller.\n\0";
    printk(MESSAGE.as_ptr() as *const c_char);

    __raw_writew(0xFFFF, FPGA_IRQ0MR);
    /* Setup IRL 0-3 */
    __raw_writew(0x0003, FPGA_IMSR);
    plat_irq_setup_pins(IRQ_MODE_IRL3210);

    register_intc_controller(&raw mut FPGA_INTC_DESC);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
