/*
 * IRQ vector handles
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 1996, 1997, 2003 by Ralf Baechle
 */

// External declarations supplied by the Linux/MIPS headers and other units.
use core::ffi::{c_char, c_void};

extern "C" {
    fn read_c0_status() -> u32;
    fn read_c0_cause() -> u32;
    fn gt641xx_irq_dispatch();
    fn i8259_irq() -> i32;
    fn spurious_interrupt();
    fn do_IRQ(irq: i32);
    fn mips_cpu_irq_init();
    fn gt641xx_irq_init();
    fn init_i8259_irqs();
    fn request_irq(
        irq: i32,
        handler: unsafe extern "C" fn(i32, *mut c_void) -> (),
        flags: u32,
        name: *const c_char,
        dev: *mut c_void,
    ) -> i32;
    fn no_action(irq: i32, dev_id: *mut c_void);
    fn pr_err(fmt: *const c_char, ...);
}

// Constants supplied by the Linux/MIPS headers.
extern "C" {
    static GT641XX_CASCADE_IRQ: i32;
    static I8259_CASCADE_IRQ: i32;
    static MIPS_CPU_IRQ_BASE: i32;
    static ST0_IM: u32;
    static CAUSEF_IP2: u32;
    static CAUSEF_IP3: u32;
    static CAUSEF_IP4: u32;
    static CAUSEF_IP5: u32;
    static CAUSEF_IP6: u32;
    static CAUSEF_IP7: u32;
    static IRQF_NO_THREAD: u32;
}

pub unsafe extern "C" fn plat_irq_dispatch() {
    let pending: u32 = read_c0_status() & read_c0_cause() & ST0_IM;
    let mut irq: i32;

    if pending & CAUSEF_IP2 != 0 {
        gt641xx_irq_dispatch();
    } else if pending & CAUSEF_IP6 != 0 {
        irq = i8259_irq();
        if irq < 0 {
            spurious_interrupt();
        } else {
            do_IRQ(irq);
        }
    } else if pending & CAUSEF_IP3 != 0 {
        do_IRQ(MIPS_CPU_IRQ_BASE + 3);
    } else if pending & CAUSEF_IP4 != 0 {
        do_IRQ(MIPS_CPU_IRQ_BASE + 4);
    } else if pending & CAUSEF_IP5 != 0 {
        do_IRQ(MIPS_CPU_IRQ_BASE + 5);
    } else if pending & CAUSEF_IP7 != 0 {
        do_IRQ(MIPS_CPU_IRQ_BASE + 7);
    } else {
        spurious_interrupt();
    }
}

pub unsafe extern "C" fn arch_init_irq() {
    mips_cpu_irq_init();
    gt641xx_irq_init();
    init_i8259_irqs();

    if request_irq(
        GT641XX_CASCADE_IRQ,
        no_action,
        IRQF_NO_THREAD,
        b"cascade\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
    ) != 0
    {
        pr_err(
            b"Failed to request irq %d (cascade)\n\0".as_ptr() as *const c_char,
            GT641XX_CASCADE_IRQ,
        );
    }
    if request_irq(
        I8259_CASCADE_IRQ,
        no_action,
        IRQF_NO_THREAD,
        b"cascade\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
    ) != 0
    {
        pr_err(
            b"Failed to request irq %d (cascade)\n\0".as_ptr() as *const c_char,
            I8259_CASCADE_IRQ,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
