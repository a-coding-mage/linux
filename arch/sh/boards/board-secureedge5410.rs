// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002  David McCullough <davidm@snapgear.com>
 * Copyright (C) 2003  Paul Mundt <lethal@linux-sh.org>
 *
 * Based on files with the following comments:
 *
 *           Copyright (C) 2000  Kazumoto Kojima
 *
 *           Modified for 7751 Solution Engine by
 *           Ian da Silva and Jeremy Siegel, 2001.
 */

// C dependencies: linux/init.h, linux/irq.h, linux/interrupt.h,
// linux/timer.h, linux/delay.h, linux/sched.h, asm/machvec.h,
// mach/secureedge5410.h, asm/irq.h, asm/io.h, and cpu/timer.h.

use core::ffi::c_void;

// External kernel types, constants, functions, and globals supplied by other files.
type IrqReturnT = i32;

const IRQ_HANDLED: IrqReturnT = 1;
const IRQ_MODE_IRQ: i32 = 0;

extern "C" {
    fn printk(fmt: *const u8, ...);
    fn evt2irq(event: u32) -> u32;
    fn request_irq(
        irq: u32,
        handler: unsafe extern "C" fn(i32, *mut c_void) -> IrqReturnT,
        flags: u32,
        name: *const u8,
        dev_id: *mut c_void,
    ) -> i32;
    fn plat_irq_setup_pins(mode: i32);
}

pub static mut secureedge5410_ioport: u16 = 0;

/*
 * EraseConfig handling functions
 */
unsafe extern "C" fn eraseconfig_interrupt(_irq: i32, _dev_id: *mut c_void) -> IrqReturnT {
    printk(b"SnapGear: erase switch interrupt!\n\0".as_ptr());

    IRQ_HANDLED
}

unsafe extern "C" fn eraseconfig_init() -> i32 {
    let irq: u32 = evt2irq(0x240);

    printk(b"SnapGear: EraseConfig init\n\0".as_ptr());

    /* Setup "EraseConfig" switch on external IRQ 0 */
    if request_irq(
        irq,
        eraseconfig_interrupt,
        0,
        b"Erase Config\0".as_ptr(),
        core::ptr::null_mut(),
    ) != 0
    {
        printk(b"SnapGear: failed to register IRQ%d for Reset witch\n\0".as_ptr(), irq);
    } else {
        printk(
            b"SnapGear: registered EraseConfig switch on IRQ%d\n\0".as_ptr(),
            irq,
        );
    }
    0
}

// device_initcall(eraseconfig_init);

/*
 * Initialize IRQ setting
 *
 * IRL0 = erase switch
 * IRL1 = eth0
 * IRL2 = eth1
 * IRL3 = crypto
 */
unsafe extern "C" fn init_snapgear_IRQ() {
    printk(b"Setup SnapGear IRQ/IPR ...\n\0".as_ptr());
    /* enable individual interrupt mode for externals */
    plat_irq_setup_pins(IRQ_MODE_IRQ);
}

/*
 * The Machine Vector
 */
#[repr(C)]
pub struct ShMachineVector {
    pub mv_name: *const u8,
    pub mv_init_irq: Option<unsafe extern "C" fn()>,
}

#[used]
pub static mut mv_snapgear: ShMachineVector = ShMachineVector {
    mv_name: b"SnapGear SecureEdge5410\0".as_ptr(),
    mv_init_irq: Some(init_snapgear_IRQ),
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
