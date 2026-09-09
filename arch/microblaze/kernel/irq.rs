/*
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Declarations supplied by the Linux kernel and other translated files.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn set_irq_regs(regs: *mut pt_regs) -> *mut pt_regs;
    fn trace_hardirqs_off();
    fn trace_hardirqs_on();
    fn irq_enter();
    fn irq_exit();
    fn irqchip_init();

    static mut handle_arch_irq: unsafe extern "C" fn(*mut pt_regs);
}

pub unsafe extern "C" fn do_IRQ(regs: *mut pt_regs) {
    let old_regs: *mut pt_regs = unsafe { set_irq_regs(regs) };
    unsafe { trace_hardirqs_off() };

    unsafe { irq_enter() };
    unsafe { handle_arch_irq(regs) };
    unsafe { irq_exit() };
    unsafe { set_irq_regs(old_regs) };
    unsafe { trace_hardirqs_on() };
}

pub unsafe extern "C" fn init_IRQ() {
    /* process the entire interrupt tree in one go */
    unsafe { irqchip_init() };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
